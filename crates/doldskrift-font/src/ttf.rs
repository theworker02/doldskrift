//! Reproducible font build pipeline.
//!
//! Generates an SVG font family and companion metadata that can be converted
//! to OTF/TTF/WOFF2 with standard tools. The procedural source of truth is
//! always regenerable from `(font_version, seed)`.

use crate::density::{Density, FontFamily};
use crate::geometry::{GenerateOptions, GlyphGeometry};
use crate::svg::primitive_to_svg;
use crate::{generate_alphabet_ex, generate_project_mark, generate_visual_ascii};
use doldskrift::{Error, Result, DSK_PROJECT_MARK, FONT_VERSION, GLYPH_ENGINE_VERSION, PUA_BASE};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

/// Options for building font artifacts.
#[derive(Debug, Clone)]
pub struct FontBuildOptions {
    /// Output directory (e.g. `fonts/generated`).
    pub out_dir: PathBuf,
    /// Font grammar version.
    pub font_version: u32,
    /// Deterministic seed.
    pub seed: u64,
    /// Family name (legacy single-family builds).
    pub family_name: String,
    /// Build Text / Mono / Micro variants.
    pub all_families: bool,
}

impl Default for FontBuildOptions {
    fn default() -> Self {
        Self {
            out_dir: PathBuf::from("fonts/generated"),
            font_version: FONT_VERSION,
            seed: 1,
            family_name: "DoldskriftText".into(),
            all_families: true,
        }
    }
}

/// Result of a font build.
#[derive(Debug, Clone, Serialize)]
pub struct FontBuildResult {
    /// Path to SVG font.
    pub svg_font: PathBuf,
    /// Path to glyph JSON catalog.
    pub catalog: PathBuf,
    /// Path to CSS `@font-face` helper (references WOFF2 when present).
    pub css: PathBuf,
    /// Number of glyphs generated.
    pub glyph_count: usize,
    /// Seed used.
    pub seed: u64,
    /// Font version.
    pub font_version: u32,
}

