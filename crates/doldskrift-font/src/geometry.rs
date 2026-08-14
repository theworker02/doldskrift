//! MGE/2 glyph geometry — zone-structured, self-describing machine glyphs.

use crate::density::{Density, FontFamily};
use crate::fingerprint::{integrity_marker, GlyphFingerprint};
use crate::zones::{zone_map_for_symbol, Zone, ZoneMap, ZonePrimitive};
use doldskrift::{is_project_mark, DSK_PROJECT_MARK, GLYPH_ENGINE_VERSION, PUA_BASE};
use serde::{Deserialize, Serialize};

/// Font design grid (em square).
pub const FONT_UNITS: i32 = 1000;

/// 2D point in font units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
}

impl Point {
    /// Create a point.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Structural primitives of the Doldskrift glyph language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Primitive {
    /// Straight stroke.
    Line { from: Point, to: Point },
    /// Circular arc.
    Arc {
        center: Point,
        radius: i32,
        start_deg: i32,
        sweep_deg: i32,
    },
    /// Junction marker.
    Junction { at: Point },
    /// Filled dot.
    Dot { at: Point, radius: i32 },
    /// Open ring.
    Ring { center: Point, radius: i32 },
    /// Branch (Y-like fork).
    Branch { origin: Point, a: Point, b: Point },
    /// Terminal cap.
    Terminal { at: Point, orientation_deg: i32 },
    /// Crossing of two strokes.
    Crossing {
        at: Point,
        arm: i32,
        rotation_deg: i32,
    },
    /// Enclosure.
    Enclosure {
        origin: Point,
        width: i32,
        height: i32,
    },
}

impl Primitive {
    /// True for marker-like primitives (dots / junctions) used in graph summaries.
    pub fn is_marker_like(&self) -> bool {
        matches!(
            self,
            Self::Dot { .. } | Self::Junction { .. } | Self::Terminal { .. }
        )
    }
}

/// Complete MGE/2 glyph geometry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlyphGeometry {
    /// Unicode codepoint.
    pub codepoint: u32,
    /// Font grammar version.
    pub font_version: u32,
    /// Glyph engine version (2 for MGE/2).
    pub glyph_engine: u32,
    /// Advance width.
    pub advance: i32,
    /// Ordered primitives.
    pub primitives: Vec<Primitive>,
    /// Legacy structural features.
    pub features: StructuralFeatures,
    /// Zone occupancy map.
    pub zones: ZoneMap,
    /// DSKG-1 fingerprint.
    pub fingerprint: GlyphFingerprint,
    /// Density used at generation.
    pub density: String,
    /// Font family.
    pub family: String,
}

impl GlyphGeometry {
    /// DSKG-1 fingerprint string.
    pub fn fingerprint(&self) -> String {
        self.fingerprint.as_str()
    }

    /// True if this is the official project mark glyph.
    pub fn is_project_mark(&self) -> bool {
        is_project_mark(self.codepoint)
    }
}

/// Deterministic structural features optimized for machine recognition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuralFeatures {
    /// Number of terminals.
    pub terminals: u8,
    /// Number of intersections / crossings.
    pub intersections: u8,
    /// Number of enclosed regions (approx).
    pub enclosures: u8,
    /// Dominant orientation bucket 0..7.
    pub orientation: u8,
    /// Dot count.
    pub dots: u8,
    /// Stroke density bucket 0..7.
    pub density: u8,
    /// Compact feature bitfield.
    pub bitfield: u32,
    /// GRE/1 redundancy level.
    pub redundancy: u8,
    /// Integrity parity nibble.
    pub integrity: u8,
}

/// Generation options for MGE/2.
#[derive(Debug, Clone)]
pub struct GenerateOptions {
    /// Font grammar version.
    pub font_version: u32,
    /// Seed.
    pub seed: u64,
    /// Density.
    pub density: Density,
    /// Family.
    pub family: FontFamily,
}

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            font_version: doldskrift::FONT_VERSION,
            seed: 1,
            density: Density::Normal,
            family: FontFamily::Text,
        }
    }
}

/// Generate raw MGE/2 glyph geometry.
pub fn generate_glyph_raw(codepoint: u32, font_version: u32, seed: u64) -> GlyphGeometry {
    generate_glyph_with(
        codepoint,
        &GenerateOptions {
            font_version,
            seed,
            ..GenerateOptions::default()
        },
    )
}

