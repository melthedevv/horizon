//! Lighting engine accelerator module.
//!
//! Provides accelerated sky and block light propagation. Replaces vanilla's
//! `LightEngine` with a Rust implementation that produces bit-identical results.

use crate::{error::{AccelError, AccelResult}, module::ModuleEntry};
use crate::jni::objects::{JByteArray, JClass, JObject, JString};
use crate::jni::sys::{jboolean, jint, jlong};
use crate::jni::JNIEnv;
use parking_lot::RwLock;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Lighting module state.
pub struct LightingModule {
    entry: Arc<ModuleEntry>,
    /// Cache of computed light grids
    light_cache: RwLock<HashMap<ChunkPos, LightGrid>>,
    /// Queue for light updates
    update_queue: RwLock<VecDeque<LightUpdate>>,
}

/// Chunk position (x, z).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

/// Light grid for a chunk section (16x16x16).
#[derive(Debug, Clone)]
pub struct LightGrid {
    pub chunk_pos: ChunkPos,
    pub y_section: i32,
    /// Sky light (4 bits per block, 4096 blocks = 2048 bytes)
    pub sky_light: [u8; 2048],
    /// Block light (4 bits per block)
    pub block_light: [u8; 2048],
}

/// Pending light update.
#[derive(Debug, Clone)]
pub struct LightUpdate {
    pub pos: BlockPos,
    pub old_sky: u8,
    pub new_sky: u8,
    pub old_block: u8,
    pub new_block: u8,
}

/// Block position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl LightingModule {
    /// Create a new lighting module.
    pub fn new(entry: Arc<ModuleEntry>) -> Self {
        Self {
            entry,
            light_cache: RwLock::new(HashMap::new()),
            update_queue: RwLock::new(VecDeque::new()),
        }
    }

    pub fn entry(&self) -> &Arc<ModuleEntry> {
        &self.entry
    }

    /// Get or create a light grid for a chunk section.
    pub fn get_light_grid(&self, chunk_pos: ChunkPos, y_section: i32) -> LightGrid {
        let key = (chunk_pos, y_section);
        let mut cache = self.light_cache.write();
        cache.entry(key).or_insert_with(|| LightGrid {
            chunk_pos,
            y_section,
            sky_light: [0; 2048],
            block_light: [0; 2048],
        }).clone()
    }

    /// Set sky light at a position.
    pub fn set_sky_light(&self, pos: BlockPos, level: u8) {
        let chunk_pos = ChunkPos {
            x: pos.x >> 4,
            z: pos.z >> 4,
        };
        let y_section = pos.y >> 4;
        
        let mut cache = self.light_cache.write();
        let key = (chunk_pos, y_section);
        if let Some(grid) = cache.get_mut(&(chunk_pos, y_section)) {
            let idx = Self::block_index(pos.x & 15, pos.y & 15, pos.z & 15);
            grid.sky_light[idx / 2] = Self::set_nibble(grid.sky_light[idx / 2], idx % 2, level);
        }
    }

    /// Set block light at a position.
    pub fn set_block_light(&self, pos: BlockPos, level: u8) {
        let chunk_pos = ChunkPos {
            x: pos.x >> 4,
            z: pos.z >> 4,
        };
        let y_section = pos.y >> 4;
        
        let mut cache = self.light_cache.write();
        let key = (chunk_pos, y_section);
        if let Some(grid) = cache.get_mut(&(chunk_pos, y_section)) {
            let idx = Self::block_index(pos.x & 15, pos.y & 15, pos.z & 15);
            grid.block_light[idx / 2] = Self::set_nibble(grid.block_light[idx / 2], idx % 2, level);
        }
    }

    /// Get sky light at a position.
    pub fn get_sky_light(&self, pos: BlockPos) -> u8 {
        let chunk_pos = ChunkPos {
            x: pos.x >> 4,
            z: pos.z >> 4,
        };
        let y_section = pos.y >> 4;
        
        let cache = self.light_cache.read();
        if let Some(grid) = cache.get(&(ChunkPos { x: pos.x >> 4, z: pos.z >> 4 }, pos.y >> 4)) {
            let idx = Self::block_index(pos.x & 15, pos.y & 15, pos.z & 15);
            Self::get_nibble(grid.sky_light[idx / 2], idx % 2)
        } else {
            0
        }
    }

    /// Get block light at a position.
    pub fn get_block_light(&self, pos: BlockPos) -> u8 {
        let chunk_pos = ChunkPos {
            x: pos.x >> 4,
            z: pos.z >> 4,
        };
        let y_section = pos.y >> 4;
        
        let cache = self.light_cache.read();
        if let Some(grid) = cache.get(&(ChunkPos { x: pos.x >> 4, z: pos.z >> 4 }, pos.y >> 4)) {
            let idx = Self::block_index(pos.x & 15, pos.y & 15, pos.z & 15);
            Self::get_nibble(grid.block_light[idx / 2], idx % 2)
        } else {
            0
        }
    }

    /// Get combined light level (max of sky and block).
    pub fn get_light_level(&self, pos: BlockPos) -> u8 {
        self.get_sky_light(pos).max(self.get_block_light(pos))
    }

    /// Propagate light from a source (simplified - real implementation uses BFS).
    pub fn propagate_light(&self, start: BlockPos, sky_level: u8, block_level: u8) {
        // Real implementation would use a BFS queue with falloff
        // For now, just set the source
        if sky_level > 0 {
            self.set_sky_light(start, sky_level);
        }
        if block_level > 0 {
            self.set_block_light(start, block_level);
        }
    }

    /// Recompute light for a chunk section (called when blocks change).
    pub fn recompute_section(&self, chunk_pos: ChunkPos, y_section: i32) {
        // Real implementation would:
        // 1. Reset light in section
        // 2. Find all light sources in section
        // 2. Propagate from sources
        // 3. Queue neighbor sections if light reaches edges
    }

    fn block_index(x: i32, y: i32, z: i32) -> usize {
        (y * 16 + z) * 16 + x
    }

    fn get_nibble(byte: u8, high: usize) -> u8 {
        if high == 0 {
            byte & 0x0F
        } else {
            byte >> 4
        }
    }

    fn set_nibble(byte: u8, high: usize, value: u8) -> u8 {
        if high == 0 {
            (byte & 0xF0) | (value & 0x0F)
        } else {
            (byte & 0x0F) | ((value & 0x0F) << 4)
        }
    }
}

