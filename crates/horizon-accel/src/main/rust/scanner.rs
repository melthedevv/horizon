//! Conflict scanner for detecting mod/accelerator incompatibilities.

use crate::manifest::{AccelManifest, SupersedesEntry};
use crate::module::{ModuleEntry, ModuleRegistry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Mixin and access widener data extracted from a mod JAR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModMixinData {
    /// Mod ID (from fabric.mod.json).
    pub mod_id: String,

    /// Mod version.
    pub version: String,

    /// Mixin target classes from all mixin configs in this mod.
    #[serde(default)]
    pub mixin_targets: Vec<String>,

    /// Access widener targets (classes whose access is widened).
    #[serde(default)]
    pub access_widener_targets: Vec<String>,

    /// Classes this mod references via reflection (Class.forName, etc.).
    #[serde(default)]
    pub reflection_targets: Vec<String>,

    /// Whether this mod is a library mod (no mixins of its own, but others depend on it).
    #[serde(default)]
    pub is_library: bool,
}

/// A single conflict between a module and a mod.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    /// The accelerator module name.
    pub module_name: String,

    /// The vanilla class the module supersedes.
    pub superseded_class: String,

    /// The mod that targets this class.
    pub mod_id: String,

    /// How the mod targets it: "mixin", "access_widener", "reflection".
    pub target_type: String,

    /// The specific mixin class (if applicable).
    #[serde(default)]
    pub mixin_class: Option<String>,

    /// Severity: "error" (must fallback) or "warn" (soft supersession).
    pub severity: String,
}

/// Full conflict report for a scan.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConflictReport {
    /// All detected conflicts.
    pub conflicts: Vec<Conflict>,

    /// Modules that will be auto-disabled due to conflicts.
    pub modules_fallback: Vec<String>,

    /// Mods that caused conflicts.
    pub conflicting_mods: Vec<String>,

    /// Summary counts.
    pub total_conflicts: usize,
    pub hard_conflicts: usize,
    pub soft_conflicts: usize,
}

impl ConflictReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_conflict(&mut self, conflict: Conflict) {
        self.total_conflicts += 1;
        if conflict.severity == "error" {
            self.hard_conflicts += 1;
        } else {
            self.soft_conflicts += 1;
        }
        if !self.conflicting_mods.contains(&conflict.mod_id) {
            self.conflicting_mods.push(conflict.mod_id.clone());
        }
        self.conflicts.push(conflict);
    }
}

/// Conflict scanner that checks mods against accelerator modules.
pub struct ConflictScanner {
    registry: Arc<ModuleRegistry>,
}

impl ConflictScanner {
    /// Create a new scanner for the given registry.
    pub fn new(registry: Arc<ModuleRegistry>) -> Self {
        Self { registry }
    }

    /// Scan all mods for conflicts with accelerator modules.
    pub fn scan(&self, mods: &[ModMixinData]) -> ConflictReport {
        let mut report = ConflictReport::new();

        for mod_data in mods {
            self.scan_mod(mod_data, &mut report);
        }

        // Collect modules that need to fallback
        for conflict in &report.conflicts {
            if conflict.severity == "error" && !report.modules_fallback.contains(&conflict.module_name) {
                report.modules_fallback.push(conflict.module_name.clone());
            }
        }

        info!("Conflict scan complete: {} mods, {} conflicts ({} hard, {} soft)",
            mods.len(), report.total_conflicts, report.hard_conflicts, report.soft_conflicts);

        report
    }

    /// Scan a single mod against all modules.
    fn scan_mod(&self, mod_data: &ModMixinData, report: &mut ConflictReport) {
        for module in self.registry.all() {
            // Skip modules that are already disabled/not compiled
            if !matches!(module.state(), crate::module::ModuleState::Active) {
                continue;
            }

            // Check class supersessions
            for supersedes in &module.supersedes_classes {
                // Check mixin targets
                if mod_data.mixin_targets.iter().any(|t| t.as_str() == supersedes.class.as_str()) {
                    report.add_conflict(Conflict {
                        module_name: module.id().to_string(),
                        superseded_class: supersedes.class.clone(),
                        mod_id: mod_data.mod_id.clone(),
                        target_type: "mixin".to_string(),
                        mixin_class: mod_data.mixin_targets.iter()
                            .find(|t| t.as_str() == supersedes.class.as_str())
                            .cloned(),
                        severity: if supersedes.soft { "warn" } else { "error" }.to_string(),
                    });
                }

                // Check access widener targets
                if mod_data.access_widener_targets.iter().any(|t| t == &supersedes.class) {
                    report.add_conflict(Conflict {
                        module_name: module.id().to_string(),
                        superseded_class: supersedes.class.clone(),
                        mod_id: mod_data.mod_id.clone(),
                        target_type: "access_widener".to_string(),
                        mixin_class: None,
                        severity: if supersedes.soft { "warn" } else { "error" }.to_string(),
                    });
                }

                // Check reflection targets
                if mod_data.reflection_targets.iter().any(|t| t == &supersedes.class) {
                    report.add_conflict(Conflict {
                        module_name: module.id().to_string(),
                        superseded_class: supersedes.class.clone(),
                        mod_id: mod_data.mod_id.clone(),
                        target_type: "reflection".to_string(),
                        mixin_class: None,
                        severity: if supersedes.soft { "warn" } else { "error" }.to_string(),
                    });
                }
            }

            // Check method supersessions
            for supersedes in &module.supersedes_methods {
                // For methods, we check if the class is targeted
                if mod_data.mixin_targets.iter().any(|t| t.as_str() == supersedes.class.as_str()) {
                    report.add_conflict(Conflict {
                        module_name: module.id().to_string(),
                        superseded_class: format!("{}#{} ({})", supersedes.class, supersedes.name, supersedes.descriptor),
                        mod_id: mod_data.mod_id.clone(),
                        target_type: "mixin_method".to_string(),
                        mixin_class: mod_data.mixin_targets.iter()
                            .find(|t| t.as_str() == supersedes.class.as_str())
                            .cloned(),
                        severity: "error".to_string(),
                    });
                }
            }

            // Check field supersessions
            for supersedes in &module.supersedes_fields {
                if mod_data.mixin_targets.iter().any(|t| t.as_str() == supersedes.class.as_str()) {
                    report.add_conflict(Conflict {
                        module_name: module.id().to_string(),
                        superseded_class: format!("{}#{} ({})", supersedes.class, supersedes.name, supersedes.descriptor),
                        mod_id: mod_data.mod_id.clone(),
                        target_type: "mixin_field".to_string(),
                        mixin_class: mod_data.mixin_targets.iter()
                            .find(|t| t.as_str() == supersedes.class.as_str())
                            .cloned(),
                        severity: "error".to_string(),
                    });
                }
            }
        }
    }
}

/// Load mod mixin data from the installed mods directory.
/// This is called from Java side which has access to the mod JARs.
pub fn load_mod_mixin_data(_mods_dir: &std::path::Path) -> Vec<ModMixinData> {
    // This is a placeholder - actual implementation is in Java
    // which has access to the mod JARs and can extract mixin configs
    Vec::new()
}