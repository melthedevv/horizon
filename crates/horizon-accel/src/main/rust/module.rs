//! Module registry and state management.

use crate::manifest::{ModuleManifest, SupersedesEntry};
use crate::error::AccelResult;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

/// Runtime state of a module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleState {
    /// Module is active and handling requests.
    Active,
    /// Module is disabled by configuration.
    Disabled,
    /// Module fell back to vanilla due to conflict.
    FallbackConflict,
    /// Module fell back due to Rust error/panic.
    FallbackError,
    /// Module fell back due to version incompatibility.
    FallbackVersion,
    /// Module is not compiled in (feature flag not enabled).
    NotCompiled,
}

/// A module entry in the registry.
#[derive(Debug)]
pub struct ModuleEntry {
    pub manifest: ModuleManifest,
    pub state: RwLock<ModuleState>,
    pub fallback_reason: RwLock<Option<String>>,
    pub supersedes_classes: Vec<SupersedesEntry>,
    pub supersedes_methods: Vec<crate::manifest::SupersedesMethodEntry>,
    pub supersedes_fields: Vec<crate::manifest::SupersedesFieldEntry>,
}

impl ModuleEntry {
    pub fn new(manifest: ModuleManifest) -> Self {
        let supersedes_classes = manifest.supersedes_classes.clone();
        let supersedes_methods = manifest.supersedes_methods.clone();
        let supersedes_fields = manifest.supersedes_fields.clone();
        let initial_state = if manifest.enabled_by_default {
            ModuleState::Active
        } else {
            ModuleState::Disabled
        };
        Self {
            manifest,
            state: RwLock::new(initial_state),
            fallback_reason: RwLock::new(None),
            supersedes_classes,
            supersedes_methods,
            supersedes_fields,
        }
    }

    pub fn id(&self) -> &str {
        &self.manifest.id
    }

    pub fn name(&self) -> &str {
        &self.manifest.name
    }

    pub fn state(&self) -> ModuleState {
        *self.state.read()
    }

    pub fn set_state(&self, state: ModuleState, reason: Option<String>) {
        *self.state.write() = state;
        if let Some(r) = reason {
            *self.fallback_reason.write() = Some(r);
        }
    }

    pub fn fallback_reason(&self) -> Option<String> {
        self.fallback_reason.read().clone()
    }

    pub fn is_active(&self) -> bool {
        matches!(self.state(), ModuleState::Active)
    }

    pub fn supersedes_class(&self, class_name: &str) -> Option<&SupersedesEntry> {
        self.supersedes_classes.iter().find(|e| e.class == class_name)
    }

    /// All classes this module supersedes (for conflict scanner).
    pub fn all_superseded_classes(&self) -> Vec<String> {
        self.supersedes_classes.iter().map(|e| e.class.clone()).collect()
    }
}

/// Registry of all accelerator modules.
#[derive(Debug)]
pub struct ModuleRegistry {
    modules: HashMap<String, Arc<ModuleEntry>>,
    load_order: Vec<String>,
}

impl ModuleRegistry {
    /// Create a registry from a manifest.
    pub fn from_manifest(manifest: crate::manifest::AccelManifest) -> Self {
        let mut modules = HashMap::new();
        let mut load_order = Vec::new();

        for module_manifest in manifest.modules {
            let entry = Arc::new(ModuleEntry::new(module_manifest));
            let id = entry.id().to_string();
            modules.insert(id.clone(), entry);
            load_order.push(id);
        }

        Self { modules, load_order }
    }

    /// Get a module by ID.
    pub fn get(&self, id: &str) -> Option<Arc<ModuleEntry>> {
        self.modules.get(id).cloned()
    }

    /// Get all modules in load order.
    pub fn all(&self) -> Vec<Arc<ModuleEntry>> {
        self.load_order.iter()
            .filter_map(|id| self.modules.get(id).cloned())
            .collect()
    }

    /// Number of registered modules.
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Check if a module is active.
    pub fn is_active(&self, id: &str) -> bool {
        self.modules.get(id).map(|m| m.is_active()).unwrap_or(false)
    }

    /// Request fallback for a module.
    pub fn request_fallback(&self, id: &str, reason: String) -> bool {
        if let Some(module) = self.modules.get(id) {
            if module.is_active() {
                module.set_state(crate::module::ModuleState::FallbackError, Some(reason));
                warn!("Module '{}' fell back: {}", id, module.fallback_reason().unwrap_or_default());
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Get all module states as a serializable map.
    pub fn get_all_states(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        for (id, module) in &self.modules {
            let state = module.state();
            let reason = module.fallback_reason();
            let mut obj = serde_json::Map::new();
            obj.insert("state".to_string(), serde_json::to_value(state).unwrap());
            obj.insert("name".to_string(), serde_json::to_value(module.name()).unwrap());
            obj.insert("description".to_string(), serde_json::to_value(&module.manifest.description).unwrap());
            if let Some(r) = reason {
                obj.insert("fallback_reason".to_string(), serde_json::to_value(r).unwrap());
            }
            obj.insert("supersedes_classes".to_string(), serde_json::to_value(&module.supersedes_classes).unwrap());
            map.insert(id.clone(), serde_json::Value::Object(obj));
        }
        serde_json::Value::Object(map)
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self {
            modules: HashMap::new(),
            load_order: Vec::new(),
        }
    }
}