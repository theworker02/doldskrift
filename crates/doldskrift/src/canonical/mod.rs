//! Canonical form helpers for deterministic encoding.

use crate::semantic::{encode_value, Value};
use crate::Result;

/// Canonical encode a semantic value to bytes.
pub fn canonical_bytes(value: &Value) -> Result<Vec<u8>> {
    encode_value(value, true)
}

/// Compare two byte slices for canonical equality.
pub fn bytes_equal(a: &[u8], b: &[u8]) -> bool {
    a == b
}
