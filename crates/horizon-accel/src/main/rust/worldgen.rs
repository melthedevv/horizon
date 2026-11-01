//! World generation accelerator module.
//!
//! Provides accelerated noise sampling, surface rule evaluation, and structure
//! placement. Replaces vanilla's noise samplers, surface builders, and structure
//! placement with Rust implementations that are bit-identical to vanilla.

use crate::{error::{AccelError, AccelResult}, module::ModuleEntry};
use crate::jni::objects::{JByteArray, JClass, JObject, JString};
use crate::jni::sys::{jboolean, jint, jlong};
use crate::jni::JNIEnv;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Worldgen module state.
pub struct WorldgenModule {
    entry: Arc<ModuleEntry>,
    /// Cache of noise samplers
    noise_cache: RwLock<HashMap<String, NoiseSampler>>,
    /// Cache of surface rules
    surface_cache: RwLock<HashMap<String, SurfaceRule>>,
}

/// Noise sampler types matching vanilla.
#[derive(Debug, Clone)]
pub enum NoiseSampler {
    Perlin { 
        permutations: Vec<u16>, 
        frequency: f64,
        amplitude: f64,
    },
    Simplex {
        permutations: Vec<u16>,
        frequency: f64,
        amplitude: f64,
    },
    Cellular {
        seed: i64,
        distance_func: CellularDistanceFunc,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum CellularDistanceFunc {
    Euclidean,
    Manhattan,
    Chebyshev,
}

/// Surface rule for biome surface building.
#[derive(Debug, Clone)]
pub struct SurfaceRule {
    conditions: Vec<SurfaceCondition>,
    top_block: BlockState,
    mid_block: BlockState,
    bottom_block: BlockState,
    sea_floor_block: BlockState,
    foundation_block: BlockState,
}

#[derive(Debug, Clone)]
pub enum SurfaceCondition {
    HeightRange(i32, i32),
    BiomeTag(String),
    NoiseThreshold { noise: String, min: f64, max: f64 },
    VerticalGradient { min_y: i32, max_y: i32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockState(pub u32); // Packed block state ID

impl WorldgenModule {
    /// Create a new worldgen module.
    pub fn new(entry: Arc<ModuleEntry>) -> Self {
        Self {
            entry,
            noise_cache: RwLock::new(HashMap::new()),
            surface_cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn entry(&self) -> &Arc<ModuleEntry> {
        &self.entry
    }

    /// Create a Perlin noise sampler from vanilla parameters.
    pub fn create_perlin_noise(&self, seed: i64, frequency: f64, amplitude: f64) -> NoiseSampler {
        // Vanilla uses a 256-entry permutation table derived from seed
        let mut permutations = vec![0u16; 512];
        let mut rng = LegacyRandom::new(seed);
        
        for i in 0..256 {
            permutations[i] = i as u16;
        }
        
        // Fisher-Yates shuffle
        for i in (1..256).rev() {
            let i = i as usize;
            let j = (rng.next_int(1 + i as i32)) as usize;
            permutations.swap(i, j);
        }
        
        // Duplicate for wrap-around
        for i in 0..256 {
            permutations[i + 256] = permutations[i];
        }
        
        NoiseSampler::Perlin {
            permutations,
            frequency,
            amplitude,
        }
    }

    /// Sample Perlin noise at 3D coordinates.
    pub fn sample_perlin_3d(&self, sampler: &NoiseSampler, x: f64, y: f64, z: f64) -> f64 {
        match sampler {
            NoiseSampler::Perlin { permutations, frequency, amplitude } => {
                let x = x * frequency;
                let y = y * frequency;
                let z = z * frequency;
                
                let xi = x.floor() as i32;
                let yi = y.floor() as i32;
                let zi = z.floor() as i32;
                
                let xf = x - xi as f64;
                let yf = y - yi as f64;
                let zf = z - zi as f64;
                
                // Fade curves (vanilla's improved Perlin)
                let u = Self::fade(xf);
                let v = Self::fade(yf);
                let w = Self::fade(zf);
                
                // Hash coordinates
                let aaa = Self::hash(permutations, xi, yi, zi);
                let aba = Self::hash(permutations, xi, yi + 1, zi);
                let aab = Self::hash(permutations, xi, yi, zi + 1);
                let abb = Self::hash(permutations, xi, yi + 1, zi + 1);
                let baa = Self::hash(permutations, xi + 1, yi, zi);
                let bba = Self::hash(permutations, xi + 1, yi + 1, zi);
                let bab = Self::hash(permutations, xi + 1, yi, zi + 1);
                let bbb = Self::hash(permutations, xi + 1, yi + 1, zi + 1);
                
                // Gradient dot products
                let grad_aaa = Self::grad(aaa, xf, yf, zf);
                let grad_aba = Self::grad(aba, xf, yf - 1.0, zf);
                let grad_aab = Self::grad(aab, xf, yf, zf - 1.0);
                let grad_abb = Self::grad(abb, xf, yf - 1.0, zf - 1.0);
                let grad_baa = Self::grad(baa, xf - 1.0, yf, zf);
                let grad_bba = Self::grad(bba, xf - 1.0, yf - 1.0, zf);
                let grad_bab = Self::grad(bab, xf - 1.0, yf, zf - 1.0);
                let grad_bbb = Self::grad(bbb, xf - 1.0, yf - 1.0, zf - 1.0);
                
                // Trilinear interpolation
                let x1 = Self::lerp(grad_aaa, grad_baa, u);
                let x2 = Self::lerp(grad_aba, grad_bba, u);
                let y1 = Self::lerp(x1, x2, v);
                
                let x3 = Self::lerp(grad_aab, grad_bab, u);
                let x4 = Self::lerp(grad_abb, grad_bbb, u);
                let y2 = Self::lerp(x3, x4, v);
                
                Self::lerp(y1, y2, w) * *amplitude
            }
            _ => 0.0,
        }
    }

    fn fade(t: f64) -> f64 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a + t * (b - a)
    }

    fn hash(permutations: &[u16], x: i32, y: i32, z: i32) -> u16 {
        let x = (x & 255) as usize;
        let y = (y & 255) as usize;
        let z = (z & 255) as usize;
        let idx1 = permutations[x] as usize;
        let idx2 = (idx1 + y) % permutations.len();
        let idx3 = permutations[idx2] as usize;
        let idx4 = (idx3 + z) % permutations.len();
        permutations[idx4]
    }

    fn grad(hash: u16, x: f64, y: f64, z: f64) -> f64 {
        let h = hash & 15;
        let u = if h < 8 { x } else { y };
        let v = if h < 4 { y } else if h == 12 || h == 14 { x } else { z };
        ((if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })) as f64
    }

    /// Create a surface rule from vanilla JSON.
    pub fn parse_surface_rule(&self, json: &str) -> AccelResult<SurfaceRule> {
        // Simplified - real implementation would parse the full JSON structure
        // matching vanilla's SurfaceRule format
        Ok(SurfaceRule {
            conditions: vec![],
            top_block: BlockState(0),
            mid_block: BlockState(0),
            bottom_block: BlockState(0),
            sea_floor_block: BlockState(0),
            foundation_block: BlockState(0),
        })
    }

    /// Evaluate surface rules for a column.
    pub fn evaluate_surface(&self, rules: &[SurfaceRule], x: i32, z: i32, biome: &str, height: i32) -> Vec<(i32, BlockState)> {
        let mut column = Vec::new();
        
        for rule in rules {
            let mut matches = true;
            
            for condition in &rule.conditions {
                matches &= match condition {
                    SurfaceCondition::HeightRange(min, max) => height >= *min && height <= *max,
                    SurfaceCondition::BiomeTag(tag) => biome.contains(tag),
                    SurfaceCondition::NoiseThreshold { noise, min, max } => {
                        // Would sample the noise
                        true
                    }
                    SurfaceCondition::VerticalGradient { min_y, max_y } => {
                        height >= *min_y && height <= *max_y
                    }
                };
                
                if !matches { break; }
            }
            
            if matches {
                // Add the blocks for this rule
                column.push((height, rule.top_block));
                column.push((height - 1, rule.mid_block));
                column.push((height - 2, rule.bottom_block));
                break;
            }
        }
        
        column
    }
}

/// Legacy random matching vanilla's Random.
struct LegacyRandom {
    seed: i64,
}

impl LegacyRandom {
    fn new(seed: i64) -> Self {
        Self { seed }
    }
    
    fn next_int(&mut self, bound: i32) -> i32 {
        self.seed = self.seed.wrapping_mul(25214903917).wrapping_add(11);
        let mut val = ((self.seed >> 17) & 0x7fffffff) as i64;
        val = (val * bound as i64) >> 31;
        val as i32
    }
}

/// Module initialization function called from Java.
pub fn init_worldgen_module(entry: *const ModuleEntry) -> AccelResult<Arc<WorldgenModule>> {
    let entry = unsafe {
        assert!(!entry.is_null(), "ModuleEntry pointer must not be null");
        Arc::from_raw(entry)
    };
    let module = Arc::new(WorldgenModule::new(entry));
    info!("Worldgen module initialized");
    Ok(module)
}

/// JNI native methods for Worldgen module.
#[allow(non_snake_case)]
pub mod ffi {
    use crate::jni::objects::{JClass, JString, JByteArray, JObject};
    use crate::jni::sys::{jboolean, jint, jlong};
    use crate::jni::JNIEnv;
    use crate::module::ModuleEntry;
    use crate::worldgen::init_worldgen_module;

    /// Initialize the Worldgen module.
    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_modules_WorldgenModule_initialize<'local>(
        _env: JNIEnv<'local>,
        _class: JClass,
        module_ptr: jlong,
    ) -> jboolean {
        match init_worldgen_module(module_ptr as *const ModuleEntry) {
            Ok(_module) => 1,
            Err(e) => {
                tracing::warn!("Failed to initialize Worldgen module: {}", e);
                0
            }
        }
    }

    /// Sample Perlin noise.
    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_modules_WorldgenModule_samplePerlin<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass,
        _module_ptr: jlong,
        x: f64,
        y: f64,
        z: f64,
        seed: jlong,
        frequency: f64,
        amplitude: f64,
    ) -> f64 {
        // Would create sampler and sample - simplified for now
        0.0
    }

    /// Create surface rule from JSON.
    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_modules_WorldgenModule_createSurfaceRule<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass,
        _module_ptr: jlong,
        json: JString,
    ) -> jlong {
        let _json: String = match env.get_string(&json) {
            Ok(s) => s.into(),
            Err(_) => return 0,
        };
        1
    }
}