/// Generate with full options (density / family).
pub fn generate_glyph_with(codepoint: u32, opts: &GenerateOptions) -> GlyphGeometry {
    if is_project_mark(codepoint) {
        return project_mark_glyph(opts);
    }

    let symbol_id = if (PUA_BASE..PUA_BASE + 256).contains(&codepoint) {
        (codepoint - PUA_BASE) as u8
    } else {
        (codepoint & 0xFF) as u8
    };

    let redundancy = opts.density.redundancy_level();
    let zones = zone_map_for_symbol(symbol_id, redundancy);
    let orientation = (symbol_id.wrapping_mul(5) ^ (opts.seed as u8)) & 7;
    let integrity = integrity_marker(symbol_id, zones);
    let fingerprint = GlyphFingerprint::new(zones, redundancy, integrity, orientation);

    let mut primitives = Vec::new();
    let cx = FONT_UNITS / 2;
    let cy = FONT_UNITS / 2;
    let angle = orientation as i32 * 45;

    // Spine always present — machine baseline.
    let (dx, dy) = dir(angle);
    let spine = match opts.density {
        Density::Low => 220,
        Density::Normal => 300,
        Density::High => 360,
    };
    primitives.push(Primitive::Line {
        from: Point::new(cx - dx * spine / 200, cy - dy * spine / 200),
        to: Point::new(cx + dx * spine / 200, cy + dy * spine / 200),
    });

    // Place zone primitives — geometry encodes identity.
    for zone in Zone::ALL {
        place_zone_primitive(&mut primitives, zone, zones.get(zone), cx, cy, angle, opts);
    }

    // GRE/1 secondary marker: echo low nibble as integrity ticks near south-east.
    if redundancy >= 2 {
        let ticks = (integrity & 0b11) + 1;
        for t in 0..ticks {
            primitives.push(Primitive::Dot {
                at: Point::new(720 + t as i32 * 40, 220),
                radius: if opts.density == Density::Low { 22 } else { 14 },
            });
        }
    }
    if redundancy >= 3 {
        // Mirror primary center as a small ring (secondary channel).
        primitives.push(Primitive::Ring {
            center: Point::new(cx + 180, cy - 180),
            radius: 40,
        });
    }

    let terminals = primitives
        .iter()
        .filter(|p| matches!(p, Primitive::Terminal { .. }))
        .count() as u8;
    let intersections = primitives
        .iter()
        .filter(|p| matches!(p, Primitive::Crossing { .. }))
        .count() as u8;
    let enclosures = primitives
        .iter()
        .filter(|p| matches!(p, Primitive::Enclosure { .. } | Primitive::Ring { .. }))
        .count() as u8;
    let dots = primitives
        .iter()
        .filter(|p| matches!(p, Primitive::Dot { .. }))
        .count() as u8;
    let density_bucket = (primitives.len().min(15) as u8) & 7;

    let bitfield = (u32::from(terminals) & 0b111)
        | ((u32::from(intersections) & 0b111) << 3)
        | ((u32::from(dots) & 0b111) << 6)
        | ((u32::from(orientation) & 0b111) << 9)
        | ((u32::from(enclosures) & 0b111) << 12)
        | ((u32::from(integrity) & 0xF) << 16)
        | (u32::from(symbol_id) << 20);

    GlyphGeometry {
        codepoint,
        font_version: opts.font_version,
        glyph_engine: GLYPH_ENGINE_VERSION,
        advance: opts.family.advance(),
        primitives,
        features: StructuralFeatures {
            terminals,
            intersections,
            enclosures,
            orientation,
            dots,
            density: density_bucket,
            bitfield,
            redundancy,
            integrity,
        },
        zones,
        fingerprint,
        density: opts.density.as_str().into(),
        family: opts.family.as_str().into(),
    }
}

