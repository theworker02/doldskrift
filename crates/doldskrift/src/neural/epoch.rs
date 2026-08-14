//! LSG versioning and reader compatibility.

use serde::{Deserialize, Serialize};
use std::fmt;

/// LSG major version label.
pub const LSG_VERSION: u16 = 1;

/// Human-readable LSG version.
pub const LSG_VERSION_LABEL: &str = "LSG/1";

/// Grammar epoch within LSG/1 (`LSG/1-E0001` style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GrammarEpoch {
    /// Epoch number (1-based display as `E0001`).
    pub number: u16,
}

impl GrammarEpoch {
    /// First foundation epoch.
    pub const E0001: Self = Self { number: 1 };

    /// Format as `LSG/1-E0001`.
    pub fn label(&self) -> String {
        format!("LSG/{}-E{:04}", LSG_VERSION, self.number)
    }

    /// Parse `LSG/1-E0001` or bare `E0001`.
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        let epoch_part = if let Some(rest) = s.strip_prefix("LSG/") {
            let mut parts = rest.split('-');
            let ver = parts.next()?;
            if ver != "1" && ver != LSG_VERSION.to_string() {
                // Only LSG/1 in foundation.
                if ver.parse::<u16>().ok()? != LSG_VERSION {
                    return None;
                }
            }
            parts.next()?
        } else {
            s
        };
        let num = epoch_part
            .strip_prefix('E')
            .or_else(|| epoch_part.strip_prefix('e'))?;
        let number = num.parse().ok()?;
        Some(Self { number })
    }
}

impl fmt::Display for GrammarEpoch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

impl Default for GrammarEpoch {
    fn default() -> Self {
        Self::E0001
    }
}

/// Declares which grammar epochs a reader can reconstruct.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReaderCompatibility {
    /// Reader identifier.
    pub reader_id: String,
    /// Supported epochs (exact match required; no silent upgrade).
    pub supported_epochs: Vec<GrammarEpoch>,
}

impl ReaderCompatibility {
    /// Whether `epoch` is supported.
    pub fn supports(&self, epoch: &GrammarEpoch) -> bool {
        self.supported_epochs.iter().any(|e| e == epoch)
    }
}
