//! Human-readability and machine-separability heuristics.

use crate::geometry::{GlyphGeometry, Primitive};

/// Aggregate metrics for a glyph.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlyphMetrics {
    /// Higher → more Latin-like / familiar (bad for Doldskrift).
    pub human_readability: f64,
    /// Higher → easier to distinguish by structure (good).
    pub machine_separability: f64,
    /// Stroke density 0..1.
    pub density: f64,
}

/// Heuristic: how much the glyph resembles simple Latin letterforms.
pub fn human_readability_score(glyph: &GlyphGeometry) -> f64 {
    if glyph.is_project_mark() {
        return 0.2;
    }
    let mut score: f64 = 0.0;
    let strokes = glyph.primitives.len() as f64;
    if strokes <= 3.0 {
        score += 0.35;
    } else if strokes <= 5.0 {
        score += 0.15;
    }
    let has_enclosure = glyph
        .primitives
        .iter()
        .any(|p| matches!(p, Primitive::Enclosure { .. } | Primitive::Ring { .. }));
    if has_enclosure && strokes <= 4.0 {
        score += 0.25;
    }
    let lines = glyph
        .primitives
        .iter()
        .filter(|p| matches!(p, Primitive::Line { .. }))
        .count();
    if lines == 1 && !has_enclosure {
        score += 0.3;
    }
    if glyph.features.intersections > 0 && strokes <= 4.0 {
        score += 0.2;
    }
    score.clamp(0.0, 1.0)
}

/// Structural entropy / separability proxy in \[0, 1\].
pub fn machine_separability(glyph: &GlyphGeometry) -> f64 {
    let f = &glyph.features;
    let variety = (f.terminals as f64)
        + (f.intersections as f64) * 1.4
        + (f.enclosures as f64) * 1.2
        + (f.dots as f64) * 0.8
        + (f.density as f64) * 0.5
        + (f.redundancy as f64) * 0.6;
    (variety / 18.0).clamp(0.0, 1.0)
}

/// Compute aggregate metrics.
pub fn metrics_for(glyph: &GlyphGeometry) -> GlyphMetrics {
    GlyphMetrics {
        human_readability: human_readability_score(glyph),
        machine_separability: machine_separability(glyph),
        density: (glyph.features.density as f64) / 7.0,
    }
}
