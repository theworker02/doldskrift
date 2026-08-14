//! Lightweight error-correction helpers for visual frames.
//!
//! This is a pragmatic repetition/parity layer — not a full RS codec.

use crate::{checksum_bytes, Error, Result};
use serde::{Deserialize, Serialize};

/// ECC protection level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum EccLevel {
    /// No redundancy.
    #[default]
    None = 0,
    /// Duplicate payload once (2×).
    Low = 1,
    /// Triple repetition + parity byte.
    Medium = 2,
    /// Triple repetition + CRC32C trailer.
    High = 3,
}

impl EccLevel {
    /// Wire discriminant.
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Parse wire discriminant.
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::None),
            1 => Some(Self::Low),
            2 => Some(Self::Medium),
            3 => Some(Self::High),
            _ => None,
        }
    }
}

/// Report from an ECC decode attempt.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RecoveryReport {
    /// Whether recovery changed any bytes vs the first copy.
    pub corrected: bool,
    /// Number of copy disagreements observed.
    pub disagreements: usize,
    /// Human-readable note.
    pub note: String,
}

/// Protect `payload` according to `level`.
pub fn encode(payload: &[u8], level: EccLevel) -> Result<Vec<u8>> {
    match level {
        EccLevel::None => Ok(payload.to_vec()),
        EccLevel::Low => {
            let mut out = Vec::with_capacity(payload.len() * 2);
            out.extend_from_slice(payload);
            out.extend_from_slice(payload);
            Ok(out)
        }
        EccLevel::Medium => {
            let mut out = Vec::with_capacity(payload.len() * 3 + 1);
            out.extend_from_slice(payload);
            out.extend_from_slice(payload);
            out.extend_from_slice(payload);
            let parity = payload.iter().fold(0u8, |a, b| a ^ b);
            out.push(parity);
            Ok(out)
        }
        EccLevel::High => {
            let mut out = Vec::with_capacity(payload.len() * 3 + 4);
            out.extend_from_slice(payload);
            out.extend_from_slice(payload);
            out.extend_from_slice(payload);
            out.extend_from_slice(&checksum_bytes(payload).to_be_bytes());
            Ok(out)
        }
    }
}

/// Recover payload bytes and a recovery report.
pub fn decode(protected: &[u8], level: EccLevel) -> Result<(Vec<u8>, RecoveryReport)> {
    match level {
        EccLevel::None => Ok((
            protected.to_vec(),
            RecoveryReport {
                note: "no ecc".into(),
                ..Default::default()
            },
        )),
        EccLevel::Low => {
            if protected.len() % 2 != 0 {
                return Err(Error::Ecc("low ecc length not even".into()));
            }
            let n = protected.len() / 2;
            let a = &protected[..n];
            let b = &protected[n..];
            let (out, disagreements) = majority2(a, b);
            Ok((
                out,
                RecoveryReport {
                    corrected: disagreements > 0,
                    disagreements,
                    note: "low ecc".into(),
                },
            ))
        }
        EccLevel::Medium => {
            if protected.is_empty() {
                return Err(Error::Ecc("medium ecc empty".into()));
            }
            let body = &protected[..protected.len() - 1];
            if body.len() % 3 != 0 {
                return Err(Error::Ecc("medium ecc body not divisible by 3".into()));
            }
            let n = body.len() / 3;
            let (out, disagreements) = majority3(&body[..n], &body[n..2 * n], &body[2 * n..]);
            let parity = protected[protected.len() - 1];
            let expected = out.iter().fold(0u8, |a, b| a ^ b);
            if parity != expected {
                return Err(Error::Ecc("parity mismatch".into()));
            }
            Ok((
                out,
                RecoveryReport {
                    corrected: disagreements > 0,
                    disagreements,
                    note: "medium ecc".into(),
                },
            ))
        }
        EccLevel::High => {
            if protected.len() < 4 {
                return Err(Error::Ecc("high ecc truncated".into()));
            }
            let body = &protected[..protected.len() - 4];
            if body.len() % 3 != 0 {
                return Err(Error::Ecc("high ecc body not divisible by 3".into()));
            }
            let n = body.len() / 3;
            let (out, disagreements) = majority3(&body[..n], &body[n..2 * n], &body[2 * n..]);
            let expected = u32::from_be_bytes(protected[protected.len() - 4..].try_into().unwrap());
            let actual = checksum_bytes(&out);
            if expected != actual {
                return Err(Error::ChecksumMismatch { expected, actual });
            }
            Ok((
                out,
                RecoveryReport {
                    corrected: disagreements > 0,
                    disagreements,
                    note: "high ecc".into(),
                },
            ))
        }
    }
}

fn majority2(a: &[u8], b: &[u8]) -> (Vec<u8>, usize) {
    let mut out = a.to_vec();
    let mut disagreements = 0usize;
    for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
        if x != y {
            disagreements += 1;
            // Prefer first copy; caller may re-try with stronger ECC.
            out[i] = *x;
        }
    }
    (out, disagreements)
}

fn majority3(a: &[u8], b: &[u8], c: &[u8]) -> (Vec<u8>, usize) {
    let mut out = Vec::with_capacity(a.len());
    let mut disagreements = 0usize;
    for i in 0..a.len() {
        let x = a[i];
        let y = b[i];
        let z = c[i];
        let v = if x == y || x == z {
            x
        } else if y == z {
            y
        } else {
            disagreements += 1;
            x
        };
        if x != y || y != z {
            disagreements += 1;
        }
        out.push(v);
    }
    (out, disagreements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_high() {
        let payload = b"agent-payload";
        let enc = encode(payload, EccLevel::High).unwrap();
        let (dec, _) = decode(&enc, EccLevel::High).unwrap();
        assert_eq!(dec, payload);
    }
}
