//! Protocol versioning.

use crate::Error;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Canonical protocol family name.
pub const PROTOCOL_NAME: &str = "doldskrift";

/// Current stable Open protocol major version (DSK/1).
pub const PROTOCOL_VERSION: u8 = 1;

/// Protected-profile protocol major (DSK/3 sibling stubs; no AEAD yet).
pub const PROTOCOL_VERSION_PROTECTED: u8 = 3;

/// Protocol version wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProtocolVersion(pub u8);

impl ProtocolVersion {
    /// DOLDSKRIFT/1 (Open).
    pub const V1: Self = Self(1);
    /// DOLDSKRIFT/3 Protected stub containers.
    pub const V3: Self = Self(3);

    /// Returns true if this implementation can parse the version.
    pub const fn is_supported(self) -> bool {
        self.0 == PROTOCOL_VERSION || self.0 == PROTOCOL_VERSION_PROTECTED
    }

    /// Validate or return [`Error::UnsupportedVersion`].
    pub fn check_supported(self) -> Result<(), Error> {
        if self.is_supported() {
            Ok(())
        } else {
            Err(Error::UnsupportedVersion(self.0))
        }
    }
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        Self::V1
    }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DSK/{}", self.0)
    }
}
