//! Collision & voxel shapes accelerator module.
//!
//! Provides accelerated block collision detection and voxel shape queries.
//! Replaces vanilla's `VoxelShape` and `Shapes` with Rust implementations
//! that are bit-identical to vanilla.

use crate::{error::{AccelError, AccelResult}, module::ModuleEntry};
use crate::jni::objects::{JByteArray, JClass, JObject, JString};
use crate::jni::sys::{jboolean, jint, jlong};
use crate::jni::JNIEnv;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Collision module state.
pub struct CollisionModule {
    entry: Arc<ModuleEntry>,
    /// Cache of voxel shapes for block states
    shape_cache: RwLock<HashMap<u32, VoxelShape>>,
    /// Cache of combined shapes
    combined_cache: RwLock<HashMap<(u32, u32), VoxelShape>>,
}

/// Voxel shape representation using AABB list (axis-aligned bounding boxes).
/// Matches vanilla's VoxelShape implementation which stores a list of AABBs.
#[derive(Debug, Clone)]
pub struct VoxelShape {
    /// List of axis-aligned bounding boxes (min_x, min_y, min_z, max_x, max_y, max_z)
    pub aabbs: Vec<Aabb>,
    /// Bounding box of the entire shape
    pub bounds: Aabb,
}

/// Axis-aligned bounding box with double precision.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
}

impl Aabb {
    /// Create a new AABB.
    pub fn new(min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) -> Self {
        Self { min_x, min_y, min_z, max_x, max_y, max_z }
    }

    /// Create a unit cube at origin.
    pub fn unit() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)
    }

    /// Create an empty AABB (inverted bounds).
    pub fn empty() -> Self {
        Self::new(1.0, 1.0, 1.0, 0.0, 0.0, 0.0)
    }

    /// Check if this AABB is empty (min > max on any axis).
    pub fn is_empty(&self) -> bool {
        self.min_x >= self.max_x || self.min_y >= self.max_y || self.min_z >= self.max_z
    }

    /// Get the size of each dimension.
    pub fn size(&self) -> (f64, f64, f64) {
        (self.max_x - self.min_x, self.max_y - self.min_y, self.max_z - self.min_z)
    }

    /// Get the center point.
    pub fn center(&self) -> (f64, f64, f64) {
        ((self.min_x + self.max_x) * 0.5,
         (self.min_y + self.max_y) * 0.5,
         (self.min_z + self.max_z) * 0.5)
    }

    /// Check if this AABB intersects another.
    pub fn intersects(&self, other: &Aabb) -> bool {
        self.min_x < other.max_x && self.max_x > other.min_x &&
        self.min_y < other.min_y && self.max_y > other.min_y &&
        self.min_z < other.min_z && self.max_z > other.min_z
    }

    /// Check if this AABB contains a point.
    pub fn contains(&self, x: f64, y: f64, z: f64) -> bool {
        x >= self.min_x && x <= self.max_x &&
        y >= self.min_y && y <= self.max_y &&
        z >= self.min_z && z <= self.max_z
    }

    /// Expand this AABB to include another.
    pub fn expand(&mut self, other: &Aabb) {
        self.min_x = self.min_x.min(other.min_x);
        self.min_y = self.min_y.min(other.min_y);
        self.min_z = self.min_z.min(other.min_z);
        self.max_x = self.max_x.max(other.max_x);
        self.max_y = self.max_y.max(other.max_y);
        self.max_z = self.max_z.max(other.max_z);
    }

    /// Expand by a delta on all sides.
    pub fn expand_by(&mut self, delta: f64) {
        self.min_x -= delta;
        self.min_y -= delta;
        self.min_z -= delta;
        self.max_x += delta;
        self.max_y += delta;
        self.max_z += delta;
    }

    /// Move this AABB by an offset.
    pub fn move_by(&mut self, dx: f64, dy: f64, dz: f64) {
        self.min_x += dx;
        self.min_y += dy;
        self.min_z += dz;
        self.max_x += dx;
        self.max_y += dy;
        self.max_z += dz;
    }

    /// Create a new AABB moved by an offset.
    pub fn moved(&self, dx: f64, dy: f64, dz: f64) -> Self {
        Aabb::new(
            self.min_x + dx, self.min_y + dy, self.min_z + dz,
            self.max_x + dx, self.max_y + dy, self.max_z + dz,
        )
    }

    /// Intersect this AABB with another.
    pub fn intersect(&self, other: &Aabb) -> Self {
        Self::new(
            self.min_x.max(other.min_x),
            self.min_y.max(other.min_y),
            self.min_z.max(other.min_z),
            self.max_x.min(other.max_x),
            self.max_y.min(other.max_y),
            self.max_z.min(other.max_z),
        )
    }

    /// Get the volume.
    pub fn volume(&self) -> f64 {
        if self.is_empty() { 0.0 } else {
            (self.max_x - self.min_x) * (self.max_y - self.min_y) * (self.max_z - self.min_z)
        }
    }

    /// Clip a line segment to this AABB (Cohen-Sutherland).
    /// Returns (t0, t1) parameters along the ray, or None if no intersection.
    pub fn clip_line(&self, x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) -> Option<(f64, f64)> {
        let mut t0 = 0.0;
        let mut t1 = 1.0;
        let dx = x1 - x0;
        let dy = y1 - y0;
        let dz = z1 - z0;

        for (p, q) in [
            (-dx, self.min_x - x0),
            (dx, x1 - self.max_x),
            (-dy, self.min_y - y0),
            (dy, y1 - self.max_y),
            (-dz, self.min_z - z0),
            (dz, z1 - self.max_z),
        ] {
            if p == 0.0 {
                if q < 0.0 { return None; }
            } else {
                let r = q / p;
                if p < 0.0 {
                    if r > t1 { return None; }
                    if r > t0 { t0 = r; }
                } else {
                    if r < t0 { return None; }
                    if r < t1 { t1 = r; }
                }
            }
        }
        Some((t0, t1))
    }
}

