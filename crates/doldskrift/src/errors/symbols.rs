//! Reserved control / project symbols (outside the 256-byte alphabet).

use super::PUA_BASE;

/// Control-symbol base: `U+E100`.
pub const CONTROL_BASE: u32 = 0xE100;

/// Token / framing control symbols (experimental semantic channel).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ControlSymbol {
    /// Start of structured span.
    Start = 0,
    /// End of structured span.
    End = 1,
    /// Token boundary.
    Token = 2,
    /// Soft break / sentence boundary.
    Break = 3,
    /// Metadata marker.
    Meta = 4,
    /// Escape next symbol.
    Escape = 5,
}

impl ControlSymbol {
    /// Canonical name (`DSK_START`, …).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "DSK_START",
            Self::End => "DSK_END",
            Self::Token => "DSK_TOKEN",
            Self::Break => "DSK_BREAK",
            Self::Meta => "DSK_META",
            Self::Escape => "DSK_ESCAPE",
        }
    }

    /// Codepoint.
    pub const fn codepoint(self) -> u32 {
        CONTROL_BASE + self as u32
    }

    /// As `char`.
    pub fn as_char(self) -> char {
        char::from_u32(self.codepoint()).expect("control symbol")
    }
}

/// Official project mark — the logo *is* this glyph.
///
/// Codepoint `U+E1F0`. Recognized by MGE/2 as `DSK_PROJECT_MARK`.
pub const DSK_PROJECT_MARK: u32 = 0xE1F0;

/// Name of the project mark symbol.
pub const DSK_PROJECT_MARK_NAME: &str = "DSK_PROJECT_MARK";

/// Returns true if `cp` is the reserved project logo glyph.
#[inline]
pub const fn is_project_mark(cp: u32) -> bool {
    cp == DSK_PROJECT_MARK
}

/// Returns true if `cp` is in the control-symbol range.
#[inline]
pub const fn is_control_symbol(cp: u32) -> bool {
    cp >= CONTROL_BASE && cp < CONTROL_BASE + 16
}

/// Glyph engine major version for MGE/2.
pub const GLYPH_ENGINE_VERSION: u32 = 2;

/// Font grammar version aligned with MGE/2.
pub const FONT_VERSION_MGE2: u32 = 2;

/// Offset of a payload alphabet symbol relative to [`PUA_BASE`].
#[inline]
pub const fn symbol_id_of(cp: u32) -> Option<u8> {
    if cp >= PUA_BASE && cp < PUA_BASE + 256 {
        Some((cp - PUA_BASE) as u8)
    } else {
        None
    }
}
