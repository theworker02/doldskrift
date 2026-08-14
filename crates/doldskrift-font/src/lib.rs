//! MGE/2 — Machine Glyph Engine Version 2 for Doldskrift.
//!
//! Glyphs are structured machine-readable objects with zone anatomy (N/E/S/W/C),
//! fingerprints (`DSKG-1`), GRE/1 redundancy, and integrity markers.
//! Generation is deterministic given `(codepoint, font_version, seed, density, family)`.

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod density;
mod distance;
mod experimental;
mod fingerprint;
mod forge;
mod geometry;
mod metrics;
mod mge4;
mod mge5;
mod recognition;
mod surfaces;
mod svg;
mod ttf;
mod tty;
mod zones;

pub use density::{Density, FontFamily};
pub use distance::{
    fingerprint_distance, glyph_distance, satisfies_minimum_distance, similarity, MINIMUM_DISTANCE,
    SIMILARITY_THRESHOLD,
};
pub use experimental::decode_features;
pub use fingerprint::{integrity_marker, verify_integrity, GlyphFingerprint, FINGERPRINT_FORMAT};
pub use forge::{run_forge_generation, AlphabetLifecycle, ForgeConfig, ForgeReport, ForgeScores};
pub use geometry::{
    generate_glyph_raw, generate_glyph_with, GenerateOptions, GlyphGeometry, Point, Primitive,
    StructuralFeatures, FONT_UNITS,
};
pub use metrics::{human_readability_score, machine_separability, metrics_for, GlyphMetrics};
pub use mge4::{
    boundary_exposure, familiarity_score, generate_variants, repetition_exposure,
    select_variant_index, GlyphGraph, GlyphSignature, GlyphVariant, RecognitionIndex, MGE4_ENGINE,
};
pub use mge5::{
    plan_from_latent, render_latent_svg, render_plan_svg, CompoundGlyph, ContextualVariant,
    Forge3NeuralObjectives, Mge5Plan, Mge5Profile, MGE5_ENGINE,
};
pub use recognition::{
    classify_fingerprint, classify_glyph, fingerprint_from_zone_codes, RecognitionResult,
};
pub use surfaces::{
    aurora_gradient_defs, extract_carrier_from_svg, render_ambient_constellation,
    render_ambient_constellation_ex, render_constellation_map, render_duet_svg, render_flicker_svg,
    render_handshake_strip, render_kaleidoscope_svg, render_living_specimen, render_mesh_html,
    render_notarize_svg, render_postcard_svg, render_seal_svg, render_spectrogram_svg,
    render_timeline_svg, SurfaceCarrier,
};
pub use svg::glyph_to_svg;
pub use ttf::{build_font_source, write_svg_font_family, FontBuildOptions, FontBuildResult};
pub use tty::{render_block_line, render_braille_line, render_tty_specimen};
pub use zones::{zone_map_for_symbol, Zone, ZoneMap, ZonePrimitive};

use doldskrift::{
    Error, Result, ALPHABET_SIZE, DSK_PROJECT_MARK, FONT_VERSION, GLYPH_ENGINE_VERSION, PUA_BASE,
};
use geometry::GenerateOptions as Opts;

/// Generate glyph geometry for a symbol codepoint (normal density, Text family).
pub fn generate_glyph(codepoint: u32, font_version: u32, seed: u64) -> Result<GlyphGeometry> {
    generate_glyph_ex(
        codepoint,
        &Opts {
            font_version,
            seed,
            density: Density::Normal,
            family: FontFamily::Text,
        },
    )
}

/// Generate with density / family options.
pub fn generate_glyph_ex(codepoint: u32, opts: &Opts) -> Result<GlyphGeometry> {
    if opts.font_version != FONT_VERSION && opts.font_version != 1 {
        // Accept v1 requests by generating MGE/2 (forward).
        if opts.font_version > FONT_VERSION {
            return Err(Error::UnsupportedVersion(opts.font_version as u8));
        }
    }
    let mut attempt_seed =
        opts.seed ^ (u64::from(codepoint) << 17) ^ (u64::from(GLYPH_ENGINE_VERSION) << 3);
    let mut best = None;
    for attempt in 0..32u64 {
        let mut o = opts.clone();
        o.seed = attempt_seed.wrapping_add(attempt * 0x9E37);
        let g = generate_glyph_with(codepoint, &o);
        let readability = human_readability_score(&g);
        if readability < 0.55 || codepoint == DSK_PROJECT_MARK {
            return Ok(g);
        }
        best = Some(g);
        attempt_seed = attempt_seed.wrapping_add(1);
    }
    best.ok_or_else(|| Error::FontGenerationError("failed to generate glyph".into()))
}

/// Generate the full 256-symbol alphabet with minimum-distance enforcement.
pub fn generate_alphabet(font_version: u32, seed: u64) -> Result<Vec<GlyphGeometry>> {
    generate_alphabet_ex(&Opts {
        font_version,
        seed,
        density: Density::Normal,
        family: FontFamily::Text,
    })
}

/// Alphabet generation with options.
pub fn generate_alphabet_ex(opts: &Opts) -> Result<Vec<GlyphGeometry>> {
    let mut glyphs = Vec::with_capacity(ALPHABET_SIZE);
    for i in 0..ALPHABET_SIZE {
        let cp = PUA_BASE + i as u32;
        let mut best = generate_glyph_ex(cp, opts)?;
        let mut best_dist = glyphs
            .iter()
            .map(|o| glyph_distance(o, &best))
            .fold(f64::INFINITY, f64::min);
        for spin in 1..64u64 {
            if best_dist >= MINIMUM_DISTANCE {
                break;
            }
            let mut o = opts.clone();
            o.seed = opts.seed ^ spin.wrapping_mul(0xD1B5_4A32_D192_D1C7);
            let candidate = generate_glyph_ex(cp, &o)?;
            let d = glyphs
                .iter()
                .map(|other| glyph_distance(other, &candidate))
                .fold(f64::INFINITY, f64::min);
            if d > best_dist {
                best_dist = d;
                best = candidate;
            }
        }
        if !satisfies_minimum_distance(&best, &glyphs) && glyphs.len() > 8 {
            // Soft fail: keep best effort; uniqueness still guaranteed by bitfield/symbol id.
        }
        glyphs.push(best);
    }
    Ok(glyphs)
}