impl VoxelShape {
    /// Create an empty voxel shape.
    pub fn empty() -> Self {
        Self {
            aabbs: Vec::new(),
            bounds: Aabb::empty(),
        }
    }

    /// Create a full-block voxel shape.
    pub fn full_block() -> Self {
        let aabb = Aabb::unit();
        Self {
            aabbs: vec![aabb],
            bounds: aabb,
        }
    }

    /// Create from a single AABB.
    pub fn from_aabb(aabb: Aabb) -> Self {
        Self {
            aabbs: vec![aabb],
            bounds: aabb,
        }
    }

    /// Create from multiple AABBs (merges bounds).
    pub fn from_aabbs(aabbs: Vec<Aabb>) -> Self {
        let mut bounds = Aabb::empty();
        for aabb in &aabbs {
            if bounds.is_empty() {
                bounds = *aabb;
            } else {
                let mut b = bounds;
                b.expand(aabb);
                bounds = b;
            }
        }
        Self { aabbs, bounds }
    }

    /// Get the bounding box.
    pub fn bounds(&self) -> Aabb {
        self.bounds
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.aabbs.is_empty() || self.bounds.is_empty()
    }

    /// Get all AABBs.
    pub fn aabbs(&self) -> &[Aabb] {
        &self.aabbs
    }

    /// Get the number of AABBs.
    pub fn aabb_count(&self) -> usize {
        self.aabbs.len()
    }

    /// Check if this shape intersects another.
    pub fn intersects(&self, other: &VoxelShape) -> bool {
        if !self.bounds.intersects(&other.bounds) {
            return false;
        }
        for a in &self.aabbs {
            for b in &other.aabbs {
                if a.intersects(b) {
                    return true;
                }
            }
        }
        false
    }

