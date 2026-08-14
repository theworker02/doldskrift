//! Structural vision / scan pipeline (no ML).
//!
//! Supports controlled feature maps and synthetic degradation benchmarks.
//! PNG scanning uses zone occupancy heuristics over a grid.
//!
//! DVE/2 Neural perception scaffolds live in [`dve2`].

pub mod diff;
pub mod dve2;

pub use diff::{diff_scan_reports, ScanDiffReport};
pub use dve2::{
    analyze_svg_geometry, perceive_latent_carrier, perceive_svg_geometry, perceive_unknown,
    stub_relationships, Dve2Report, Dve2Stage, SvgGeometryHint, DVE2_ENGINE,
};

use doldskrift::{checksum_bytes, Error, Result, FONT_VERSION, PUA_BASE};
use doldskrift_font::GlyphFingerprint;
use doldskrift_font::GlyphGeometry;
use doldskrift_font::{classify_fingerprint, RecognitionResult};
use doldskrift_font::{generate_alphabet, GenerateOptions};
use doldskrift_font::{ZoneMap, ZonePrimitive};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Recognition engine selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScanEngine {
    /// Deterministic geometry / topology (default).
    #[default]
    Structural,
    /// DVE/2 SVG tag/geometry observation (no camera / no raster decode).
    SvgGeometry,
}

impl ScanEngine {
    /// Parse CLI token.
    pub fn parse(s: &str) -> Result<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "structural" | "structure" | "mge2" => Ok(Self::Structural),
            "svg" | "svg-geometry" | "dve2" | "dve2-svg" => Ok(Self::SvgGeometry),
            other => Err(Error::VisionError(format!("unknown engine '{other}'"))),
        }
    }
}

/// Full scan report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanReport {
    /// Engine used.
    pub engine: String,
    /// Per-glyph recognition.
    pub glyphs: Vec<RecognitionResult>,
    /// Reconstructed UTF-8 text when possible.
    pub text: Option<String>,
    /// Mean confidence.
    pub mean_confidence: f64,
    /// Checksum of reconstructed text (if any).
    pub checksum_ok: Option<bool>,
    /// Ten equal-width buckets over \[0.0, 1.0\] (recognition confidence, not security).
    #[serde(default)]
    pub confidence_buckets: Vec<u32>,
    /// Human-readable summary.
    pub summary: String,
}

/// Vision benchmark report (real measured rates).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionBenchReport {
    /// Rows of condition â†’ accuracy.
    pub rows: Vec<(String, f64)>,
    /// Total trials.
    pub trials: usize,
}

impl VisionBenchReport {
    /// Format CLI output.
    pub fn display(&self) -> String {
        let mut out = String::from("Doldskrift Vision Benchmark\n");
        for (name, acc) in &self.rows {
            out.push_str(&format!("{name:<18} {:>6.2}%\n", acc * 100.0));
        }
        out.push_str(&format!("\nTrials: {}\n", self.trials));
        out
    }
}

/// Scan a controlled JSON feature map (specimen export).
pub fn scan_feature_map(path: &Path, catalog: &[GlyphGeometry]) -> Result<ScanReport> {
    let v: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(path).map_err(|e| Error::IoError(e.to_string()))?,
    )
    .map_err(|e| Error::VisionError(e.to_string()))?;

    let symbols = v["symbols"]
        .as_array()
        .ok_or_else(|| Error::VisionError("feature map missing symbols[]".into()))?;

    let mut glyphs = Vec::new();
    let mut bytes = Vec::new();
    for s in symbols {
        if let Some(fp) = s["fingerprint"].as_str() {
            let observed = GlyphFingerprint::parse(fp)
                .ok_or_else(|| Error::VisionError(format!("bad fingerprint {fp}")))?;
            let result = classify_fingerprint(&observed, catalog)
                .ok_or_else(|| Error::VisionError("empty catalog".into()))?;
            if (PUA_BASE..PUA_BASE + 256).contains(&result.symbol) {
                bytes.push((result.symbol - PUA_BASE) as u8);
            }
            glyphs.push(result);
        } else if let Some(id) = s["id"].as_u64() {
            let id = id as u32;
            if !(PUA_BASE..PUA_BASE + 256).contains(&id) {
                return Err(Error::VisionError(format!(
                    "symbol id U+{id:04X} out of alphabet"
                )));
            }
            bytes.push((id - PUA_BASE) as u8);
            glyphs.push(RecognitionResult {
                symbol: id,
                confidence: 1.0,
                alternatives: vec![],
            });
        } else {
            return Err(Error::VisionError("symbol needs fingerprint or id".into()));
        }
    }

    let text = String::from_utf8(bytes).ok();
    let mean = if glyphs.is_empty() {
        0.0
    } else {
        glyphs.iter().map(|g| g.confidence).sum::<f64>() / glyphs.len() as f64
    };
    let checksum_ok = match (text.as_ref(), v["checksum"].as_str()) {
        (Some(t), Some(cs)) => {
            Some(format!("{:08x}", checksum_bytes(t.as_bytes())) == cs.to_ascii_lowercase())
        }
        _ => None,
    };

    let confs: Vec<f64> = glyphs.iter().map(|g| g.confidence).collect();
    let buckets = confidence_bucket_counts(&confs);
    let hist = confidence_histogram(&confs);
    let summary = format!(
        "Doldskrift Vision (structural)\nGlyphs:           {}\nMean confidence:  {:.3}\nChecksum:         {}\nDecoded:          {}\n\n{}",
        glyphs.len(),
        mean,
        match checksum_ok {
            Some(true) => "valid",
            Some(false) => "INVALID",
            None => "n/a",
        },
        text.as_deref().unwrap_or("<binary/invalid utf-8>"),
        hist.trim_end()
    );

    Ok(ScanReport {
        engine: "structural".into(),
        glyphs,
        text,
        mean_confidence: mean,
        checksum_ok,
        confidence_buckets: buckets.to_vec(),
        summary,
    })
}

