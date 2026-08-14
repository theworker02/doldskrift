//! Experimental machine-optimized alphabet (feature-gated research).
//!
//! Regions of a glyph can encode bits via structural features:
//! top terminal, right branch, bottom dot, left arc, center crossing, rotation.
//! Kept behind the `experimental-machine-alphabet` idea until robust.
//! This module documents the bit layout used by [`crate::geometry::StructuralFeatures::bitfield`].

/// Bit layout inside the experimental machine bitfield (low 16 bits).
pub mod bits {
    /// Terminals in bits 0..2.
    pub const TERMINALS: u32 = 0b111;
    /// Intersections in bits 3..5.
    pub const INTERSECTIONS: u32 = 0b111 << 3;
    /// Dots in bits 6..8.
    pub const DOTS: u32 = 0b111 << 6;
    /// Orientation bucket in bits 9..11.
    pub const ORIENTATION: u32 = 0b111 << 9;
    /// Enclosures in bits 12..14.
    pub const ENCLOSURES: u32 = 0b111 << 12;
}

/// Extract machine-readable feature bits from a bitfield.
pub fn decode_features(bitfield: u32) -> (u8, u8, u8, u8, u8) {
    (
        (bitfield & bits::TERMINALS) as u8,
        ((bitfield & bits::INTERSECTIONS) >> 3) as u8,
        ((bitfield & bits::DOTS) >> 6) as u8,
        ((bitfield & bits::ORIENTATION) >> 9) as u8,
        ((bitfield & bits::ENCLOSURES) >> 12) as u8,
    )
}