/// Build font source artifacts into `out_dir`.
pub fn build_font_source(opts: &FontBuildOptions) -> Result<FontBuildResult> {
    fs::create_dir_all(&opts.out_dir).map_err(|e| Error::IoError(e.to_string()))?;

    let families = if opts.all_families {
        vec![FontFamily::Text, FontFamily::Mono, FontFamily::Micro]
    } else {
        vec![FontFamily::Text]
    };

    let mut primary_alphabet = None;
    let mut primary_visual = None;
    let mut svg_path = opts.out_dir.join("DoldskriftText-Regular.svg");

    for family in families {
        let density = family.default_density();
        let gen = GenerateOptions {
            font_version: opts.font_version,
            seed: opts.seed,
            density,
            family,
        };
        let mut alphabet = generate_alphabet_ex(&gen)?;
        let mark = generate_project_mark(opts.seed)?;
        alphabet.push(mark);
        let visual = generate_visual_ascii(opts.font_version, opts.seed)?;
        let name = family.as_str();
        let path = opts.out_dir.join(format!("{name}-Regular.svg"));
        write_svg_font_family(&path, name, &alphabet, &visual)?;
        if family == FontFamily::Text {
            svg_path = path;
            primary_alphabet = Some(alphabet);
            primary_visual = Some(visual);
        }
        let _ = Density::Normal; // keep density module linked via family defaults
    }

    let alphabet = primary_alphabet.unwrap();
    let visual = primary_visual.unwrap();

    let catalog_path = opts.out_dir.join("glyphs.json");
    let css_path = opts.out_dir.join("doldskrift.css");
    let source_meta = opts.out_dir.join("build-manifest.json");

    let visual_entries: Vec<VisualEntry> = visual
        .iter()
        .map(|(ch, g)| VisualEntry {
            char: ch.to_string(),
            codepoint: *ch as u32,
            glyph: g.clone(),
        })
        .collect();
    let glyph_count = alphabet.len() + visual_entries.len();
    let catalog = GlyphCatalog {
        font_version: opts.font_version,
        glyph_engine: GLYPH_ENGINE_VERSION,
        seed: opts.seed,
        family: FontFamily::Text.as_str().into(),
        pua_base: PUA_BASE,
        project_mark: DSK_PROJECT_MARK,
        alphabet,
        visual: visual_entries,
    };
    let catalog_json = serde_json::to_string_pretty(&catalog)
        .map_err(|e| Error::FontGenerationError(e.to_string()))?;
    fs::write(&catalog_path, catalog_json).map_err(|e| Error::IoError(e.to_string()))?;

    let css = r#"/* Doldskrift MGE/2 — generated stylesheet */
@font-face {
    font-family: "DoldskriftText";
    src: url("./DoldskriftText-Regular.svg#DoldskriftText") format("svg");
    font-weight: normal;
    font-style: normal;
    font-display: block;
}
@font-face {
    font-family: "DoldskriftMono";
    src: url("./DoldskriftMono-Regular.svg#DoldskriftMono") format("svg");
    font-weight: normal;
    font-style: normal;
    font-display: block;
}
@font-face {
    font-family: "DoldskriftMicro";
    src: url("./DoldskriftMicro-Regular.svg#DoldskriftMicro") format("svg");
    font-weight: normal;
    font-style: normal;
    font-display: block;
}
.doldskrift, .agent-text {
    font-family: "DoldskriftText", "Doldskrift", monospace;
    letter-spacing: 0.04em;
    line-height: 1.4;
}
.doldskrift-mono { font-family: "DoldskriftMono", monospace; }
.doldskrift-micro { font-family: "DoldskriftMicro", monospace; }
"#;
    fs::write(&css_path, css).map_err(|e| Error::IoError(e.to_string()))?;

    let readme = opts.out_dir.join("README.md");
    fs::write(
        &readme,
        r#"# Generated Doldskrift Font Artifacts

These files are produced by `dold font build` from the procedural glyph grammar.

- `Doldskrift-Regular.svg` — SVG font source (checked into `fonts/source` after promotion)
- `glyphs.json` — full geometry catalog
- `doldskrift.css` — `@font-face` helper

To produce OTF/TTF/WOFF2, run the converter in `tools/fontconvert` (fonttools),
or use any SVG-font → OpenType pipeline. The geometry is fully determined by
`(font_version, seed)` recorded in `build-manifest.json`.
"#,
    )
    .map_err(|e| Error::IoError(e.to_string()))?;

    let manifest = serde_json::json!({
        "font_version": opts.font_version,
        "glyph_engine": GLYPH_ENGINE_VERSION,
        "seed": opts.seed,
        "families": ["DoldskriftText", "DoldskriftMono", "DoldskriftMicro"],
        "project_mark": format!("U+{DSK_PROJECT_MARK:04X}"),
        "glyph_count": glyph_count,
        "generator": "doldskrift-font-mge2",
    });
    fs::write(
        &source_meta,
        serde_json::to_string_pretty(&manifest).unwrap(),
    )
    .map_err(|e| Error::IoError(e.to_string()))?;

    if let Some(parent) = opts.out_dir.parent() {
        let source_dir = parent.join("source");
        let _ = fs::create_dir_all(&source_dir);
        let _ = fs::copy(&svg_path, source_dir.join("Doldskrift-Regular.svg"));
        let _ = fs::copy(&catalog_path, source_dir.join("glyphs.json"));
    }

    Ok(FontBuildResult {
        svg_font: svg_path,
        catalog: catalog_path,
        css: css_path,
        glyph_count,
        seed: opts.seed,
        font_version: opts.font_version,
    })
}

#[derive(Serialize)]
struct GlyphCatalog {
    font_version: u32,
    glyph_engine: u32,
    seed: u64,
    family: String,
    pua_base: u32,
    project_mark: u32,
    alphabet: Vec<GlyphGeometry>,
    visual: Vec<VisualEntry>,
}

#[derive(Serialize)]
struct VisualEntry {
    char: String,
    codepoint: u32,
    glyph: GlyphGeometry,
}

/// Write an SVG Font 1.1 document covering PUA alphabet + visual ASCII.
pub fn write_svg_font_family(
    path: &Path,
    family: &str,
    alphabet: &[GlyphGeometry],
    visual: &[(char, GlyphGeometry)],
) -> Result<()> {
    let mut body = String::new();
    body.push_str(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" version="1.1">
<defs>
"#,
    );
    body.push_str(&format!(
        r#"<font id="Doldskrift" horiz-adv-x="620">
<font-face font-family="{family}" units-per-em="1000" ascent="800" descent="-200" />
<missing-glyph horiz-adv-x="620" d="M100 100h400v600h-400z" />
"#
    ));

    for (ch, g) in visual {
        body.push_str(&glyph_elem(*ch as u32, Some(*ch), g));
    }
    for g in alphabet {
        let ch = char::from_u32(g.codepoint).unwrap_or('\u{FFFD}');
        body.push_str(&glyph_elem(g.codepoint, Some(ch), g));
    }

    body.push_str("</font>\n</defs>\n</svg>\n");
    fs::write(path, body).map_err(|e| Error::IoError(e.to_string()))
}

