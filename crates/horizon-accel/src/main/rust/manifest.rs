//! Accelerator manifest format.
//!
//! Each accelerator module declares what vanilla classes/methods it supersedes.
//! The conflict scanner uses this to detect when a mod's mixins or access wideners
//! target something the module replaces.

use serde::{Deserialize, Serialize, de::Error as DeError};
use std::collections::HashMap;

/// Full accelerator manifest, embedded as a resource in the mod JAR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccelManifest {
    /// Manifest schema version.
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,

    /// Accelerator name (e.g., "horizon-accel").
    pub name: String,

    /// Accelerator version (semver).
    pub version: String,

    /// Minimum protocol version required.
    #[serde(default = "default_min_protocol")]
    pub min_protocol_version: u32,

    /// Maximum protocol version supported.
    #[serde(default = "default_max_protocol")]
    pub max_protocol_version: u32,

    /// List of modules in this accelerator.
    pub modules: Vec<ModuleManifest>,

    /// Global fallback settings.
    #[serde(default)]
    pub fallback: FallbackConfig,
}

fn default_schema_version() -> u32 { 1 }
fn default_min_protocol() -> u32 { 1 }
fn default_max_protocol() -> u32 { u32::MAX }

/// Per-module declaration of what it supersedes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleManifest {
    /// Unique module identifier (e.g., "chunk_io", "worldgen", "lighting").
    pub id: String,

    /// Human-readable name.
    pub name: String,

    /// Description of what this module accelerates.
    pub description: String,

    /// Whether this module is enabled by default.
    #[serde(default = "default_true")]
    pub enabled_by_default: bool,

    /// Feature flag required to compile this module.
    #[serde(default)]
    pub feature_flag: Option<String>,

    /// Vanilla classes this module fully replaces.
    /// If a mod targets any of these (via mixin, access widener, or reflection),
    /// the module will be auto-disabled for that instance.
    #[serde(default)]
    pub supersedes_classes: Vec<SupersedesEntry>,

    /// Specific methods this module replaces (more granular than whole classes).
    #[serde(default)]
    pub supersedes_methods: Vec<SupersedesMethodEntry>,

    /// Vanilla fields this module replaces.
    #[serde(default)]
    pub supersedes_fields: Vec<SupersedesFieldEntry>,

    /// Minimum Minecraft version this module supports.
    #[serde(default)]
    pub min_minecraft_version: Option<String>,

    /// Maximum Minecraft version this module supports.
    #[serde(default)]
    pub max_minecraft_version: Option<String>,

    /// Module-specific configuration (passed to module init).
    #[serde(default)]
    pub config: serde_json::Value,
}

fn default_true() -> bool { true }

/// A vanilla class that a module supersedes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupersedesEntry {
    /// Fully qualified class name (e.g., "net.minecraft.world.chunk.Chunk").
    pub class: String,

    /// Reason for superseding (for diagnostics).
    pub reason: String,

    /// Whether this is a "soft" supersession — the module provides an alternative
    /// implementation but the vanilla class still exists and may be instantiated.
    #[serde(default)]
    pub soft: bool,
}

/// A specific vanilla method that a module supersedes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupersedesMethodEntry {
    /// Declaring class.
    pub class: String,

    /// Method name.
    pub name: String,

    /// JVM descriptor (e.g., "(Lnet/minecraft/core/BlockPos;)V").
    pub descriptor: String,

    /// Reason for superseding.
    pub reason: String,
}

/// A specific vanilla field that a module supersedes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupersedesFieldEntry {
    /// Declaring class.
    pub class: String,

    /// Field name.
    pub name: String,

    /// JVM descriptor (e.g., "Lnet/minecraft/world/level/Chunk;").
    pub descriptor: String,

    /// Reason for superseding.
    pub reason: String,
}

/// Global fallback behavior configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackConfig {
    /// Whether to automatically fall back on conflicts.
    #[serde(default = "default_true")]
    pub auto_fallback_on_conflict: bool,

    /// Whether to automatically fall back on Rust errors/panics.
    #[serde(default = "default_true")]
    pub auto_fallback_on_error: bool,

    /// Whether to notify the player when a fallback occurs.
    #[serde(default = "default_true")]
    pub notify_player: bool,

    /// Log level for fallback events.
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_log_level() -> String { "warn".to_string() }

impl Default for FallbackConfig {
    fn default() -> Self {
        Self {
            auto_fallback_on_conflict: true,
            auto_fallback_on_error: true,
            notify_player: true,
            log_level: "warn".to_string(),
        }
    }
}

/// Manifest resource path inside the mod JAR.
pub const MANIFEST_RESOURCE_PATH: &str = "accel-manifest.json";

/// Load manifest from a JSON string.
pub fn load_manifest(json: &str) -> Result<AccelManifest, serde_json::Error> {
    let mut manifest: AccelManifest = serde_json::from_str(json)?;
    // Validate
    if manifest.schema_version != 1 {
        return Err(<serde_json::Error as DeError>::custom(format!(
            "Unsupported manifest schema version: {}",
            manifest.schema_version
        )));
    }
    for module in &manifest.modules {
        if module.id.is_empty() {
            return Err(<serde_json::Error as DeError>::custom("Module id cannot be empty"));
        }
        if module.name.is_empty() {
            return Err(<serde_json::Error as DeError>::custom("Module name cannot be empty"));
        }
    }
    Ok(manifest)
}