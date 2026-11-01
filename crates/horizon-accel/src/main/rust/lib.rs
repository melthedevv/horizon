//! Horizon Fabric Accelerator
//!
//! A `cdylib` loaded via JNI by the `horizon-accel` Fabric mod.
//! Provides Rust implementations of vanilla subsystems that can replace
//! their Java counterparts when no mod conflicts exist.
//!
//! Architecture:
//! - Each accelerator module declares what it supersedes in `accel-manifest.json`
//! - At launch, the conflict scanner reads all installed mods' mixin targets
//!   and access wideners, and auto-disables any module that conflicts
//! - Modules are differential-tested against vanilla; non-identical ones ship disabled
//! - Fail-safe: Rust panics/JNI errors are caught at the boundary, logged,
//!   and the module falls back to vanilla for the session

#![allow(unsafe_code)] // JNI exports require unsafe; isolated with SAFETY comments
#![warn(missing_docs)]

mod error;
mod handshake;
mod manifest;
mod module;
mod scanner;
mod fallback;
mod chunk_io;
mod worldgen;

pub use error::{AccelError, AccelResult, HandshakeError};
pub use handshake::{handshake, AccelHandshake, ACCEL_PROTOCOL_VERSION};
pub use manifest::{AccelManifest, ModuleManifest, SupersedesEntry, SupersedesMethodEntry, SupersedesFieldEntry, FallbackConfig, MANIFEST_RESOURCE_PATH, load_manifest};
pub use module::{ModuleEntry, ModuleRegistry, ModuleState};
pub use scanner::{ConflictScanner, ConflictReport, ModMixinData, Conflict, load_mod_mixin_data};
pub use fallback::{FallbackHandler, FallbackReason};
pub use chunk_io::{ChunkIoModule, RegionFileHandle, NbtTag, PalettedContainer, init_chunk_io_module};
pub use worldgen::{WorldgenModule, NoiseSampler, SurfaceRule, BlockState, init_worldgen_module};

use std::sync::Arc;
use parking_lot::RwLock;
use once_cell::sync::Lazy;

// Re-export the jni crate for submodules
pub extern crate jni;
use jni::JNIEnv;
use jni::objects::{JClass, JString, JObject, JByteArray};
use jni::sys::{jint, jboolean, jlong, JNI_VERSION_1_8};
use tracing::{info, warn, error};

// Force jni crate to be compiled and available to all modules
#[allow(unused)]
use jni::objects as _jni_objects;
#[allow(unused)]
use jni::sys as _jni_sys;

/// Global module registry, initialized once per JVM session.
static REGISTRY: Lazy<RwLock<Option<Arc<ModuleRegistry>>>> = Lazy::new(|| RwLock::new(None));

/// Global fallback handler.
static FALLBACK: Lazy<FallbackHandler> = Lazy::new(FallbackHandler::new);

/// JNI export: called by Java on library load to verify compatibility.
///
/// Returns the protocol version on success, or a negative error code.
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_horizonlab_accel_AccelNative_initialize(
    env: JNIEnv<'_>,
    _class: JClass,
    java_protocol_version: jint,
) -> jint {
    let result = handshake(java_protocol_version as u32);
    match result {
        Ok(version) => {
            info!("Horizon Accelerator handshake OK: protocol v{}", version);
            version as jint
        }
        Err(e) => {
            error!("Horizon Accelerator handshake failed: {}", e);
            e.to_error_code()
        }
    }
}

/// JNI export: register all modules from the manifest resource.
///
/// Called once during mod initialization. Returns number of modules registered.
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_horizonlab_accel_AccelNative_registerModules(
    mut env: JNIEnv<'_>,
    _class: JClass,
    manifest_json: JString,
    disabled_modules_json: JString,
) -> jint {
    let manifest_str: String = match env.get_string(&manifest_json) {
        Ok(s) => s.into(),
        Err(e) => {
            error!("Failed to read manifest JSON: {}", e);
            return -1;
        }
    };

    let disabled_modules_str: String = match env.get_string(&disabled_modules_json) {
        Ok(s) => s.into(),
        Err(e) => {
            error!("Failed to read disabled modules JSON: {}", e);
            return -1;
        }
    };

    let manifest: AccelManifest = match serde_json::from_str(&manifest_str) {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to parse manifest: {}", e);
            return -2;
        }
    };

    let disabled_modules: Vec<String> = match serde_json::from_str(&disabled_modules_str) {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to parse disabled modules: {}", e);
            return -3;
        }
    };

    let registry = ModuleRegistry::from_manifest(manifest);
    
    // Disable modules specified in config
    for module_id in disabled_modules {
        if let Some(module) = registry.get(&module_id) {
            module.set_state(ModuleState::Disabled, Some("Disabled via config".to_string()));
            info!("Module '{}' disabled via config", module_id);
        } else {
            warn!("Config references unknown module '{}'", module_id);
        }
    }

    let module_count = registry.module_count();

    *REGISTRY.write() = Some(Arc::new(registry));
    info!("Registered {} accelerator modules", module_count);
    module_count as jint
}