fn glyph_elem(cp: u32, ch: Option<char>, g: &GlyphGeometry) -> String {
    let unicode = ch
        .map(xml_escape_char)
        .unwrap_or_else(|| format!("&#x{cp:X};"));
    let d = primitives_to_path_d(&g.primitives);
    format!(
        r#"<glyph unicode="{unicode}" glyph-name="dk_{cp:04X}" horiz-adv-x="{}" d="{d}" />"#,
        g.advance
    )
}

fn xml_escape_char(c: char) -> String {
    match c {
        '&' => "&amp;".into(),
        '<' => "&lt;".into(),
        '>' => "&gt;".into(),
        '"' => "&quot;".into(),
        '\'' => "&apos;".into(),
        _ => c.to_string(),
    }
}

fn primitives_to_path_d(primitives: &[crate::geometry::Primitive]) -> String {
    // Approximate path data for SVG font (y-up).
    let mut d = String::new();
    for p in primitives {
        // Reuse SVG helper then strip tags — simpler: emit path ops directly.
        let _ = primitive_to_svg(p);
        match p {
            crate::geometry::Primitive::Line { from, to } => {
                d.push_str(&format!("M{} {} L{} {} ", from.x, from.y, to.x, to.y));
            }
            crate::geometry::Primitive::Dot { at, radius } => {
                // Approximate circle as two arcs
                let r = *radius;
                d.push_str(&format!(
                    "M{} {} a{r} {r} 0 1 0 {} 0 a{r} {r} 0 1 0 -{} 0 ",
                    at.x - r,
                    at.y,
                    r * 2,
                    r * 2
                ));
            }
            crate::geometry::Primitive::Ring { center, radius } => {
                let r = *radius;
                d.push_str(&format!(
                    "M{} {} a{r} {r} 0 1 0 {} 0 a{r} {r} 0 1 0 -{} 0 ",
                    center.x - r,
                    center.y,
                    r * 2,
                    r * 2
                ));
            }
            crate::geometry::Primitive::Enclosure {
                origin,
                width,
                height,
            } => {
                d.push_str(&format!(
                    "M{} {} h{} v{} h{} z ",
                    origin.x, origin.y, width, height, -width
                ));
            }
            crate::geometry::Primitive::Branch { origin, a, b } => {
                d.push_str(&format!(
                    "M{} {} L{} {} M{} {} L{} {} ",
                    origin.x, origin.y, a.x, a.y, origin.x, origin.y, b.x, b.y
                ));
            }
            crate::geometry::Primitive::Crossing { at, arm, .. } => {
                d.push_str(&format!(
                    "M{} {} L{} {} M{} {} L{} {} ",
                    at.x - arm,
                    at.y,
                    at.x + arm,
                    at.y,
                    at.x,
                    at.y - arm,
                    at.x,
                    at.y + arm
                ));
            }
            crate::geometry::Primitive::Terminal { at, .. } => {
                d.push_str(&format!("M{} {} L{} {} ", at.x - 30, at.y, at.x + 30, at.y));
            }
            crate::geometry::Primitive::Junction { at } => {
                d.push_str(&format!(
                    "M{} {} L{} {} M{} {} L{} {} ",
                    at.x - 20,
                    at.y,
                    at.x + 20,
                    at.y,
                    at.x,
                    at.y - 20,
                    at.x,
                    at.y + 20
                ));
            }
            crate::geometry::Primitive::Arc {
                center,
                radius,
                start_deg,
                sweep_deg,
            } => {
                // Approximate arc with a line chord for SVG font compactness
                let rad0 = f64::from(*start_deg).to_radians();
                let rad1 = f64::from(*start_deg + *sweep_deg).to_radians();
                let x1 = f64::from(center.x) + f64::from(*radius) * rad0.cos();
                let y1 = f64::from(center.y) + f64::from(*radius) * rad0.sin();
                let x2 = f64::from(center.x) + f64::from(*radius) * rad1.cos();
                let y2 = f64::from(center.y) + f64::from(*radius) * rad1.sin();
                let large = if sweep_deg.abs() > 180 { 1 } else { 0 };
                let sweep = if *sweep_deg >= 0 { 1 } else { 0 };
                d.push_str(&format!(
                    "M{x1:.1} {y1:.1} A{radius} {radius} 0 {large} {sweep} {x2:.1} {y2:.1} "
                ));
            }
        }
    }
    d
}
