//! Operating modes and capability flags.

use crate::Error;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Doldskrift operating mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum Mode {
    /// Font substitution only; underlying Unicode is unchanged.
    Visual = 0,
    /// Reversible byte→PUA encoding with a fixed, versioned mapping.
    Encoded = 1,
    /// Seeded dynamic mapping; requires a session manifest.
    Session = 2,
    /// Protected visual object (DSK/3 sibling) — glyphs encode ciphertext; Gate required.
    ///
    /// Stub only: no AEAD yet. Semantic decode must refuse.
    Protected = 3,
}

impl Mode {
    /// Wire / container discriminant.
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Parse from wire discriminant.
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Visual),
            1 => Some(Self::Encoded),
            2 => Some(Self::Session),
            3 => Some(Self::Protected),
            _ => None,
        }
    }

    /// Canonical lowercase name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Encoded => "encoded",
            Self::Session => "session",
            Self::Protected => "protected",
        }
    }

    /// Capability flag associated with this mode.
    pub const fn capability(self) -> Capability {
        match self {
            Self::Visual => Capability::Visual,
            Self::Encoded => Capability::Encoded,
            Self::Session => Capability::Session,
            Self::Protected => Capability::Protected,
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Mode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "visual" | "v" => Ok(Self::Visual),
            "encoded" | "e" => Ok(Self::Encoded),
            "session" | "s" => Ok(Self::Session),
            "protected" | "p" => Ok(Self::Protected),
            other => Err(Error::Other(format!("unknown mode '{other}'"))),
        }
    }
}

/// Advertised implementation capabilities (capability negotiation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Capability {
    /// Visual (font-only) mode.
    Visual,
    /// Fixed encoded mode.
    Encoded,
    /// Session / seeded mapping mode.
    Session,
    /// Streaming encode/decode.
    Stream,
    /// `.dsk` container support.
    Container,
    /// Experimental vision / OCR pipeline.
    Vision,
    /// Protected / Gate surface (DSK/3 sibling; stub until AEAD ships).
    Protected,
}

impl Capability {
    /// Canonical token used on the wire (`DSK_VISUAL`, …).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "DSK_VISUAL",
            Self::Encoded => "DSK_ENCODED",
            Self::Session => "DSK_SESSION",
            Self::Stream => "DSK_STREAM",
            Self::Container => "DSK_CONTAINER",
            Self::Vision => "DSK_VISION",
            Self::Protected => "DSK_PROTECTED",
        }
    }

    /// Parse a capability token.
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_uppercase().as_str() {
            "DSK_VISUAL" | "VISUAL" => Some(Self::Visual),
            "DSK_ENCODED" | "ENCODED" => Some(Self::Encoded),
            "DSK_SESSION" | "SESSION" => Some(Self::Session),
            "DSK_STREAM" | "STREAM" => Some(Self::Stream),
            "DSK_CONTAINER" | "CONTAINER" => Some(Self::Container),
            "DSK_VISION" | "VISION" => Some(Self::Vision),
            "DSK_PROTECTED" | "PROTECTED" => Some(Self::Protected),
            _ => None,
        }
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
