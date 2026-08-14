//! MGE/2 zone anatomy (N/E/S/W/C).

use serde::{Deserialize, Serialize};
use std::fmt;

/// Cardinal / center zone of a glyph cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Zone {
    /// North.
    N,
    /// East.
    E,
    /// South.
    S,
    /// West.
    W,
    /// Center.
    C,
}

impl Zone {
    /// All zones in fingerprint order.
    pub const ALL: [Zone; 5] = [Zone::N, Zone::E, Zone::S, Zone::W, Zone::C];

    /// Single-letter label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::N => "N",
            Self::E => "E",
            Self::S => "S",
            Self::W => "W",
            Self::C => "C",
        }
    }
}

impl fmt::Display for Zone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Primitive kind occupying a zone (compact code 0..=7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ZonePrimitive {
    /// Empty / absent.
    Empty = 0,
    /// Terminal cap.
    Terminal = 1,
    /// Dot marker.
    Dot = 2,
    /// Arc segment.
    Arc = 3,
    /// Branch / fork.
    Branch = 4,
    /// Ring.
    Ring = 5,
    /// Junction.
    Junction = 6,
    /// Bar / cross / fork hybrid.
    Cross = 7,
}

impl ZonePrimitive {
    /// Parse from u8 (masks to 3 bits).
    pub const fn from_u8(v: u8) -> Self {
        match v & 0b111 {
            1 => Self::Terminal,
            2 => Self::Dot,
            3 => Self::Arc,
            4 => Self::Branch,
            5 => Self::Ring,
            6 => Self::Junction,
            7 => Self::Cross,
            _ => Self::Empty,
        }
    }

    /// Wire value.
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Short name for fingerprints.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "0",
            Self::Terminal => "T",
            Self::Dot => "D",
            Self::Arc => "A",
            Self::Branch => "B",
            Self::Ring => "R",
            Self::Junction => "J",
            Self::Cross => "X",
        }
    }
}

/// Occupancy of all five zones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ZoneMap {
    /// North.
    pub n: ZonePrimitive,
    /// East.
    pub e: ZonePrimitive,
    /// South.
    pub s: ZonePrimitive,
    /// West.
    pub w: ZonePrimitive,
    /// Center.
    pub c: ZonePrimitive,
}

impl ZoneMap {
    /// Pack into a 15-bit value (3 bits × 5).
    pub const fn pack(self) -> u16 {
        (self.n.as_u8() as u16)
            | ((self.e.as_u8() as u16) << 3)
            | ((self.s.as_u8() as u16) << 6)
            | ((self.w.as_u8() as u16) << 9)
            | ((self.c.as_u8() as u16) << 12)
    }

    /// Unpack from 15-bit value.
    pub const fn unpack(v: u16) -> Self {
        Self {
            n: ZonePrimitive::from_u8((v & 0b111) as u8),
            e: ZonePrimitive::from_u8(((v >> 3) & 0b111) as u8),
            s: ZonePrimitive::from_u8(((v >> 6) & 0b111) as u8),
            w: ZonePrimitive::from_u8(((v >> 9) & 0b111) as u8),
            c: ZonePrimitive::from_u8(((v >> 12) & 0b111) as u8),
        }
    }

    /// Hamming distance between zone maps.
    pub fn hamming(self, other: Self) -> u32 {
        (self.n != other.n) as u32
            + (self.e != other.e) as u32
            + (self.s != other.s) as u32
            + (self.w != other.w) as u32
            + (self.c != other.c) as u32
    }

    /// Get zone content.
    pub const fn get(self, zone: Zone) -> ZonePrimitive {
        match zone {
            Zone::N => self.n,
            Zone::E => self.e,
            Zone::S => self.s,
            Zone::W => self.w,
            Zone::C => self.c,
        }
    }
}

/// Derive a primary zone map that encodes `symbol_id` bits.
///
/// Bit allocation (symbol identity describing itself):
/// ```text
/// orientation-ish → N (2 bits via primitive choice)
/// terminal count  → E (low bits)
/// branch pattern  → S
/// center structure→ C
/// dot arrangement → W
/// ```
pub fn zone_map_for_symbol(symbol_id: u8, redundancy: u8) -> ZoneMap {
    let b0 = symbol_id & 0b11;
    let b1 = (symbol_id >> 2) & 0b111;
    let b2 = (symbol_id >> 5) & 0b111;
    let mut map = ZoneMap {
        n: ZonePrimitive::from_u8(1 + (b0 % 7)),
        e: ZonePrimitive::from_u8(1 + (b1 % 7)),
        s: ZonePrimitive::from_u8(1 + (b2 % 7)),
        w: ZonePrimitive::from_u8(1 + (((symbol_id.reverse_bits()) & 0b111) % 7)),
        c: ZonePrimitive::from_u8(1 + ((symbol_id.wrapping_mul(3) >> 2) & 0b111) % 7),
    };
    // GRE/1: echo low nibble into a secondary marker via forcing non-empty W/S.
    if redundancy >= 2 && map.w == ZonePrimitive::Empty {
        map.w = ZonePrimitive::Dot;
    }
    if redundancy >= 3 && map.s == ZonePrimitive::Empty {
        map.s = ZonePrimitive::Terminal;
    }
    map
}
