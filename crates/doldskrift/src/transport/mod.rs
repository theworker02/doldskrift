//! Logical transport framing (not a network stack).
//!
//! Provides length-prefixed message envelopes for agent pipes.

use crate::{checksum_bytes, verify_checksum, Error, Result, MAGIC};

/// Length-prefixed envelope: magic || u32 len || payload || u32 crc32c.
pub fn wrap(payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + 4 + payload.len() + 4);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    out.extend_from_slice(payload);
    out.extend_from_slice(&checksum_bytes(payload).to_be_bytes());
    out
}

/// Unwrap a [`wrap`] envelope.
pub fn unwrap(data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < 12 {
        return Err(Error::TruncatedPayload {
            needed: 12 - data.len(),
        });
    }
    if &data[0..4] != MAGIC.as_slice() {
        return Err(Error::InvalidHeader);
    }
    let len = u32::from_be_bytes(data[4..8].try_into().unwrap()) as usize;
    let end = 8 + len;
    if data.len() < end + 4 {
        return Err(Error::TruncatedPayload {
            needed: end + 4 - data.len(),
        });
    }
    let payload = &data[8..end];
    let expected = u32::from_be_bytes(data[end..end + 4].try_into().unwrap());
    verify_checksum(payload, expected)?;
    Ok(payload.to_vec())
}
