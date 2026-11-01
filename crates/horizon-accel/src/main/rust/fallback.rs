//! Fallback handling for when modules must disable themselves.

use crate::error::AccelError;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{warn};

/// Reason a module fell back to vanilla.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FallbackReason {
    /// Conflict with a mod's mixin/access widener.
    Conflict {
        mod_id: String,
        superseded_class: String,
    },

    /// Rust panic or error in the module.
    RustError(String),

    /// Version incompatibility (Minecraft or protocol).
    VersionMismatch {
        expected: String,
        actual: String,
    },

    /// Module explicitly disabled by config.
    DisabledByConfig,

    /// Feature flag not enabled at compile time.
    NotCompiled,
}

/// Fallback handler - tracks fallback state and coordinates with Java.
pub struct FallbackHandler {
    /// Per-module fallback state.
    states: RwLock<HashMap<String, FallbackState>>,

    /// Latest conflict report.
    conflict_report: RwLock<crate::scanner::ConflictReport>,
}

/// Internal fallback state for a module.
#[derive(Debug, Clone)]
struct FallbackState {
    reason: FallbackReason,
    timestamp: std::time::SystemTime,
    notified_player: bool,
}

impl FallbackHandler {
    pub fn new() -> Self {
        Self {
            states: RwLock::new(HashMap::new()),
            conflict_report: RwLock::new(crate::scanner::ConflictReport::new()),
        }
    }

    /// Request a fallback for a module.
    pub fn request_fallback(&self, module_id: &str, reason: FallbackReason) {
        let mut states = self.states.write();
        let state = FallbackState {
            reason: reason.clone(),
            timestamp: std::time::SystemTime::now(),
            notified_player: false,
        };
        states.insert(module_id.to_string(), state);

        warn!("Module '{}' requested fallback: {:?}", module_id, reason);

        // In a real implementation, this would notify the Java side
        // to update the module state and show a notification to the player
    }

    /// Get the fallback reason for a module, if any.
    pub fn get_fallback_reason(&self, module_id: &str) -> Option<FallbackReason> {
        self.states.read().get(module_id).map(|s| s.reason.clone())
    }

    /// Check if a module is in fallback.
    pub fn is_in_fallback(&self, module_id: &str) -> bool {
        self.states.read().contains_key(module_id)
    }

    /// Mark a module's fallback as notified to the player.
    pub fn mark_notified(&self, module_id: &str) {
        if let Some(state) = self.states.write().get_mut(module_id) {
            state.notified_player = true;
        }
    }

    /// Set the conflict report (called after scanning).
    pub fn set_conflict_report(&self, report: crate::scanner::ConflictReport) {
        *self.conflict_report.write() = report;
    }

    /// Get the current conflict report.
    pub fn get_conflict_report(&self) -> crate::scanner::ConflictReport {
        self.conflict_report.read().clone()
    }

    /// Get all modules currently in fallback.
    pub fn get_all_fallbacks(&self) -> Vec<(String, FallbackReason)> {
        self.states.read().iter()
            .map(|(k, v)| (k.clone(), v.reason.clone()))
            .collect()
    }

    /// Clear all fallback states (e.g., on new scan).
    pub fn clear(&self) {
        self.states.write().clear();
    }
}

impl Default for FallbackHandler {
    fn default() -> Self {
        Self::new()
    }
}