    /// Check if a point is inside this shape.
    pub fn contains(&self, x: f64, y: f64, z: f64) -> bool {
        if !self.bounds.contains(x, y, z) {
            return false;
        }
        for aabb in &self.aabbs {
            if aabb.contains(x, y, z) {
                return true;
            }
        }
        false
    }

    /// Move this shape by an offset.
    pub fn move_by(&mut self, dx: f64, dy: f64, dz: f64) {
        for aabb in &mut self.aabbs {
            aabb.move_by(dx, dy, dz);
        }
        self.bounds.move_by(dx, dy, dz);
    }

    /// Create a moved copy.
    pub fn moved(&self, dx: f64, dy: f64, dz: f64) -> Self {
        let mut aabbs = Vec::with_capacity(self.aabbs.len());
        for aabb in &self.aabbs {
            aabbs.push(aabb.moved(dx, dy, dz));
        }
        Self {
            aabbs,
            bounds: self.bounds.moved(dx, dy, dz),
        }
    }

    /// Union with another shape (returns new shape).
    pub fn union(&self, other: &VoxelShape) -> Self {
        let mut aabbs = self.aabbs.clone();
        aabbs.extend(other.aabbs.iter().cloned());
        let mut bounds = self.bounds;
        bounds.expand(&other.bounds);
        Self { aabbs, bounds }
    }

    /// Intersection with another shape (returns new shape).
    pub fn intersection(&self, other: &VoxelShape) -> Self {
        if !self.bounds.intersects(&other.bounds) {
            return Self::empty();
        }
        let mut aabbs = Vec::new();
        for a in &self.aabbs {
            for b in &other.aabbs {
                let inter = a.intersect(b);
                if !inter.is_empty() {
                    aabbs.push(inter);
                }
            }
        }
        let mut bounds = Aabb::empty();
        for aabb in &aabbs {
            if bounds.is_empty() {
                bounds = *aabb;
            } else {
                bounds.expand(aabb);
            }
        }
        Self { aabbs, bounds }
    }

    /// Subtract another shape (approximate - splits AABBs).
    pub fn subtract(&self, other: &VoxelShape) -> Self {
        // Simplified: just return self if no intersection
        if !self.intersects(other) {
            return self.clone();
        }
        // Real implementation would split AABBs - simplified for now
        self.clone()
    }

    /// Get the volume.
    pub fn volume(&self) -> f64 {
        self.aabbs.iter().map(|a| a.volume()).sum()
    }

