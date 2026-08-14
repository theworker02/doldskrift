//! Glyph-facing helpers that do not pull in the font engine.
//!
//! Full geometry generation lives in `doldskrift-font`.

use crate::{is_project_mark, symbol_id_of, DSK_PROJECT_MARK, PUA_BASE};

/// Classify a codepoint into a coarse glyph role.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlyphRole {
    /// Byte alphabet symbol.
    Alphabet,
    /// Control / framing symbol.
    Control,
    /// Official project mark.
    ProjectMark,
    /// Unknown / outside Doldskrift ranges.
    Other,
}

/// Role for a Unicode codepoint.
pub fn glyph_role(codepoint: u32) -> GlyphRole {
    if is_project_mark(codepoint) {
        GlyphRole::ProjectMark
    } else if symbol_id_of(codepoint).is_some() {
        GlyphRole::Alphabet
    } else if (0xE100..0xE110).contains(&codepoint) {
        GlyphRole::Control
    } else {
        GlyphRole::Other
    }
}

/// Codepoint of alphabet index `i`.
#[inline]
pub fn alphabet_codepoint(i: u8) -> u32 {
    PUA_BASE + u32::from(i)
}

/// Project mark codepoint.
#[inline]
pub fn project_mark() -> u32 {
    DSK_PROJECT_MARK
}
