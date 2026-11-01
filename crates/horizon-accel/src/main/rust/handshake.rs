//! Protocol handshake between Java mod and Rust library.

use crate::error::{AccelError, AccelResult, HandshakeError};

/// Current protocol version. Increment on breaking changes.
pub const ACCEL_PROTOCOL_VERSION: u32 = 1;

/// Minimum compatible protocol version.
pub const MIN_PROTOCOL_VERSION: u32 = 1;

/// Maximum compatible protocol version (inclusive).
pub const MAX_PROTOCOL_VERSION: u32 = 1;

/// Handshake result containing the agreed protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccelHandshake {
    pub protocol_version: u32,
    pub library_version: &'static str,
}

/// Perform the handshake with the Java side.
///
/// Returns the agreed protocol version on success, or an error code.
pub fn handshake(java_protocol_version: u32) -> AccelResult<u32> {
    if java_protocol_version < MIN_PROTOCOL_VERSION {
        return Err(AccelError::ProtocolMismatch {
            expected: MIN_PROTOCOL_VERSION,
            actual: java_protocol_version,
        });
    }
    if java_protocol_version > MAX_PROTOCOL_VERSION {
        return Err(AccelError::ProtocolMismatch {
            expected: MAX_PROTOCOL_VERSION,
            actual: java_protocol_version,
        });
    }

    // Version is compatible
    Ok(ACCEL_PROTOCOL_VERSION)
}

/// Library version string (injected at build time).
pub const LIBRARY_VERSION: &str = env!("CARGO_PKG_VERSION");

/// JNI_OnLoad expected return value.
pub const JNI_VERSION_1_8: i32 = 0x00010008;