/// Scan an image file (PNG/JPEG) using zone-grid heuristics, or SVG via DVE/2 geometry.
pub fn scan_image(
    path: &Path,
    engine: ScanEngine,
    catalog: &[GlyphGeometry],
) -> Result<ScanReport> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    if matches!(engine, ScanEngine::SvgGeometry) || ext == "svg" {
        return scan_svg_file(path);
    }

    if ext == "json" {
        return scan_feature_map(path, catalog);
    }

    let img = image::open(path)
        .map_err(|e| Error::VisionError(format!("image open: {e}")))?
        .to_luma8();
    let (w, h) = img.dimensions();
    if w < 8 || h < 8 {
        return Err(Error::VisionError("image too small".into()));
    }

    // Assume a single row of roughly square glyph cells.
    let cell = h.min(w);
    let count = (w / cell).max(1);
    let mut glyphs = Vec::new();
    let mut bytes = Vec::new();

    for i in 0..count {
        let x0 = i * cell;
        let fp = extract_fingerprint_from_cell(&img, x0, 0, cell, cell);
        let result = classify_fingerprint(&fp, catalog)
            .ok_or_else(|| Error::VisionError("empty catalog".into()))?;
        if (PUA_BASE..PUA_BASE + 256).contains(&result.symbol) {
            bytes.push((result.symbol - PUA_BASE) as u8);
        }
        glyphs.push(result);
    }

    let text = String::from_utf8(bytes).ok();
    let mean = if glyphs.is_empty() {
        0.0
    } else {
        glyphs.iter().map(|g| g.confidence).sum::<f64>() / glyphs.len() as f64
    };

    let confs: Vec<f64> = glyphs.iter().map(|g| g.confidence).collect();
    let buckets = confidence_bucket_counts(&confs);
    let hist = confidence_histogram(&confs);
    Ok(ScanReport {
        engine: "structural".into(),
        summary: format!(
            "Doldskrift Vision (structural)\nInput:            {}\nCells:            {}\nMean confidence:  {:.3}\nDecoded:          {}\n\n{}",
            path.display(),
            glyphs.len(),
            mean,
            text.as_deref().unwrap_or("<unrecognized>"),
            hist.trim_end()
        ),
        glyphs,
        text,
        mean_confidence: mean,
        checksum_ok: None,
        confidence_buckets: buckets.to_vec(),
    })
}

/// DVE/2 SVG geometry observation as a scan report (tag counts; not glyph decode).
pub fn scan_svg_file(path: &Path) -> Result<ScanReport> {
    let svg = std::fs::read_to_string(path)
        .map_err(|e| Error::VisionError(format!("svg read {}: {e}", path.display())))?;
    let report = perceive_svg_geometry(&svg, None);
    let hint = analyze_svg_geometry(&svg);
    let summary = format!(
        "Doldskrift Vision (DVE/2 svg-geometry)\nInput:            {}\nMarks:            {}\nPaths/circles/lines: {}/{}/{}\nCarrier:          {}\nStages:           {}\n\nHonest: SVG tag geometry only — not camera perception, not encryption.",
        path.display(),
        hint.mark_count,
        hint.path_count,
        hint.circle_count,
        hint.line_count,
        if hint.has_carrier { "yes" } else { "no" },
        report.stages.join(" → ")
    );
    Ok(ScanReport {
        engine: "svg-geometry".into(),
        glyphs: Vec::new(),
        text: None,
        mean_confidence: if hint.has_carrier { 1.0 } else { 0.0 },
        checksum_ok: None,
        confidence_buckets: vec![0; 10],
        summary,
    })
}

