//! Glyph structural distance and collision prevention.

use crate::fingerprint::GlyphFingerprint;
use crate::geometry::GlyphGeometry;

/// Minimum structural distance between distinct alphabet glyphs.
pub const MINIMUM_DISTANCE: f64 = 1.5;

/// Structural similarity threshold (legacy alias; lower distance = farther apart).
pub const SIMILARITY_THRESHOLD: f64 = 0.82;

/// Compute structural distance between two glyphs (0 = identical structure).
pub fn glyph_distance(a: &GlyphGeometry, b: &GlyphGeometry) -> f64 {
    let mut d = a.fingerprint.distance(&b.fingerprint);
    // Topology extras
    d += (a.primitives.len() as i32 - b.primitives.len() as i32).unsigned_abs() as f64 * 0.15;
    d += (a.features.terminals as i32 - b.features.terminals as i32).unsigned_abs() as f64 * 0.25;
    d += (a.features.intersections as i32 - b.features.intersections as i32).unsigned_abs() as f64
        * 0.3;
    d += (a.features.dots as i32 - b.features.dots as i32).unsigned_abs() as f64 * 0.2;
    d
}

/// Legacy similarity in \[0,1\] (1 = nearly identical) derived from distance.
pub fn similarity(a: &GlyphGeometry, b: &GlyphGeometry) -> f64 {
    let d = glyph_distance(a, b);
    (1.0 - (d / 12.0).clamp(0.0, 1.0)).clamp(0.0, 1.0)
}

/// Fingerprint-only distance.
pub fn fingerprint_distance(a: &GlyphFingerprint, b: &GlyphFingerprint) -> f64 {
    a.distance(b)
}

/// Returns true if `candidate` is far enough from all `existing` glyphs.
pub fn satisfies_minimum_distance(candidate: &GlyphGeometry, existing: &[GlyphGeometry]) -> bool {
    existing
        .iter()
        .all(|g| glyph_distance(g, candidate) >= MINIMUM_DISTANCE)
}