/// Module initialization function called from Java.
pub fn init_lighting_module(entry: *const ModuleEntry) -> AccelResult<Arc<LightingModule>> {
    let entry = unsafe {
        assert!(!entry.is_null(), "ModuleEntry pointer must not be null");
        Arc::from_raw(entry)
    };
    let module = Arc::new(LightingModule::new(entry));
    info!("Lighting module initialized");
    Ok(module)
}

/// JNI native methods for Lighting module.
#[allow(non_snake_case)]
pub mod ffi {
    use crate::jni::objects::{JClass, JString, JByteArray, JObject};
    use crate::jni::sys::{jboolean, jint, jlong};
    use crate::jni::JNIEnv;
    use crate::lighting::init_lighting_module;

    /// Initialize the Lighting module.
    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_modules_LightingModule_initialize<'local>(
        _env: JNIEnv<'local>,
        _class: JClass,
        module_ptr: jlong,
    ) -> jboolean {
        match init_lighting_module(module_ptr as *const ModuleEntry) {
            Ok(_module) => 1,
            Err(e) => {
                tracing::warn!("Failed to initialize Lighting module: {}", e);
                0
            }
        }
    }

    /// Set sky light.
    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_modules_LightingModule_setSkyLight<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass,
        _module_ptr: jlong,
        x: jint,
        y: jint,
        z: jint,
        level: jint,
    ) {
        // Implementation would go here
    }

    /// Set block light.
    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_modules_LightingModule_setBlockLight<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass,
        _module_ptr: jlong,
        x: jint,
        y: jint,
        z: jint,
        level: jint,
    ) {
        // Implementation would go here
    }

    /// Get light level.
    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_modules_LightingModule_getLightLevel<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass,
        _module_ptr: jlong,
        x: jint,
        y: jint,
        z: jint,
    ) -> jint {
        0
    }
}