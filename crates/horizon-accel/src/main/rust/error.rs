//! Error types for the accelerator.

use thiserror::Error;

/// Result type for accelerator operations.
pub type AccelResult<T> = Result<T, AccelError>;

/// Errors that can occur in the accelerator.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AccelError {
    /// JNI call failed.
    #[error("JNI error: {0}")]
    Jni(String),

    /// Handshake protocol version mismatch.
    #[error("Protocol version mismatch: expected {expected}, got {actual}")]
    ProtocolMismatch { expected: u32, actual: u32 },

    /// Manifest parsing failed.
    #[error("Manifest parse error: {0}")]
    ManifestParse(#[from] serde_json::Error),

    /// Module not found.
    #[error("Module not found: {0}")]
    ModuleNotFound(String),

    /// Module already registered.
    #[error("Module already registered: {0}")]
    ModuleAlreadyRegistered(String),

    /// Module is in fallback state.
    #[error("Module '{0}' is in fallback: {1}")]
    ModuleInFallback(String, String),

    /// Conflict with another mod.
    #[error("Conflict: module '{module}' supersedes '{superseded}' but mod '{mod_id}' targets it")]
    Conflict {
        module: String,
        superseded: String,
        mod_id: String,
    },

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Native library not found or failed to load.
    #[error("Native library error: {0}")]
    NativeLibrary(String),

    /// Invalid configuration.
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Internal error.
    #[error("Internal error: {0}")]
    Internal(String),
}

impl AccelError {
    /// Convert to a negative error code for JNI return values.
    pub fn to_error_code(&self) -> i32 {
        match self {
            AccelError::Jni(_) => -100,
            AccelError::ProtocolMismatch { .. } => -101,
            AccelError::ManifestParse(_) => -102,
            AccelError::ModuleNotFound(_) => -103,
            AccelError::ModuleAlreadyRegistered(_) => -104,
            AccelError::ModuleInFallback(_, _) => -105,
            AccelError::Conflict { .. } => -106,
            AccelError::Io(_) => -107,
            AccelError::NativeLibrary(_) => -108,
            AccelError::InvalidConfig(_) => -109,
            AccelError::Internal(_) => -110,
        }
    }
}

/// Handshake error codes (match Java side).
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeError {
    /// Protocol version too old.
    VersionTooOld = -1,
    /// Protocol version too new (incompatible).
    VersionTooNew = -2,
    /// Native library missing required symbols.
    MissingSymbols = -3,
    /// Initialization failed.
    InitFailed = -4,
}

impl From<HandshakeError> for i32 {
    fn from(e: HandshakeError) -> i32 {
        e as i32
    }
}