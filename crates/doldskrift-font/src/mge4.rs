//! MGE/4 research: contextual variants, graph signatures, familiarity proxies.

use crate::geometry::{generate_glyph_with, GenerateOptions, GlyphGeometry};
use crate::metrics::human_readability_score;
use crate::{glyph_distance, Density, FontFamily};
use doldskrift::{Result, FONT_VERSION, PUA_BASE};
use serde::{Deserialize, Serialize};

/// Glyph engine major for MGE/4 features (experimental; MGE/2 geometry remains default).
pub const MGE4_ENGINE: u32 = 4;

/// Compact recognition signature (fast candidate elimination).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GlyphSignature(pub String);

impl GlyphSignature {
    /// Build from zone-ish topology codes embedded in geometry fingerprint.
    pub fn from_geometry(g: &GlyphGeometry) -> Self {
        let fp = g.fingerprint();
        // Compress fingerprint into a short token.
        let compact = fp
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .take(24)
            .collect::<String>();
        Self(format!("G4-{compact}"))
    }

    /// Prefix used for recognition index lookup.
    pub fn prefix(&self, n: usize) -> &str {
        let end = n.min(self.0.len());
        &self.0[..end]
    }
}

/// Canonical graph view of a glyph (nodes/edges derived from primitives count).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlyphGraph {
    /// Node count (junctions / terminals approximation).
    pub nodes: usize,
    /// Edge count (strokes approximation).
    pub edges: usize,
    /// Marker count (dots / integrity).
    pub markers: usize,
    /// Compact signature.
    pub signature: GlyphSignature,
    /// Symbol id when known.
    pub symbol_id: Option<u8>,
}

impl GlyphGraph {
    /// Derive a graph summary from geometry.
    pub fn from_geometry(g: &GlyphGeometry) -> Self {
        let prims = g.primitives.len();
        Self {
            nodes: (prims / 2).max(1),
            edges: prims.max(1),
            markers: g.primitives.iter().filter(|p| p.is_marker_like()).count(),
            signature: GlyphSignature::from_geometry(g),
            symbol_id: if g.codepoint >= PUA_BASE && g.codepoint < PUA_BASE + 256 {
                Some((g.codepoint - PUA_BASE) as u8)
            } else {
                None
            },
        }
    }
}

/// Contextual visual variant of a symbol (structurally equivalent for recognition).
#[derive(Debug, Clone)]
pub struct GlyphVariant {
    /// Variant index.
    pub index: u8,
    /// Geometry.
    pub geometry: GlyphGeometry,
    /// Graph summary.
    pub graph: GlyphGraph,
}

/// Deterministic variant selection (not cryptographic).
pub fn select_variant_index(
    symbol: u8,
    position: u32,
    frame_id: u64,
    seed: u64,
    variant_count: u8,
) -> u8 {
    if variant_count == 0 {
        return 0;
    }
    let mut h = seed
        ^ (u64::from(symbol) << 1)
        ^ (u64::from(position) << 17)
        ^ frame_id.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
    h ^= h >> 33;
    (h % u64::from(variant_count)) as u8
}

/// Generate contextual variants for a symbol id.
pub fn generate_variants(symbol: u8, seed: u64, count: u8) -> Result<Vec<GlyphVariant>> {
    let count = count.max(1);
    let mut out = Vec::with_capacity(count as usize);
    for i in 0..count {
        let opts = GenerateOptions {
            font_version: FONT_VERSION,
            seed: seed ^ (u64::from(i) << 24) ^ (u64::from(symbol) << 8) ^ 0x4D47_4534, // "MGE4"
            density: Density::Normal,
            family: FontFamily::Text,
        };
        let mut geometry = generate_glyph_with(PUA_BASE + u32::from(symbol), &opts);
        // Force contextual orientation diversity while preserving symbol identity.
        geometry.fingerprint.orientation = (geometry.fingerprint.orientation + i) & 7;
        let graph = GlyphGraph::from_geometry(&geometry);
        out.push(GlyphVariant {
            index: i,
            geometry,
            graph,
        });
    }
    Ok(out)
}

/// Heuristic: lower is more machine-oriented (less human-letter-like).
/// Uses existing human_readability_score as familiarity proxy (0..1).
pub fn familiarity_score(g: &GlyphGeometry) -> f64 {
    human_readability_score(g)
}

/// Repetition exposure: average pairwise similarity along a rendered sequence of geometries.
/// Lower is better for machine profiles (less obvious repeats).
pub fn repetition_exposure(glyphs: &[GlyphGeometry]) -> f64 {
    if glyphs.len() < 2 {
        return 0.0;
    }
    let mut sum = 0.0;
    let mut n = 0usize;
    for w in glyphs.windows(2) {
        let d = glyph_distance(&w[0], &w[1]);
        // Map distance to similarity-ish 0..1 (heuristic).
        sum += (1.0 / (1.0 + d)).clamp(0.0, 1.0);
        n += 1;
    }
    sum / n as f64
}

/// Boundary exposure proxy: fraction of near-identical adjacent pairs (suggests visible repeats).
pub fn boundary_exposure(glyphs: &[GlyphGeometry], threshold: f64) -> f64 {
    if glyphs.len() < 2 {
        return 0.0;
    }
    let mut hits = 0usize;
    for w in glyphs.windows(2) {
        if glyph_distance(&w[0], &w[1]) < threshold {
            hits += 1;
        }
    }
    hits as f64 / (glyphs.len() - 1) as f64
}

/// Recognition index: signature prefix → candidate symbol ids.
#[derive(Debug, Default, Clone)]
pub struct RecognitionIndex {
    map: std::collections::BTreeMap<String, Vec<u8>>,
}

impl RecognitionIndex {
    /// Build from alphabet geometries.
    pub fn from_alphabet(glyphs: &[GlyphGeometry], prefix_len: usize) -> Self {
        let mut map: std::collections::BTreeMap<String, Vec<u8>> =
            std::collections::BTreeMap::new();
        for g in glyphs {
            let sig = GlyphSignature::from_geometry(g);
            let key = sig.prefix(prefix_len).to_owned();
            if let Some(id) = GlyphGraph::from_geometry(g).symbol_id {
                map.entry(key).or_default().push(id);
            }
        }
        Self { map }
    }

    /// Lookup candidates.
    pub fn candidates(&self, signature: &GlyphSignature, prefix_len: usize) -> &[u8] {
        self.map
            .get(signature.prefix(prefix_len))
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_differ_visually_same_symbol() {
        let vars = generate_variants(42, 7, 4).unwrap();
        assert_eq!(vars.len(), 4);
        assert!(vars.iter().all(|v| v.graph.symbol_id == Some(42)));
        // Seeds differ → at least orientation or primitive layout should vary across variants.
        let orients: std::collections::BTreeSet<_> = vars
            .iter()
            .map(|v| v.geometry.fingerprint.orientation)
            .collect();
        let prim_lens: std::collections::BTreeSet<_> =
            vars.iter().map(|v| v.geometry.primitives.len()).collect();
        assert!(
            orients.len() >= 2 || prim_lens.len() >= 2,
            "expected contextual variants to differ in orientation or complexity"
        );
    }

    #[test]
    fn variant_selection_deterministic() {
        let a = select_variant_index(3, 10, 99, 1, 4);
        let b = select_variant_index(3, 10, 99, 1, 4);
        assert_eq!(a, b);
    }
}