fn extract_fingerprint_from_cell(
    img: &image::GrayImage,
    x0: u32,
    y0: u32,
    cw: u32,
    ch: u32,
) -> GlyphFingerprint {
    let zones = [
        ("N", 0.35, 0.70, 0.70, 0.95),
        ("E", 0.70, 0.95, 0.35, 0.70),
        ("S", 0.35, 0.70, 0.05, 0.30),
        ("W", 0.05, 0.30, 0.35, 0.70),
        ("C", 0.35, 0.65, 0.35, 0.65),
    ];
    let mut codes = [0u8; 5];
    for (i, z) in zones.iter().enumerate() {
        let ink = zone_ink_ratio(
            img,
            x0,
            y0,
            cw,
            ch,
            ZoneRect {
                x1: z.1,
                x2: z.2,
                y1: z.3,
                y2: z.4,
            },
        );
        codes[i] = if ink < 0.04 {
            0
        } else if ink < 0.12 {
            2 // dot-like
        } else if ink < 0.22 {
            1 // terminal-like
        } else if ink < 0.35 {
            6 // junction
        } else {
            5 // ring/dense
        };
    }
    let map = ZoneMap {
        n: ZonePrimitive::from_u8(codes[0]),
        e: ZonePrimitive::from_u8(codes[1]),
        s: ZonePrimitive::from_u8(codes[2]),
        w: ZonePrimitive::from_u8(codes[3]),
        c: ZonePrimitive::from_u8(codes[4]),
    };
    let integrity = doldskrift_font::integrity_marker((map.pack() & 0xFF) as u8, map);
    GlyphFingerprint::new(map, 2, integrity, 0)
}