/// JNI export: run the conflict scanner against installed mods.
///
/// Returns 0 on success, negative on error. Conflict report is available via getConflictReport.
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_horizonlab_accel_AccelNative_scanConflicts(
    mut env: JNIEnv<'_>,
    _class: JClass,
    mods_data_json: JString,
) -> jint {
    let mods_str: String = match env.get_string(&mods_data_json) {
        Ok(s) => s.into(),
        Err(e) => {
            error!("Failed to read mods data JSON: {}", e);
            return -1;
        }
    };

    let mods_data: Vec<ModMixinData> = match serde_json::from_str(&mods_str) {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to parse mods data: {}", e);
            return -2;
        }
    };

    let registry = match REGISTRY.read().as_ref().cloned() {
        Some(r) => r,
        None => {
            error!("Module registry not initialized");
            return -3;
        }
    };

    let scanner = ConflictScanner::new(registry.clone());
    let report = scanner.scan(&mods_data);

    // Apply fallbacks based on conflicts
    for conflict in &report.conflicts {
        warn!("Conflict detected: module '{}' supersedes '{}' but mod '{}' targets it",
            conflict.module_name, conflict.superseded_class, conflict.mod_id);
        FALLBACK.request_fallback(
            &conflict.module_name,
            FallbackReason::Conflict {
                mod_id: conflict.mod_id.clone(),
                superseded_class: conflict.superseded_class.clone(),
            },
        );
    }

    // Store report for Java to retrieve
    FALLBACK.set_conflict_report(report);

    0
}

/// JNI export: get the conflict report as JSON.
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_horizonlab_accel_AccelNative_getConflictReport<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
) -> JString<'local> {
    let report = FALLBACK.get_conflict_report();
    let json = serde_json::to_string(&report).unwrap_or_else(|_| "{}".to_string());
    env.new_string(json).unwrap_or_else(|_| env.new_string("{}").unwrap())
}

/// JNI export: check if a module is currently active (not fallen back).
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_horizonlab_accel_AccelNative_isModuleActive(
    mut env: JNIEnv<'_>,
    _class: JClass,
    module_name: JString,
) -> jboolean {
    let name: String = match env.get_string(&module_name) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };

    let registry = match REGISTRY.read().as_ref().cloned() {
        Some(r) => r,
        None => return 0,
    };

    if registry.is_active(&name) {
        1
    } else {
        0
    }
}

/// JNI export: called when a module's Rust code panics or returns an error.
/// Triggers fallback for that module.
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_horizonlab_accel_AccelNative_reportModuleError(
    mut env: JNIEnv<'_>,
    _class: JClass,
    module_name: JString,
    error_msg: JString,
) {
    let name: String = match env.get_string(&module_name) {
        Ok(s) => s.into(),
        Err(_) => return,
    };
    let msg: String = match env.get_string(&error_msg) {
        Ok(s) => s.into(),
        Err(_) => return,
    };

    error!("Module '{}' reported error: {}", name, msg);
    FALLBACK.request_fallback(&name, FallbackReason::RustError(msg));
}

/// JNI export: get the list of all modules and their states as JSON.
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_horizonlab_accel_AccelNative_getModuleStates<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
) -> JString<'local> {
    let registry = match REGISTRY.read().as_ref().cloned() {
        Some(r) => r,
        None => return env.new_string("{}").unwrap(),
    };

    let states = registry.get_all_states();
    let json = serde_json::to_string(&states).unwrap_or_else(|_| "{}".to_string());
    env.new_string(json).unwrap_or_else(|_| env.new_string("{}").unwrap())
}

// Re-export JNI_VERSION_1_8 for the JVM
#[unsafe(no_mangle)]
pub extern "system" fn JNI_OnLoad(
    _vm: *mut jni::sys::JavaVM,
    _reserved: *mut std::os::raw::c_void,
) -> jint {
    // Initialize tracing if not already done
    let _ = tracing_subscriber::fmt()
        .with_env_filter("horizon_accel=info,warn,error")
        .try_init();
    JNI_VERSION_1_8
}

