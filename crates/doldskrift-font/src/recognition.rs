//! Structural glyph recognition with confidence (no ML).

use crate::fingerprint::GlyphFingerprint;
use crate::geometry::GlyphGeometry;
use crate::zones::{ZoneMap, ZonePrimitive};
use serde::{Deserialize, Serialize};

/// Recognition hypothesis for a single glyph observation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecognitionResult {
    /// Best-matching symbol codepoint.
    pub symbol: u32,
    /// Confidence in \[0, 1\].
    pub confidence: f64,
    /// Alternative (symbol, confidence) pairs, best-first after the winner.
    pub alternatives: Vec<(u32, f64)>,
}

/// Classify an observed fingerprint against a catalog of glyphs.
pub fn classify_fingerprint(
    observed: &GlyphFingerprint,
    catalog: &[GlyphGeometry],
) -> Option<RecognitionResult> {
    if catalog.is_empty() {
        return None;
    }
    let mut scored: Vec<(u32, f64)> = catalog
        .iter()
        .map(|g| {
            let dist = observed.distance(&g.fingerprint);
            let conf = (1.0 - (dist / 8.0).clamp(0.0, 1.0)).clamp(0.0, 1.0);
            (g.codepoint, conf)
        })
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let (symbol, confidence) = scored[0];
    let alternatives = scored
        .into_iter()
        .skip(1)
        .take(5)
        .filter(|(_, c)| *c > 0.001)
        .collect();
    Some(RecognitionResult {
        symbol,
        confidence,
        alternatives,
    })
}

/// Classify using full glyph geometry (uses fingerprint + light topology).
pub fn classify_glyph(
    observed: &GlyphGeometry,
    catalog: &[GlyphGeometry],
) -> Option<RecognitionResult> {
    classify_fingerprint(&observed.fingerprint, catalog)
}

/// Build a fingerprint from raw zone codes (vision pipeline).
pub fn fingerprint_from_zone_codes(
    n: u8,
    e: u8,
    s: u8,
    w: u8,
    c: u8,
    orientation: u8,
    redundancy: u8,
) -> GlyphFingerprint {
    let zones = ZoneMap {
        n: ZonePrimitive::from_u8(n),
        e: ZonePrimitive::from_u8(e),
        s: ZonePrimitive::from_u8(s),
        w: ZonePrimitive::from_u8(w),
        c: ZonePrimitive::from_u8(c),
    };
    let integrity = crate::fingerprint::integrity_marker((zones.pack() & 0xFF) as u8, zones);
    GlyphFingerprint::new(zones, redundancy, integrity, orientation)
}
