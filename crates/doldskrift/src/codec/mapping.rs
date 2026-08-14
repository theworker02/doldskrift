//! Byte↔symbol mappings.

use crate::{
    byte_to_symbol, checksum_bytes, symbol_to_byte, Error, Result, ALPHABET_SIZE, PROTOCOL_VERSION,
    PUA_BASE,
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stable identifier for a mapping table.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MappingId(pub String);

impl MappingId {
    /// Fixed mapping id for DOLDSKRIFT/1 encoded mode.
    pub fn static_v1() -> Self {
        Self(format!("static-v{PROTOCOL_VERSION}"))
    }

    /// Session mapping id derived from seed checksum.
    pub fn from_seed(seed: &[u8]) -> Self {
        let c = checksum_bytes(seed);
        Self(format!("session-v{PROTOCOL_VERSION}-{c:08x}"))
    }
}

impl fmt::Display for MappingId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A reversible 256-entry byte permutation used for encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mapping {
    /// `encode_table[byte] = symbol_offset` (0..255), where symbol = PUA_BASE + offset.
    encode_table: [u8; ALPHABET_SIZE],
    /// Inverse: `decode_table[symbol_offset] = byte`.
    decode_table: [u8; ALPHABET_SIZE],
    id: MappingId,
}

impl Mapping {
    /// Identity mapping used by encoded mode (byte `b` → `U+E000 + b`).
    pub fn identity() -> Self {
        let mut encode_table = [0u8; ALPHABET_SIZE];
        let mut decode_table = [0u8; ALPHABET_SIZE];
        for i in 0..ALPHABET_SIZE {
            encode_table[i] = i as u8;
            decode_table[i] = i as u8;
        }
        Self {
            encode_table,
            decode_table,
            id: MappingId::static_v1(),
        }
    }

    /// Build a mapping from an explicit encode table (must be a permutation).
    pub fn from_encode_table(encode_table: [u8; ALPHABET_SIZE], id: MappingId) -> Result<Self> {
        let mut decode_table = [0u8; ALPHABET_SIZE];
        let mut seen = [false; ALPHABET_SIZE];
        for (byte, &sym) in encode_table.iter().enumerate() {
            if seen[sym as usize] {
                return Err(Error::MappingCollision(format!(
                    "symbol offset {sym} assigned twice"
                )));
            }
            seen[sym as usize] = true;
            decode_table[sym as usize] = byte as u8;
        }
        if seen.iter().any(|&s| !s) {
            return Err(Error::InvalidMapping(
                "encode table is not a complete permutation".into(),
            ));
        }
        Ok(Self {
            encode_table,
            decode_table,
            id,
        })
    }

    /// Deterministically generate a permutation from `seed` using SplitMix64.
    ///
    /// This is **not** a cryptographic KDF. It exists for reproducible
    /// obfuscation / representation only.
    pub fn from_seed(seed: &[u8]) -> Result<Self> {
        let id = MappingId::from_seed(seed);
        let mut state = mix_seed(seed);
        let mut encode_table = [0u8; ALPHABET_SIZE];
        for (i, slot) in encode_table.iter_mut().enumerate() {
            *slot = i as u8;
        }
        // Fisher–Yates with SplitMix64
        for i in (1..ALPHABET_SIZE).rev() {
            state = splitmix64(state);
            let j = (state as usize) % (i + 1);
            encode_table.swap(i, j);
        }
        Self::from_encode_table(encode_table, id)
    }

    /// Mapping identifier.
    pub fn id(&self) -> &MappingId {
        &self.id
    }

    /// Encode a single byte to a PUA character.
    #[inline]
    pub fn encode_byte(&self, byte: u8) -> char {
        let offset = self.encode_table[byte as usize];
        char::from_u32(PUA_BASE + u32::from(offset)).expect("PUA symbol")
    }

    /// Decode a single PUA character to a byte.
    pub fn decode_char(&self, ch: char) -> Result<u8> {
        let offset = symbol_to_byte(ch).ok_or(Error::InvalidCodepoint(ch as u32))? as usize;
        Ok(self.decode_table[offset])
    }

    /// Encode UTF-8 text to a string of PUA symbols.
    pub fn encode_str(&self, text: &str) -> String {
        let bytes = text.as_bytes();
        let mut out = String::with_capacity(bytes.len() * 3); // PUA BMP → 3 UTF-8 bytes
        for &b in bytes {
            out.push(self.encode_byte(b));
        }
        out
    }

    /// Decode a string of PUA symbols back to UTF-8 text.
    pub fn decode_str(&self, encoded: &str) -> Result<String> {
        let mut bytes = Vec::with_capacity(encoded.chars().count());
        for ch in encoded.chars() {
            bytes.push(self.decode_char(ch)?);
        }
        String::from_utf8(bytes).map_err(Error::from)
    }

    /// Borrow the encode table (for manifests / debugging).
    pub fn encode_table(&self) -> &[u8; ALPHABET_SIZE] {
        &self.encode_table
    }
}

/// Alias documenting the fixed encoded-mode mapping.
pub type StaticMapping = Mapping;

/// Mix arbitrary seed bytes into a u64.
fn mix_seed(seed: &[u8]) -> u64 {
    let mut state = 0x243f_6a88_85a3_08d3_u64; // fractional π bits
    if seed.is_empty() {
        return splitmix64(state);
    }
    for chunk in seed.chunks(8) {
        let mut buf = [0u8; 8];
        buf[..chunk.len()].copy_from_slice(chunk);
        state ^= u64::from_le_bytes(buf);
        state = splitmix64(state);
    }
    state
}

/// SplitMix64 — deterministic, non-cryptographic PRNG step.
#[inline]
fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Helper used by tests / font tooling: identity symbol for a byte.
#[inline]
#[must_use]
pub fn identity_symbol(byte: u8) -> char {
    byte_to_symbol(byte)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_roundtrip() {
        let m = Mapping::identity();
        let text = "Hello, 世界 👋";
        assert_eq!(m.decode_str(&m.encode_str(text)).unwrap(), text);
    }

    #[test]
    fn seed_deterministic() {
        let a = Mapping::from_seed(b"agent-alpha").unwrap();
        let b = Mapping::from_seed(b"agent-alpha").unwrap();
        assert_eq!(a, b);
        assert_ne!(a.encode_table(), Mapping::identity().encode_table());
    }

    #[test]
    fn seed_roundtrip() {
        let m = Mapping::from_seed(b"session-001").unwrap();
        let text = "deploy ready ✓";
        assert_eq!(m.decode_str(&m.encode_str(text)).unwrap(), text);
    }
}
