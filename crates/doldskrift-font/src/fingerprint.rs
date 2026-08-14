//! Glyph structural fingerprints (DSKG-1).

use crate::zones::{ZoneMap, ZonePrimitive};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Structural fingerprint format version label.
pub const FINGERPRINT_FORMAT: &str = "DSKG-1";

/// Compact structural fingerprint for deterministic recognition.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GlyphFingerprint {
    /// Format tag (`DSKG-1`).
    pub format: String,
    /// Zone occupancy.
    pub zones: ZoneMap,
    /// Redundancy level (GRE/1).
    pub redundancy: u8,
    /// Integrity / parity nibble (0..=15).
    pub integrity: u8,
    /// Orientation bucket 0..7.
    pub orientation: u8,
}

impl GlyphFingerprint {
    /// Build from zones + metadata.
    pub fn new(zones: ZoneMap, redundancy: u8, integrity: u8, orientation: u8) -> Self {
        Self {
            format: FINGERPRINT_FORMAT.into(),
            zones,
            redundancy,
            integrity: integrity & 0x0F,
            orientation: orientation & 0x07,
        }
    }

    /// Canonical string: `DSKG-1:N2-E0-S1-W3-C4-R1-I9-O3`
    pub fn as_str(&self) -> String {
        format!(
            "{format}:N{n}-E{e}-S{s}-W{w}-C{c}-R{r}-I{i}-O{o}",
            format = self.format,
            n = self.zones.n.as_u8(),
            e = self.zones.e.as_u8(),
            s = self.zones.s.as_u8(),
            w = self.zones.w.as_u8(),
            c = self.zones.c.as_u8(),
            r = self.redundancy,
            i = self.integrity,
            o = self.orientation,
        )
    }

    /// Parse a fingerprint string.
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        let rest = s.strip_prefix("DSKG-1:")?;
        let mut n = 0u8;
        let mut e = 0u8;
        let mut south = 0u8;
        let mut w = 0u8;
        let mut c = 0u8;
        let mut r = 1u8;
        let mut i = 0u8;
        let mut o = 0u8;
        for part in rest.split('-') {
            let (k, v) = part.split_at(1);
            let val: u8 = v.parse().ok()?;
            match k {
                "N" => n = val,
                "E" => e = val,
                "S" => south = val,
                "W" => w = val,
                "C" => c = val,
                "R" => r = val,
                "I" => i = val,
                "O" => o = val,
                _ => return None,
            }
        }
        Some(Self::new(
            ZoneMap {
                n: ZonePrimitive::from_u8(n),
                e: ZonePrimitive::from_u8(e),
                s: ZonePrimitive::from_u8(south),
                w: ZonePrimitive::from_u8(w),
                c: ZonePrimitive::from_u8(c),
            },
            r,
            i,
            o,
        ))
    }

    /// Structural distance to another fingerprint (lower = more similar).
    pub fn distance(&self, other: &Self) -> f64 {
        let zone_d = self.zones.hamming(other.zones) as f64;
        let red_d = (self.redundancy as i32 - other.redundancy as i32).unsigned_abs() as f64 * 0.5;
        let int_d = (self.integrity as i32 - other.integrity as i32).unsigned_abs() as f64 * 0.35;
        let ori_d = {
            let a = self.orientation as i32;
            let b = other.orientation as i32;
            let d = (a - b).unsigned_abs().min(8 - (a - b).unsigned_abs());
            d as f64 * 0.4
        };
        zone_d + red_d + int_d + ori_d
    }
}

impl fmt::Display for GlyphFingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_str())
    }
}

/// Compute a 4-bit integrity marker from symbol id + zone pack (lightweight parity).
pub fn integrity_marker(symbol_id: u8, zones: ZoneMap) -> u8 {
    let packed = zones.pack();
    let mut x = u32::from(symbol_id) ^ u32::from(packed) ^ (u32::from(packed) << 7);
    x ^= x >> 4;
    x ^= x >> 2;
    (x & 0x0F) as u8
}

/// Verify integrity marker.
pub fn verify_integrity(symbol_id: u8, zones: ZoneMap, marker: u8) -> bool {
    integrity_marker(symbol_id, zones) == (marker & 0x0F)
}