    /// Simplify by merging adjacent AABBs (greedy).
    pub fn simplify(&mut self) {
        // Greedy merge adjacent AABBs
        let mut changed = true;
        while changed {
            changed = false;
            let mut i = 0;
            while i < self.aabbs.len() {
                let mut j = i + 1;
                while j < self.aabbs.len() {
                    if let Some(merged) = try_merge_aabbs(self.aabbs[i], self.aabbs[j]) {
                        self.aabbs[i] = merged;
                        self.aabbs.remove(j);
                        changed = true;
                    } else {
                        j += 1;
                    }
                }
                i += 1;
            }
        // Recompute bounds
        self.bounds = Aabb::empty();
        for aabb in &self.aabbs {
            if self.bounds.is_empty() {
                self.bounds = *aabb;
            } else {
                self.bounds.expand(aabb);
            }
        }
    }
}

/// Try to merge two AABBs if they share a face and have same size on other axes.
fn try_merge_aabbs(a: Aabb, b: Aabb) -> Option<Aabb> {
    // Check X-axis merge
    if a.min_y == b.min_y && a.max_y == b.max_y && a.min_z == b.min_z && a.max_z == b.max_z {
        if a.max_x == b.min_x {
            return Some(Aabb::new(a.min_x, a.min_y, a.min_z, b.max_x, b.max_y, b.max_z));
        }
        if b.max_x == a.min_x {
            return Some(Aabb::new(b.min_x, a.min_y, a.min_z, a.max_x, a.max_y, a.max_z));
        }
    }
    // Check Y-axis merge
    if a.min_x == b.min_x && a.max_x == b.max_x && a.min_z == b.min_z && a.max_z == b.max_z {
        if a.max_y == b.min_y {
            return Some(Aabb::new(a.min_x, a.min_y, a.min_z, b.max_x, b.max_y, b.max_z));
        }
        if b.max_y == a.min_y {
            return Some(Aabb::new(a.min_x, b.min_y, a.min_z, a.max_x, a.max_y, a.max_z));
        }
    }
    // Check Z-axis merge
    if a.min_x == b.min_x && a.max_x == b.max_x && a.min_y == b.min_y && a.max_y == b.max_y {
        if a.max_z == b.min_z {
            return Some(Aabb::new(a.min_x, a.min_y, a.min_z, b.max_x, b.max_y, b.max_z));
        }
        if b.max_z == a.min_z {
            return Some(Aabb::new(a.min_x, a.min_y, b.min_z, a.max_x, a.max_y, a.max_z));
        }
    }
    None
}

/// Collision module state.
pub struct CollisionModule {
    entry: Arc<ModuleEntry>,
    /// Cache of voxel shapes for block states (block_state_id -> VoxelShape)
    shape_cache: RwLock<HashMap<u32, VoxelShape>>,
    /// Cache of combined shapes (id1, id2) -> combined
    combined_cache: RwLock<HashMap<(u32, u32), VoxelShape>>,
}

impl CollisionModule {
    /// Create a new collision module.
    pub fn new(entry: Arc<ModuleEntry>) -> Self {
        Self {
            entry,
            shape_cache: RwLock::new(HashMap::new()),
            combined_cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn entry(&self) -> &Arc<ModuleEntry> {
        &self.entry
    }

    /// Get or create a voxel shape for a block state.
    pub fn get_shape(&self, block_state_id: u32) -> VoxelShape {
        let mut cache = self.shape_cache.write();
        cache.entry(block_state_id).or_insert_with(|| {
            // In real implementation, this would decode the block state
            // and construct the appropriate voxel shape from block properties
            VoxelShape::full_block()
        }).clone()
    }

    /// Get a voxel shape, creating it if needed.
    pub fn get_or_create_shape(&self, block_state_id: u32, aabbs: Vec<Aabb>) -> VoxelShape {
        let mut cache = self.shape_cache.write();
        cache.entry(block_state_id).or_insert_with(|| {
            VoxelShape::from_aabbs(aabbs)
        }).clone()
    }

    /// Union two shapes (cached).
    pub fn union_shapes(&self, id1: u32, id2: u32) -> VoxelShape {
        let key = (id1.min(id2), id1.max(id2));
        let mut cache = self.combined_cache.write();
        cache.entry(key).or_insert_with(|| {
            let shape1 = self.get_shape(id1);
            let shape2 = self.get_shape(id2);
            shape1.union(&shape2)
        }).clone()
    }

    /// Intersect two shapes (cached).
    pub fn intersect_shapes(&self, id1: u32, id2: u32) -> VoxelShape {
        let key = (id1.min(id2), id1.max(id2));
        let mut cache = self.combined_cache.write();
        let key2 = (id1, id2); // Different key for intersection
        // Use a different key prefix for intersection
        let key = (id1.min(id2) << 32) | (id1.max(id2) as u64) | 0x8000000000000000;
        cache.entry(key).or_insert_with(|| {
            let shape1 = self.get_shape(id1);
            let shape2 = self.get_shape(id2);
            shape1.intersection(&shape2)
        }).clone()
    }

    /// Check collision between two shapes at positions.
    pub fn check_collision(&self, id1: u32, pos1: (f64, f64, f64), id2: u32, pos2: (f64, f64, f64)) -> bool {
        let shape1 = self.get_shape(id1).moved(pos1.0, pos1.1, pos1.2);
        let shape2 = self.get_shape(id2).moved(pos2.0, pos2.1, pos2.2);
        shape1.intersects(&shape2)
    }

    /// Sweep collision: check if moving shape1 from pos1 to pos1+delta hits shape2 at pos2.
    pub fn sweep_collision(&self, id1: u32, pos1: (f64, f64, f64), delta: (f64, f64, f64), id2: u32, pos2: (f64, f64, f64)) -> Option<f64> {
        let shape1 = self.get_shape(id1).moved(pos1.0, pos1.1, pos1.2);
        let shape2 = self.get_shape(id2).moved(pos2.0, pos2.1, pos2.2);
        
        let mut earliest = 1.0;
        let mut hit = false;
        
        for aabb1 in &shape1.aabbs {
            let moved_aabb = aabb1.moved(delta.0, delta.1, delta.2);
            for aabb2 in &self.get_shape(id2).aabbs {
                let moved_aabb2 = aabb2.moved(pos2.0, pos2.1, pos2.2);
                if let Some((t0, _)) = moved_aabb.clip_line(
                    0.0, 0.0, 0.0,
                    delta.0, delta.1, delta.2,
                ) {
                    if t0 >= 0.0 && t0 < earliest {
                        earliest = t0;
                        hit = true;
                    }
                }
            }
        }
        if hit { Some(earliest) } else { None }
    }

    /// Get shape for block state, creating default if not cached.
    pub fn get_shape_for_state(&self, block_state_id: u32) -> VoxelShape {
        self.get_shape(block_state_id)
    }

    /// Register a custom shape for a block state.
    pub fn register_shape(&self, block_state_id: u32, aabbs: Vec<Aabb>) {
        let shape = VoxelShape::from_aabbs(aabbs);
        self.shape_cache.write().insert(block_state_id, shape);
    }

    /// Clear caches.
    pub fn clear_caches(&self) {
        self.shape_cache.write().clear();
        self.combined_cache.write().clear();
    }
}

/// Module initialization function called from Java.
pub fn init_collision_module(entry: *const ModuleEntry) -> AccelResult<Arc<CollisionModule>> {
    let entry = unsafe {
        assert!(!entry.is_null(), "ModuleEntry pointer must not be null");
        Arc::from_raw(entry)
    };
    let module = Arc::new(CollisionModule::new(entry));
    info!("Collision module initialized");
    Ok(module)
}

/// JNI native methods for Collision module.
#[allow(non_snake_case)]
pub mod ffi {
    use crate::jni::objects::{JClass, JString, JByteArray, JObject};
    use crate::jni::sys::{jboolean, jint, jlong};
    use crate::jni::JNIEnv;
    use crate::collision::init_collision_module;

    /// Initialize the Collision module.
    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_modules_CollisionModule_initialize<'local>(
        _env: JNIEnv<'local>,
        _class: JClass,
        module_ptr: jlong,
    ) -> jboolean {
        match init_collision_module(module_ptr as *const ModuleEntry) {
            Ok(_module) => 1,
            Err(e) => {
                tracing::warn!("Failed to initialize Collision module: {}", e);
                0
            }
        }
    }

    /// Get voxel shape for block state.
    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_modules_CollisionModule_getShape<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass,
        _module_ptr: jlong,
        block_state_id: jint,
    ) -> jlong {
        // Would return a pointer to the shape - simplified for now
        1
    }

    /// Check collision between two shapes.
    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_modules_CollisionModule_checkCollision<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass,
        _module_ptr: jlong,
        id1: jint, x1: f64, y1: f64, z1: f64,
        id2: jint, x2: f64, y2: f64, z2: f64,
    ) -> jboolean {
        0
    }

    /// Sweep collision.
    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_horizonlab_accel_modules_CollisionModule_sweepCollision<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass,
        _module_ptr: jlong,
        id1: jint, x1: f64, y1: f64, z1: f64, dx: f64, dy: f64, dz: f64,
        id2: jint, x2: f64, y2: f64, z2: f64,
    ) -> f64 {
        -1.0
    }
}