/// JNI bindings for ChunkIoModule native methods
#[cfg(feature = "chunk_io")]
mod chunk_io_ffi {
    use crate::jni::objects::{JClass, JString, JByteArray, JObject};
    use crate::jni::sys::{jboolean, jint, jlong};
    use crate::jni::JNIEnv;
    use crate::chunk_io::ChunkIoModule;
    use std::sync::Arc;
    use tracing::{debug, error, warn};

    // Global ChunkIoModule instance
    static mut CHUNK_IO_MODULE: Option<Arc<ChunkIoModule>> = None;

    fn get_module() -> Option<Arc<ChunkIoModule>> {
        unsafe { CHUNK_IO_MODULE.clone() }
    }

    fn set_module(module: Arc<ChunkIoModule>) {
        unsafe { CHUNK_IO_MODULE = Some(module); }
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_ChunkIoModule_initialize(
        mut env: JNIEnv<'_>,
        _class: JClass,
        module_ptr: jlong,
    ) -> jboolean {
        let entry = unsafe {
            assert!(!(module_ptr as *const crate::module::ModuleEntry).is_null());
            Arc::from_raw(module_ptr as *const crate::module::ModuleEntry)
        };
        match crate::chunk_io::init_chunk_io_module(entry) {
            Ok(module) => {
                set_module(module);
                1
            }
            Err(e) => {
                error!("Failed to initialize Chunk I/O module: {}", e);
                0
            }
        }
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_ChunkIoModule_readChunk<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass,
        _module_ptr: jlong,
        path: JString,
        chunk_x: jint,
        chunk_z: jint,
    ) -> JByteArray<'local> {
        let path_str: String = match env.get_string(&path) {
            Ok(s) => s.into(),
            Err(_) => return env.new_byte_array(0).unwrap(),
        };

        let module = match get_module() {
            Some(m) => m,
            None => return env.new_byte_array(0).unwrap(),
        };

        let path = std::path::Path::new(&path_str);
        match module.read_chunk(path, chunk_x as i32, chunk_z as i32) {
            Ok(Some(data)) => env.new_byte_array(data.len()).unwrap().write_bytes(&data).unwrap(),
            Ok(None) => env.new_byte_array(0).unwrap(),
            Err(e) => {
                warn!("Native readChunk failed: {}", e);
                env.new_byte_array(0).unwrap()
            }
        }
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_ChunkIoModule_writeChunk(
        mut env: JNIEnv<'_>,
        _class: JClass,
        _module_ptr: jlong,
        path: JString,
        chunk_x: jint,
        chunk_z: jint,
        data: JByteArray,
    ) -> jboolean {
        let path_str: String = match env.get_string(&path) {
            Ok(s) => s.into(),
            Err(_) => return 0,
        };

        let data: Vec<u8> = match env.convert_byte_array(&data) {
            Ok(d) => d,
            Err(_) => return 0,
        };

        let module = match get_module() {
            Some(m) => m,
            None => return 0,
        };

        let path = std::path::Path::new(&path_str);
        match module.write_chunk(path, chunk_x as i32, chunk_z as i32, &data) {
            Ok(true) => 1,
            Ok(false) => 0,
            Err(e) => {
                warn!("Native writeChunk failed: {}", e);
                0
            }
        }
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_ChunkIoModule_readChunkFromPath(
        mut env: JNIEnv<'_>,
        _class: JClass,
        _module_ptr: jlong,
        path: JString,
        _registries: JObject,
    ) -> jlong {
        // Return 0 for now - full Chunk object serialization is complex
        // This would require serializing the Chunk object to bytes
        0
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_ChunkIoModule_writeChunkToPath(
        mut env: JNIEnv<'_>,
        _class: JClass,
        _module_ptr: jlong,
        _chunk: JObject,
        _path: JString,
        _registries: JObject,
    ) -> jboolean {
        0
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_ChunkIoModule_parseNbt<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass,
        _module_ptr: jlong,
        data: JByteArray,
    ) -> jlong {
        // Return 0 for now - full NbtCompound serialization is complex
        0
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_ChunkIoModule_writeNbt(
        mut env: JNIEnv<'_>,
        _class: JClass,
        _module_ptr: jlong,
        _compound: JObject,
    ) -> JByteArray<'_> {
        env.new_byte_array(0).unwrap()
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_ChunkIoModule_getPalettedValue(
        mut env: JNIEnv<'_>,
        _class: JClass,
        _module_ptr: jlong,
        _container: JObject,
        index: jint,
    ) -> jint {
        // This would need to access the PalettedContainer object's fields
        // For now, return 0
        0
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_ChunkIoModule_setPalettedValue(
        mut env: JNIEnv<'_>,
        _class: JClass,
        _module_ptr: jlong,
        _container: JObject,
        index: jint,
        value: jint,
    ) {
        // Implementation would go here
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::*;
    use crate::module::*;
    use crate::scanner::*;
    use crate::handshake::*;
    use std::sync::Arc;

    // ========================================================================
    // Handshake Tests
    // ========================================================================

    #[test]
    fn handshake_version_match() {
        // Java and Rust agree on protocol version 1
        let result = handshake(ACCEL_PROTOCOL_VERSION);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ACCEL_PROTOCOL_VERSION);
    }

    #[test]
    fn handshake_version_mismatch_too_old() {
        // Java sends version 0 (too old)
        let result = handshake(0);
        assert!(result.is_err());
        match result.unwrap_err() {
            AccelError::ProtocolMismatch { expected, actual } => {
                assert_eq!(expected, MIN_PROTOCOL_VERSION);
                assert_eq!(actual, 0);
            }
            _ => panic!("Expected ProtocolMismatch error"),
        }
    }

    #[test]
    fn handshake_version_mismatch_too_new() {
        // Java sends version 2 (too new for current max of 1)
        let result = handshake(2);
        assert!(result.is_err());
        match result.unwrap_err() {
            AccelError::ProtocolMismatch { expected, actual } => {
                assert_eq!(expected, MAX_PROTOCOL_VERSION);
                assert_eq!(actual, 2);
            }
            _ => panic!("Expected ProtocolMismatch error"),
        }
    }

    // ========================================================================
    // Manifest Parsing Tests
    // ========================================================================

    #[test]
    fn manifest_parsing_valid() {
        let json = r#"{
            "schema_version": 1,
            "name": "test-accel",
            "version": "1.0.0",
            "min_protocol_version": 1,
            "max_protocol_version": 1,
            "modules": [
                {
                    "id": "test_module",
                    "name": "Test Module",
                    "description": "A test module",
                    "enabled_by_default": true,
                    "feature_flag": "test",
                    "supersedes_classes": [
                        { "class": "net.minecraft.TestClass", "reason": "Test", "soft": false }
                    ],
                    "supersedes_methods": [],
                    "supersedes_fields": [],
                    "config": {}
                }
            ],
            "fallback": {
                "auto_fallback_on_conflict": true,
                "auto_fallback_on_error": true,
                "notify_player": true,
                "log_level": "warn"
            }
        }"#;

        let manifest = load_manifest(json).expect("Valid manifest should parse");
        assert_eq!(manifest.name, "test-accel");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.modules.len(), 1);
        assert_eq!(manifest.modules[0].id, "test_module");
        assert_eq!(manifest.modules[0].supersedes_classes.len(), 1);
        assert_eq!(manifest.modules[0].supersedes_classes[0].class, "net.minecraft.TestClass");
    }

    #[test]
    fn manifest_parsing_malformed_json() {
        let json = "{ this is not valid json }";
        let result = load_manifest(json);
        assert!(result.is_err());
    }

    #[test]
    fn manifest_parsing_missing_required_fields() {
        // Missing required "name" field
        let json = r#"{
            "schema_version": 1,
            "version": "1.0.0",
            "modules": []
        }"#;
        let result = load_manifest(json);
        assert!(result.is_err());
    }

    #[test]
    fn manifest_parsing_empty_module_id_rejected() {
        let json = r#"{
            "schema_version": 1,
            "name": "test-accel",
            "version": "1.0.0",
            "modules": [
                { "id": "", "name": "Test", "description": "Test", "enabled_by_default": true, "supersedes_classes": [], "supersedes_methods": [], "supersedes_fields": [], "config": {} }
            ],
            "fallback": { "auto_fallback_on_conflict": true, "auto_fallback_on_error": true, "notify_player": true, "log_level": "warn" }
        }"#;
        let result = load_manifest(json);
        assert!(result.is_err());
    }

    #[test]
    fn manifest_parsing_invalid_schema_version() {
        let json = r#"{
            "schema_version": 999,
            "name": "test-accel",
            "version": "1.0.0",
            "modules": [],
            "fallback": { "auto_fallback_on_conflict": true, "auto_fallback_on_error": true, "notify_player": true, "log_level": "warn" }
        }"#;
        let result = load_manifest(json);
        assert!(result.is_err());
    }

