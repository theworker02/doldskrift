//! Adaptive glyph density levels.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Glyph structural density / redundancy tradeoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Density {
    /// Tiny displays — fewer primitives, larger strokes.
    Low,
    /// General use.
    #[default]
    Normal,
    /// Maximum structural redundancy (GRE/1 high).
    High,
}

impl Density {
    /// Canonical name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
        }
    }

    /// GRE/1 redundancy level encoded into glyphs.
    pub const fn redundancy_level(self) -> u8 {
        match self {
            Self::Low => 1,
            Self::Normal => 2,
            Self::High => 3,
        }
    }

    /// Relative stroke weight multiplier (×100).
    pub const fn stroke_weight(self) -> i32 {
        match self {
            Self::Low => 140,
            Self::Normal => 100,
            Self::High => 85,
        }
    }
}

impl fmt::Display for Density {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Density {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "low" | "l" | "micro" => Ok(Self::Low),
            "normal" | "n" | "default" => Ok(Self::Normal),
            "high" | "h" => Ok(Self::High),
            other => Err(format!("unknown density '{other}'")),
        }
    }
}

/// Font family variants solving technical problems (not stylistic fashion).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FontFamily {
    /// Default visual representation.
    #[default]
    Text,
    /// Fixed-width cells for terminals / code.
    Mono,
    /// Simplified glyphs for small rendering sizes.
    Micro,
}

impl FontFamily {
    /// Family name string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "DoldskriftText",
            Self::Mono => "DoldskriftMono",
            Self::Micro => "DoldskriftMicro",
        }
    }

    /// Default density for this family.
    pub const fn default_density(self) -> Density {
        match self {
            Self::Text => Density::Normal,
            Self::Mono => Density::Normal,
            Self::Micro => Density::Low,
        }
    }

    /// Advance width in font units.
    pub const fn advance(self) -> i32 {
        match self {
            Self::Text => 620,
            Self::Mono => 600,
            Self::Micro => 520,
        }
    }
}

impl fmt::Display for FontFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for FontFamily {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "text" | "doldskrifttext" | "regular" => Ok(Self::Text),
            "mono" | "doldskriftmono" => Ok(Self::Mono),
            "micro" | "doldskriftmicro" => Ok(Self::Micro),
            other => Err(format!("unknown font family '{other}'")),
        }
    }
}