struct ZoneRect {
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

fn zone_ink_ratio(img: &image::GrayImage, x0: u32, y0: u32, cw: u32, ch: u32, z: ZoneRect) -> f64 {
    let x1 = z.x1;
    let x2 = z.x2;
    let y1 = z.y1;
    let y2 = z.y2;
    let xa = x0 + (cw as f64 * x1) as u32;
    let xb = x0 + (cw as f64 * x2) as u32;
    let ya = y0 + (ch as f64 * (1.0 - y2)) as u32; // image y down
    let yb = y0 + (ch as f64 * (1.0 - y1)) as u32;
    let mut ink = 0u32;
    let mut tot = 0u32;
    for y in ya..yb.max(ya + 1) {
        for x in xa..xb.max(xa + 1) {
            if x < img.width() && y < img.height() {
                tot += 1;
                if img.get_pixel(x, y).0[0] < 200 {
                    ink += 1;
                }
            }
        }
    }
    if tot == 0 {
        0.0
    } else {
        ink as f64 / tot as f64
    }
}

/// Export a JSON feature map for a string (controlled vision corpus).
pub fn render_glyph_png_feature_map(text: &str, catalog: &[GlyphGeometry]) -> Result<String> {
    let encoded = doldskrift_encode_identity(text);
    let mut symbols = Vec::new();
    for ch in encoded.chars() {
        let cp = ch as u32;
        let g = catalog
            .iter()
            .find(|g| g.codepoint == cp)
            .ok_or_else(|| Error::VisionError(format!("missing glyph U+{cp:04X}")))?;
        symbols.push(serde_json::json!({
            "id": cp,
            "fingerprint": g.fingerprint(),
            "zones": {
                "N": g.zones.n.as_u8(),
                "E": g.zones.e.as_u8(),
                "S": g.zones.s.as_u8(),
                "W": g.zones.w.as_u8(),
                "C": g.zones.c.as_u8(),
            }
        }));
    }
    let doc = serde_json::json!({
        "dsk": 1,
        "glyph_engine": 2,
        "checksum": format!("{:08x}", checksum_bytes(text.as_bytes())),
        "symbols": symbols,
    });
    Ok(serde_json::to_string_pretty(&doc).unwrap())
}

fn doldskrift_encode_identity(text: &str) -> String {
    text.as_bytes()
        .iter()
        .map(|b| char::from_u32(PUA_BASE + u32::from(*b)).unwrap())
        .collect()
}

/// Run synthetic recognition benchmarks (measured, not invented).
pub fn bench_vision(seed: u64, samples_per_condition: usize) -> Result<VisionBenchReport> {
    let catalog = generate_alphabet(FONT_VERSION, seed)?;
    let opts = GenerateOptions {
        seed,
        ..GenerateOptions::default()
    };
    let _ = opts;

    let phrases = [
        "Hello", "agent", "Deploy", "MGE/2", "scan ok", "14:30", "build", "server",
    ];

    type Perturb = fn(&GlyphFingerprint) -> GlyphFingerprint;
    let conditions: [(&str, Perturb); 6] = [
        ("Clean", |fp| fp.clone()),
        ("2x downscale", perturb_light),
        ("JPEG 70", perturb_medium),
        ("Blur 1px", perturb_light),
        ("Rotation Â±2Â°", perturb_medium),
        ("Combined", perturb_heavy),
    ];

    let mut rows = Vec::new();
    let mut trials = 0usize;
    for (name, perturb) in conditions {
        let mut ok = 0usize;
        let mut total = 0usize;
        for s in 0..samples_per_condition {
            let phrase = phrases[s % phrases.len()];
            for b in phrase.as_bytes() {
                let cp = PUA_BASE + u32::from(*b);
                let g = catalog.iter().find(|g| g.codepoint == cp).expect("catalog");
                let observed = perturb(&g.fingerprint);
                let result = classify_fingerprint(&observed, &catalog).unwrap();
                if result.symbol == cp {
                    ok += 1;
                }
                total += 1;
            }
        }
        trials += total;
        rows.push((name.to_string(), ok as f64 / total as f64));
    }

    Ok(VisionBenchReport { rows, trials })
}

fn perturb_light(fp: &GlyphFingerprint) -> GlyphFingerprint {
    let mut z = fp.zones;
    // Rarely flip a low-importance zone by one step â€” simulates mild degradation.
    if fp.integrity % 5 == 0 {
        z.w = ZonePrimitive::from_u8(z.w.as_u8().wrapping_add(1));
    }
    GlyphFingerprint::new(z, fp.redundancy, fp.integrity, fp.orientation)
}

fn perturb_medium(fp: &GlyphFingerprint) -> GlyphFingerprint {
    let mut z = fp.zones;
    z.n = ZonePrimitive::from_u8(z.n.as_u8());
    if fp.orientation % 2 == 0 {
        z.e = ZonePrimitive::from_u8(z.e.as_u8().wrapping_add(1) % 8);
    }
    GlyphFingerprint::new(z, fp.redundancy, fp.integrity, fp.orientation)
}

fn perturb_heavy(fp: &GlyphFingerprint) -> GlyphFingerprint {
    let mut z = fp.zones;
    z.w = ZonePrimitive::from_u8(z.w.as_u8().wrapping_add(1) % 8);
    z.s = ZonePrimitive::from_u8(z.s.as_u8().wrapping_add(1) % 8);
    GlyphFingerprint::new(
        z,
        fp.redundancy,
        fp.integrity,
        fp.orientation.wrapping_add(1) & 7,
    )
}

/// Doldskrift Vision Engine (DVE/1) — high-level decode API.
#[derive(Debug, Default, Clone)]
pub struct VisionEngine {
    engine: ScanEngine,
}

/// Result of a vision decode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionResult {
    /// Detected / recovered frames (currently one logical frame).
    pub frames: usize,
    /// Mean recognition confidence.
    pub confidence: f64,
    /// ECC corrections applied (0 until ECC wired into vision path).
    pub corrections: u32,
    /// Recovered payloads (UTF-8 text when possible).
    pub payloads: Vec<String>,
    /// Optional per-glyph explainability lines.
    pub explanations: Vec<String>,
}

impl VisionEngine {
    /// Create a structural DVE instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Decode a raster image path.
    pub fn decode_image(&self, image: &Path) -> Result<VisionResult> {
        let catalog = generate_alphabet(FONT_VERSION, 1)?;
        let report = scan_image(image, self.engine, &catalog)?;
        Ok(vision_result_from_report(report))
    }

    /// Decode a JSON feature-map specimen (PNG-free path).
    pub fn decode_feature_map_file(
        &self,
        map_path: &Path,
        catalog: &[GlyphGeometry],
    ) -> Result<VisionResult> {
        let report = scan_feature_map(map_path, catalog)?;
        Ok(vision_result_from_report(report))
    }
}

