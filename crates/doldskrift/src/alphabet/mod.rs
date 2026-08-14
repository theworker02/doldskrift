//! Byte alphabet helpers for DOLDSKRIFT/1 PUA symbols.

use crate::{byte_to_symbol, symbol_to_byte, ALPHABET_SIZE, PUA_BASE};

/// Iterate all 256 alphabet codepoints.
pub fn all_codepoints() -> impl Iterator<Item = u32> {
    (0..ALPHABET_SIZE as u32).map(|i| PUA_BASE + i)
}

/// Iterate all 256 alphabet chars.
pub fn all_symbols() -> impl Iterator<Item = char> {
    (0..=255u8).map(byte_to_symbol)
}

/// Map a byte to its alphabet index (identity for DOLDSKRIFT/1).
#[inline]
pub fn index_of_byte(byte: u8) -> u8 {
    byte
}

/// Map a symbol char back to alphabet index.
#[inline]
pub fn index_of_symbol(ch: char) -> Option<u8> {
    symbol_to_byte(ch)
}