/// Official logo glyph — part of the writing system (`DSK_PROJECT_MARK`).
#[allow(clippy::vec_init_then_push)]
fn project_mark_glyph(opts: &GenerateOptions) -> GlyphGeometry {
    let zones = ZoneMap {
        n: ZonePrimitive::Terminal,
        e: ZonePrimitive::Arc,
        s: ZonePrimitive::Terminal,
        w: ZonePrimitive::Junction,
        c: ZonePrimitive::Branch,
    };
    let integrity = integrity_marker(0xF0, zones);
    let fingerprint = GlyphFingerprint::new(zones, 3, integrity, 2);
    let mut primitives = Vec::new();
    // Geometric D: spine + arc + internal branch + junctions (matches brand mark).
    primitives.push(Primitive::Line {
        from: Point::new(280, 220),
        to: Point::new(280, 780),
    });
    primitives.push(Primitive::Arc {
        center: Point::new(420, 500),
        radius: 260,
        start_deg: 270,
        sweep_deg: 180,
    });
    primitives.push(Primitive::Branch {
        origin: Point::new(420, 500),
        a: Point::new(560, 380),
        b: Point::new(560, 620),
    });
    primitives.push(Primitive::Junction {
        at: Point::new(280, 220),
    });
    primitives.push(Primitive::Junction {
        at: Point::new(280, 780),
    });
    primitives.push(Primitive::Terminal {
        at: Point::new(680, 500),
        orientation_deg: 0,
    });
    primitives.push(Primitive::Dot {
        at: Point::new(520, 500),
        radius: 18,
    });
    // GRE high redundancy markers
    primitives.push(Primitive::Dot {
        at: Point::new(760, 240),
        radius: 12,
    });
    primitives.push(Primitive::Dot {
        at: Point::new(800, 240),
        radius: 12,
    });

    GlyphGeometry {
        codepoint: DSK_PROJECT_MARK,
        font_version: opts.font_version,
        glyph_engine: GLYPH_ENGINE_VERSION,
        advance: opts.family.advance(),
        primitives,
        features: StructuralFeatures {
            terminals: 2,
            intersections: 0,
            enclosures: 0,
            orientation: 2,
            dots: 3,
            density: 7,
            bitfield: 0xF0_00_00 | u32::from(integrity) << 16,
            redundancy: 3,
            integrity,
        },
        zones,
        fingerprint,
        density: opts.density.as_str().into(),
        family: opts.family.as_str().into(),
    }
}

fn place_zone_primitive(
    out: &mut Vec<Primitive>,
    zone: Zone,
    prim: ZonePrimitive,
    cx: i32,
    cy: i32,
    angle: i32,
    opts: &GenerateOptions,
) {
    let (zx, zy) = match zone {
        Zone::N => (cx, cy + 260),
        Zone::E => (cx + 260, cy),
        Zone::S => (cx, cy - 260),
        Zone::W => (cx - 260, cy),
        Zone::C => (cx, cy),
    };
    let scale = match opts.density {
        Density::Low => 130,
        Density::Normal => 100,
        Density::High => 80,
    };
    match prim {
        ZonePrimitive::Empty => {}
        ZonePrimitive::Terminal => out.push(Primitive::Terminal {
            at: Point::new(zx, zy),
            orientation_deg: angle + zone_angle(zone),
        }),
        ZonePrimitive::Dot => out.push(Primitive::Dot {
            at: Point::new(zx, zy),
            radius: 20 * scale / 100,
        }),
        ZonePrimitive::Arc => out.push(Primitive::Arc {
            center: Point::new(zx, zy),
            radius: 70 * scale / 100,
            start_deg: angle,
            sweep_deg: 140,
        }),
        ZonePrimitive::Branch => {
            let (dx, dy) = dir(angle + 90);
            let arm = 90 * scale / 100;
            out.push(Primitive::Branch {
                origin: Point::new(zx, zy),
                a: Point::new(zx + dx * arm / 100, zy + dy * arm / 100),
                b: Point::new(zx - dx * arm / 100, zy - dy * arm / 100),
            });
        }
        ZonePrimitive::Ring => out.push(Primitive::Ring {
            center: Point::new(zx, zy),
            radius: 55 * scale / 100,
        }),
        ZonePrimitive::Junction => out.push(Primitive::Junction {
            at: Point::new(zx, zy),
        }),
        ZonePrimitive::Cross => out.push(Primitive::Crossing {
            at: Point::new(zx, zy),
            arm: 70 * scale / 100,
            rotation_deg: angle + 20,
        }),
    }
}

fn zone_angle(zone: Zone) -> i32 {
    match zone {
        Zone::N => 90,
        Zone::E => 0,
        Zone::S => 270,
        Zone::W => 180,
        Zone::C => 45,
    }
}

fn dir(deg: i32) -> (i32, i32) {
    let d = deg.rem_euclid(360);
    match d {
        0 => (100, 0),
        45 => (70, 70),
        90 => (0, 100),
        135 => (-70, 70),
        180 => (-100, 0),
        225 => (-70, -70),
        270 => (0, -100),
        315 => (70, -70),
        _ => dir((d / 45) * 45),
    }
}