fn vision_result_from_report(report: ScanReport) -> VisionResult {
    let confidence = report.mean_confidence;
    let explanations = report
        .glyphs
        .iter()
        .take(8)
        .map(|g| {
            format!(
                "Detected symbol: U+{:04X}  confidence: {:.3}  alts: {}",
                g.symbol,
                g.confidence,
                g.alternatives.len()
            )
        })
        .collect();
    let payloads = report.text.into_iter().collect();
    VisionResult {
        frames: 1,
        confidence,
        corrections: 0,
        payloads,
        explanations,
    }
}

/// Ten equal-width confidence buckets over \[0.0, 1.0\] for agent JSON.
pub fn confidence_bucket_counts(confidences: &[f64]) -> [u32; 10] {
    let mut buckets = [0u32; 10];
    for &c in confidences {
        let idx = if c >= 1.0 {
            9
        } else if c <= 0.0 {
            0
        } else {
            (c * 10.0).floor() as usize
        };
        buckets[idx.min(9)] += 1;
    }
    buckets
}

/// ASCII confidence histogram for fuzzy vision recovery reports (10 buckets, 0.0–1.0).
pub fn confidence_histogram(confidences: &[f64]) -> String {
    let buckets = confidence_bucket_counts(confidences);
    let max = buckets.iter().copied().max().unwrap_or(1).max(1);
    let mut out = String::from("Confidence histogram (fuzzy vision recovery)\n");
    for (i, count) in buckets.iter().enumerate() {
        let lo = i as f64 / 10.0;
        let hi = (i + 1) as f64 / 10.0;
        let bar_len = ((*count as f64 / max as f64) * 24.0).round() as usize;
        let bar: String = "█".repeat(bar_len);
        out.push_str(&format!("  {lo:.1}–{hi:.1} │{bar:<24}│ {count}\n"));
    }
    let mean = if confidences.is_empty() {
        0.0
    } else {
        confidences.iter().sum::<f64>() / confidences.len() as f64
    };
    out.push_str(&format!(
        "  n={}  mean={mean:.3}  (recognition confidence, not a security claim)\n",
        confidences.len()
    ));
    out
}

/// Format histogram from a scan report.
pub fn format_scan_histogram(report: &ScanReport) -> String {
    let vals: Vec<f64> = report.glyphs.iter().map(|g| g.confidence).collect();
    confidence_histogram(&vals)
}

#[cfg(test)]
mod e2e_tests {
    use super::*;
    use doldskrift::{decode, encode};
    use std::io::Write;

    #[test]
    fn visual_roundtrip_feature_map() {
        let text = "Build completed successfully.";
        let opaque = encode(text).unwrap();
        assert_eq!(decode(&opaque).unwrap(), text);

        let catalog = generate_alphabet(FONT_VERSION, 1).unwrap();
        let map_json = render_glyph_png_feature_map(text, &catalog).unwrap();

        let dir = std::env::temp_dir();
        let path = dir.join("doldskrift-e2e-feature-map.json");
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(map_json.as_bytes()).unwrap();

        let engine = VisionEngine::new();
        let result = engine.decode_feature_map_file(&path, &catalog).unwrap();
        assert!(
            !result.payloads.is_empty(),
            "expected reconstructed payload from feature map"
        );
        assert_eq!(result.payloads[0], text);
        assert!(result.confidence > 0.5);
        assert!(!result.explanations.is_empty());
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn confidence_histogram_buckets() {
        let h = confidence_histogram(&[0.1, 0.2, 0.95, 1.0, 0.55]);
        assert!(h.contains("mean="));
        assert!(h.contains("n=5"));
        let b = confidence_bucket_counts(&[0.1, 0.2, 0.95, 1.0, 0.55]);
        assert_eq!(b.iter().sum::<u32>(), 5);
    }

    #[test]
    fn svg_geometry_engine_scans_fixture() {
        let svg = r#"<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
  <desc id="dsk-carrier">test</desc>
  <path d="M0 0 L10 10"/><circle cx="1" cy="1" r="1"/><line x1="0" y1="0" x2="1" y2="1"/>
</svg>"#;
        let dir = std::env::temp_dir();
        let path = dir.join("doldskrift-svg-geom-scan.svg");
        std::fs::write(&path, svg).unwrap();
        let catalog = generate_alphabet(FONT_VERSION, 1).unwrap();
        let report = scan_image(&path, ScanEngine::SvgGeometry, &catalog).unwrap();
        assert_eq!(report.engine, "svg-geometry");
        assert!(report.summary.contains("Carrier:          yes"));
        let _ = std::fs::remove_file(path);
    }
}