    // ========================================================================
    // Conflict Scanner Tests
    // ========================================================================

    fn create_test_manifest() -> AccelManifest {
        let json = r#"{
            "schema_version": 1,
            "name": "test-accel",
            "version": "1.0.0",
            "min_protocol_version": 1,
            "max_protocol_version": 1,
            "modules": [
                {
                    "id": "test_module",
                    "name": "Test Module",
                    "description": "A test module",
                    "enabled_by_default": true,
                    "feature_flag": "test",
                    "supersedes_classes": [
                        { "class": "net.minecraft.TestClass", "reason": "Test", "soft": false },
                        { "class": "net.minecraft.SoftClass", "reason": "Test soft", "soft": true }
                    ],
                    "supersedes_methods": [],
                    "supersedes_fields": [],
                    "config": {}
                }
            ],
            "fallback": {
                "auto_fallback_on_conflict": true,
                "auto_fallback_on_error": true,
                "notify_player": true,
                "log_level": "warn"
            }
        }"#;
        load_manifest(json).expect("Test manifest should parse")
    }

    fn create_test_registry() -> Arc<ModuleRegistry> {
        let manifest = create_test_manifest();
        Arc::new(ModuleRegistry::from_manifest(manifest))
    }

    #[test]
    fn conflict_scanner_mod_targets_superseded_class_gets_module_disabled() {
        let registry = create_test_registry();
        let scanner = ConflictScanner::new(registry.clone());

        // Mod targets a class that the module supersedes (hard conflict)
        let mods = vec![
            ModMixinData {
                mod_id: "conflicting_mod".to_string(),
                version: "1.0.0".to_string(),
                mixin_targets: vec!["net.minecraft.TestClass".to_string()],
                access_widener_targets: vec![],
                reflection_targets: vec![],
                is_library: false,
            }
        ];

        let report = scanner.scan(&mods);

        // Should detect 1 hard conflict
        assert_eq!(report.total_conflicts, 1);
        assert_eq!(report.hard_conflicts, 1);
        assert_eq!(report.soft_conflicts, 0);
        assert_eq!(report.conflicts.len(), 1);
        assert_eq!(report.conflicts[0].module_name, "test_module");
        assert_eq!(report.conflicts[0].superseded_class, "net.minecraft.TestClass");
        assert_eq!(report.conflicts[0].mod_id, "conflicting_mod");
        assert_eq!(report.conflicts[0].target_type, "mixin");
        assert_eq!(report.conflicts[0].severity, "error");

        // Module should be marked for fallback
        assert_eq!(report.modules_fallback.len(), 1);
        assert_eq!(report.modules_fallback[0], "test_module");
        assert!(report.conflicting_mods.contains(&"conflicting_mod".to_string()));
    }

    #[test]
    fn conflict_scanner_mod_targets_soft_superseded_class_gets_warning() {
        let registry = create_test_registry();
        let scanner = ConflictScanner::new(registry.clone());

        // Mod targets a class that the module soft-supersedes
        let mods = vec![
            ModMixinData {
                mod_id: "soft_conflict_mod".to_string(),
                version: "1.0.0".to_string(),
                mixin_targets: vec!["net.minecraft.SoftClass".to_string()],
                access_widener_targets: vec![],
                reflection_targets: vec![],
                is_library: false,
            }
        ];

        let report = scanner.scan(&mods);

        // Should detect 1 soft conflict (warning only)
        assert_eq!(report.total_conflicts, 1);
        assert_eq!(report.hard_conflicts, 0);
        assert_eq!(report.soft_conflicts, 1);
        assert_eq!(report.conflicts[0].severity, "warn");

        // Module should NOT be marked for fallback (soft conflict)
        assert_eq!(report.modules_fallback.len(), 0);
    }

    #[test]
    fn conflict_scanner_mod_with_no_overlap_changes_nothing() {
        let registry = create_test_registry();
        let scanner = ConflictScanner::new(registry.clone());

        // Mod targets completely different classes
        let mods = vec![
            ModMixinData {
                mod_id: "unrelated_mod".to_string(),
                version: "1.0.0".to_string(),
                mixin_targets: vec!["net.minecraft.UnrelatedClass".to_string(), "net.minecraft.AnotherClass".to_string()],
                access_widener_targets: vec![],
                reflection_targets: vec![],
                is_library: false,
            }
        ];

        let report = scanner.scan(&mods);

        // Should detect NO conflicts
        assert_eq!(report.total_conflicts, 0);
        assert_eq!(report.hard_conflicts, 0);
        assert_eq!(report.soft_conflicts, 0);
        assert_eq!(report.conflicts.len(), 0);
        assert_eq!(report.modules_fallback.len(), 0);
        assert_eq!(report.conflicting_mods.len(), 0);
    }

    #[test]
    fn conflict_scanner_access_widener_conflict() {
        let registry = create_test_registry();
        let scanner = ConflictScanner::new(registry.clone());

        // Mod uses access widener on a superseded class
        let mods = vec![
            ModMixinData {
                mod_id: "access_widener_mod".to_string(),
                version: "1.0.0".to_string(),
                mixin_targets: vec![],
                access_widener_targets: vec!["net.minecraft.TestClass".to_string()],
                reflection_targets: vec![],
                is_library: false,
            }
        ];

        let report = scanner.scan(&mods);

        assert_eq!(report.total_conflicts, 1);
        assert_eq!(report.conflicts[0].target_type, "access_widener");
        assert_eq!(report.conflicts[0].severity, "error");
        assert_eq!(report.modules_fallback.len(), 1);
    }

    #[test]
    fn conflict_scanner_reflection_conflict() {
        let registry = create_test_registry();
        let scanner = ConflictScanner::new(registry.clone());

        // Mod uses reflection on a superseded class
        let mods = vec![
            ModMixinData {
                mod_id: "reflection_mod".to_string(),
                version: "1.0.0".to_string(),
                mixin_targets: vec![],
                access_widener_targets: vec![],
                reflection_targets: vec!["net.minecraft.TestClass".to_string()],
                is_library: false,
            }
        ];

        let report = scanner.scan(&mods);

        assert_eq!(report.total_conflicts, 1);
        assert_eq!(report.conflicts[0].target_type, "reflection");
        assert_eq!(report.conflicts[0].severity, "error");
        assert_eq!(report.modules_fallback.len(), 1);
    }

    #[test]
    fn conflict_scanner_multiple_mods_multiple_conflicts() {
        let registry = create_test_registry();
        let scanner = ConflictScanner::new(registry.clone());

        // Multiple mods, one conflicting, one not
        let mods = vec![
            ModMixinData {
                mod_id: "conflicting_mod".to_string(),
                version: "1.0.0".to_string(),
                mixin_targets: vec!["net.minecraft.TestClass".to_string()],
                access_widener_targets: vec![],
                reflection_targets: vec![],
                is_library: false,
            },
            ModMixinData {
                mod_id: "unrelated_mod".to_string(),
                version: "2.0.0".to_string(),
                mixin_targets: vec!["net.minecraft.UnrelatedClass".to_string()],
                access_widener_targets: vec![],
                reflection_targets: vec![],
                is_library: false,
            }
        ];

        let report = scanner.scan(&mods);

        assert_eq!(report.total_conflicts, 1);
        assert_eq!(report.hard_conflicts, 1);
        assert_eq!(report.modules_fallback.len(), 1);
        assert_eq!(report.conflicting_mods.len(), 1);
        assert!(report.conflicting_mods.contains(&"conflicting_mod".to_string()));
        assert!(!report.conflicting_mods.contains(&"unrelated_mod".to_string()));
    }

    #[test]
    fn conflict_scanner_module_not_active_skipped() {
        let registry = create_test_registry();
        
        // Disable the module
        if let Some(module) = registry.get("test_module") {
            module.set_state(ModuleState::Disabled, Some("disabled by config".to_string()));
        }
        
        let scanner = ConflictScanner::new(registry.clone());

        let mods = vec![
            ModMixinData {
                mod_id: "conflicting_mod".to_string(),
                version: "1.0.0".to_string(),
                mixin_targets: vec!["net.minecraft.TestClass".to_string()],
                access_widener_targets: vec![],
                reflection_targets: vec![],
                is_library: false,
            }
        ];

        let report = scanner.scan(&mods);

        // Module is disabled, so no conflicts should be detected
        assert_eq!(report.total_conflicts, 0);
        assert_eq!(report.modules_fallback.len(), 0);
    }
}