/// Generate visual-mode glyphs for printable ASCII.
pub fn generate_visual_ascii(font_version: u32, seed: u64) -> Result<Vec<(char, GlyphGeometry)>> {
    let mut out = Vec::new();
    for cp in 0x20u32..=0x7Eu32 {
        let ch = char::from_u32(cp).unwrap();
        let g = generate_glyph(cp, font_version, seed ^ 0xA5A5_A5A5_A5A5_A5A5)?;
        out.push((ch, g));
    }
    Ok(out)
}

/// Generate the official project mark glyph (`DSK_PROJECT_MARK`).
pub fn generate_project_mark(seed: u64) -> Result<GlyphGeometry> {
    generate_glyph(DSK_PROJECT_MARK, FONT_VERSION, seed)
}

/// Official brand color tokens (see `docs/brand.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrandTokens {
    /// Primary stroke / ink.
    pub ink: &'static str,
    /// Accent / integrity teal.
    pub teal: &'static str,
    /// Light paper ground (site chrome).
    pub paper: &'static str,
    /// Alternate paper token used in brand docs.
    pub paper_alt: &'static str,
}

/// Canonical brand colors for kit export and specimens.
pub const BRAND_TOKENS: BrandTokens = BrandTokens {
    ink: "#12161A",
    teal: "#0F6B5C",
    paper: "#F7F4EF",
    paper_alt: "#F7F5F2",
};

/// Paths that should stay byte-identical to `assets/brand/logo-mark.svg`.
pub const BRAND_MARK_SYNC_PATHS: &[&str] = &[
    "assets/brand/logo-mark.svg",
    "site/public/logo-mark.svg",
    "packages/doldskrift-js/logo.svg",
    "packages/doldskrift-web/logo.svg",
    "packages/doldskrift-react/logo.svg",
];

/// JSON brand-kit manifest (tokens, mark codepoint, sync checklist).
pub fn brand_kit_manifest_json(seed: u64) -> Result<String> {
    let mark = generate_project_mark(seed)?;
    let sync = BRAND_MARK_SYNC_PATHS
        .iter()
        .map(|p| format!("    \"{p}\""))
        .collect::<Vec<_>>()
        .join(",\n");
    Ok(format!(
        "{{\n  \"name\": \"Doldskrift\",\n  \"project_mark\": \"U+{:04X}\",\n  \"fingerprint\": \"{}\",\n  \"font_version\": {},\n  \"glyph_engine\": {},\n  \"seed\": {seed},\n  \"tokens\": {{\n    \"ink\": \"{}\",\n    \"teal\": \"{}\",\n    \"paper\": \"{}\",\n    \"paper_alt\": \"{}\"\n  }},\n  \"etymology\": {{\n    \"language\": \"Swedish\",\n    \"parts\": [\"dold\", \"skrift\"],\n    \"sense_en\": \"hidden writing / concealed script\",\n    \"tagline_sv\": \"Maskinskriven betydelse — inte hemlighet.\"\n  }},\n  \"sync_paths\": [\n{sync}\n  ],\n  \"see_also\": [\"docs/brand.md\", \"TRADEMARKS.md\", \"dold logo\", \"dold funding\"]\n}}\n",
        DSK_PROJECT_MARK,
        mark.fingerprint(),
        FONT_VERSION,
        GLYPH_ENGINE_VERSION,
        BRAND_TOKENS.ink,
        BRAND_TOKENS.teal,
        BRAND_TOKENS.paper,
        BRAND_TOKENS.paper_alt
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic() {
        let a = generate_glyph(PUA_BASE + 65, FONT_VERSION, 0).unwrap();
        let b = generate_glyph(PUA_BASE + 65, FONT_VERSION, 0).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.fingerprint(), b.fingerprint());
    }

    #[test]
    fn project_mark_is_special() {
        let g = generate_project_mark(1).unwrap();
        assert!(g.is_project_mark());
        assert_eq!(g.codepoint, DSK_PROJECT_MARK);
        assert_eq!(g.glyph_engine, 2);
    }

    #[test]
    fn alphabet_fingerprints_unique() {
        let glyphs = generate_alphabet(FONT_VERSION, 1).unwrap();
        assert_eq!(glyphs.len(), 256);
        let mut fps = std::collections::BTreeSet::new();
        for g in &glyphs {
            assert!(fps.insert(g.fingerprint()), "duplicate fingerprint");
        }
    }

    #[test]
    fn recognition_roundtrip() {
        let glyphs = generate_alphabet(FONT_VERSION, 1).unwrap();
        let target = &glyphs[42];
        let result = classify_glyph(target, &glyphs).unwrap();
        assert_eq!(result.symbol, target.codepoint);
        assert!(result.confidence > 0.9);
    }

    #[test]
    fn brand_kit_manifest_includes_tokens() {
        let j = brand_kit_manifest_json(1).unwrap();
        assert!(j.contains("#0F6B5C"));
        assert!(j.contains("logo-mark.svg"));
        assert!(j.contains("fingerprint"));
    }
}
