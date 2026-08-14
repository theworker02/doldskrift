//! Unexpected visual surfaces: ambient constellation, temporal flicker, living variants,
//! postcard, duet, spectrogram, handshake strip, notarize margins, mesh HTML.
//!
//! Honest: these are machine-readable demos, **not** steganography or encryption.

use crate::mge4::{generate_variants, select_variant_index};
use crate::svg::glyph_to_svg;
use crate::{generate_glyph, generate_project_mark};
use doldskrift::{Result, FONT_VERSION, PUA_BASE};
use serde::{Deserialize, Serialize};

/// Metadata embedded in ambient / flicker SVGs for lossless decode without vision ML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceCarrier {
    /// Surface kind.
    pub kind: String,
    /// Encoded PUA string (Open mode). Primary / channel A.
    pub encoded: String,
    /// Optional SHA-256 hex of semantic plaintext (integrity, not a signature).
    pub sha256: Option<String>,
    /// Epoch / seed label for living documents.
    pub epoch: Option<u64>,
    /// Protocol note (honesty).
    pub note: String,
    /// Optional second Open channel (duet channel B).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_b: Option<String>,
    /// Optional wire / capability payload (handshake strip).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wire: Option<String>,
}

/// Deterministic aurora gradient stops derived from a content hash seed (beautiful + machine-stable).
pub fn aurora_gradient_defs(seed: u64, id: &str) -> String {
    let h1 = mix64(seed, 0xA0110A);
    let h2 = mix64(seed, 0xB0220B);
    let h3 = mix64(seed, 0xC0330C);
    let c0 = hsl_teal(h1, 28, 14);
    let c1 = hsl_teal(h2, 48, 22);
    let c2 = hsl_teal(h3, 36, 10);
    let cx = 30 + (h1 % 40);
    let cy = 20 + (h2 % 35);
    format!(
        r##"<radialGradient id="{id}" cx="{cx}%" cy="{cy}%" r="78%">
      <stop offset="0%" stop-color="{c1}"/>
      <stop offset="55%" stop-color="{c0}"/>
      <stop offset="100%" stop-color="{c2}"/>
    </radialGradient>
    <linearGradient id="{id}-band" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="{c1}" stop-opacity="0.35"/>
      <stop offset="50%" stop-color="{c0}" stop-opacity="0.12"/>
      <stop offset="100%" stop-color="{c2}" stop-opacity="0.4"/>
    </linearGradient>"##
    )
}

fn hsl_teal(h: u64, sat: u32, light: u32) -> String {
    let hue = 150 + (h % 45);
    format!("hsl({hue},{sat}%,{light}%)")
}

/// Render an abstract “constellation” SVG that still carries the encoded PUA in metadata + stars.
///
/// When `animate` is true, stars get SMIL opacity/radius twinkle (decorative; decode uses carrier).
pub fn render_ambient_constellation(encoded: &str, seed: u64, sha256: Option<&str>) -> String {
    render_ambient_constellation_ex(encoded, seed, sha256, false)
}

/// Ambient constellation with optional SMIL animation (`dold ambient --animate`).
pub fn render_ambient_constellation_ex(
    encoded: &str,
    seed: u64,
    sha256: Option<&str>,
    animate: bool,
) -> String {
    let kind = if animate {
        "ambient-constellation-animated"
    } else {
        "ambient-constellation"
    };
    let carrier = SurfaceCarrier {
        kind: kind.into(),
        encoded: encoded.to_owned(),
        sha256: sha256.map(|s| s.to_owned()),
        epoch: None,
        note: "Open visual channel — not secrecy. Decode via dold ambient --decode.".into(),
        channel_b: None,
        wire: None,
    };
    let meta = serde_json::to_string(&carrier).unwrap_or_else(|_| "{}".into());
    let meta_esc = xml_escape(&meta);
    let mut stars = String::new();
    for (i, ch) in encoded.chars().enumerate() {
        let cp = ch as u32;
        let sid = if (PUA_BASE..PUA_BASE + 256).contains(&cp) {
            (cp - PUA_BASE) as u8
        } else {
            (cp as u8).wrapping_add(i as u8)
        };
        let h = mix64(seed, u64::from(sid) ^ ((i as u64) << 8));
        let x = 24.0 + ((h % 920) as f64);
        let y = 24.0 + (((h >> 10) % 520) as f64);
        let r = 1.5 + (((h >> 20) % 5) as f64) * 0.6;
        let hue = 160 + (h % 40);
        let anim = if animate {
            let dur = 2.0 + ((h % 30) as f64) * 0.12;
            let begin = ((h >> 5) % 20) as f64 * 0.15;
            format!(
                r##"<animate attributeName="opacity" values="0.35;0.95;0.35" dur="{dur:.2}s" begin="{begin:.2}s" repeatCount="indefinite"/>
  <animate attributeName="r" values="{r:.1};{r2:.1};{r:.1}" dur="{dur:.2}s" begin="{begin:.2}s" repeatCount="indefinite"/>"##,
                r2 = r * 1.55
            )
        } else {
            String::new()
        };
        stars.push_str(&format!(
            r#"<circle cx="{x:.1}" cy="{y:.1}" r="{r:.1}" fill="hsl({hue},42%,62%)" opacity="0.9" data-dsk-cp="U+{cp:04X}">{anim}</circle>"#
        ));
        stars.push('\n');
        // faint edges between consecutive stars
        if i > 0 {
            let prev = encoded.chars().nth(i - 1).unwrap() as u32;
            let ph = mix64(
                seed,
                u64::from(if (PUA_BASE..PUA_BASE + 256).contains(&prev) {
                    (prev - PUA_BASE) as u8
                } else {
                    prev as u8
                }) ^ (((i - 1) as u64) << 8),
            );
            let px = 24.0 + ((ph % 920) as f64);
            let py = 24.0 + (((ph >> 10) % 520) as f64);
            stars.push_str(&format!(
                r##"<line x1="{px:.1}" y1="{py:.1}" x2="{x:.1}" y2="{y:.1}" stroke="#0F6B5C" stroke-opacity="0.18" stroke-width="0.8"/>"##
            ));
            stars.push('\n');
        }
    }
    let mark = generate_project_mark(seed).ok();
    let mark_g = mark
        .as_ref()
        .map(|g| strip_svg_shell(&glyph_to_svg(g)))
        .unwrap_or_default();
    let aurora = aurora_gradient_defs(seed ^ mix64(seed, encoded.len() as u64), "aurora");
    let label = if animate {
        "ambient channel · Open · SMIL twinkle · not encryption"
    } else {
        "ambient channel · Open mode · aurora · not encryption"
    };

    format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="960" height="560" viewBox="0 0 960 560" role="img" aria-label="Doldskrift ambient constellation">
  <desc id="dsk-carrier">{meta_esc}</desc>
  <defs>
    {aurora}
  </defs>
  <rect width="100%" height="100%" fill="url(#aurora)"/>
  <rect width="100%" height="100%" fill="url(#aurora-band)"/>
  <g opacity="0.95">{stars}</g>
  <g transform="translate(18,18) scale(0.045)" stroke="#E8EFE9" fill="none" stroke-width="36">{mark_g}</g>
  <text x="72" y="42" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="11">{label}</text>
</svg>
"##
    )
}

/// Extract carrier JSON from an ambient/flicker SVG `<desc id="dsk-carrier">…</desc>`.
pub fn extract_carrier_from_svg(svg: &str) -> Option<SurfaceCarrier> {
    let key = r#"id="dsk-carrier""#;
    let idx = svg.find(key)?;
    let after = &svg[idx..];
    let start = after.find('>')? + 1;
    let end = after.find("</desc>")?;
    let raw = &after[start..end];
    let unescaped = xml_unescape(raw);
    serde_json::from_str(&unescaped).ok()
}

/// Animated SVG: glyphs appear in temporal order; machines read frame sequence metadata.
pub fn render_flicker_svg(encoded: &str, seed: u64) -> Result<String> {
    let catalog_seed = seed;
    let mut cells = String::new();
    let n = encoded.chars().count().max(1);
    let cell_w = 72i32;
    let width = (n as i32) * cell_w + 48;
    let duration = (n as f64) * 0.35 + 0.8;

    for (i, ch) in encoded.chars().enumerate() {
        let cp = ch as u32;
        let g = if (PUA_BASE..PUA_BASE + 256).contains(&cp) {
            generate_glyph(cp, FONT_VERSION, catalog_seed)?
        } else {
            generate_glyph(PUA_BASE + (cp % 256), FONT_VERSION, catalog_seed)?
        };
        let inner = strip_svg_shell(&glyph_to_svg(&g));
        let x = 24 + i as i32 * cell_w;
        let begin = i as f64 * 0.35;
        cells.push_str(&format!(
            r##"<g transform="translate({x},40) scale(0.06)" opacity="0" data-frame="{i}" data-dsk-cp="U+{cp:04X}">
  <animate attributeName="opacity" values="0;1;1;0" keyTimes="0;0.15;0.75;1" dur="{duration:.2}s" begin="{begin:.2}s" repeatCount="indefinite"/>
  <g stroke="#D8EFE6" fill="none" stroke-width="36">{inner}</g>
</g>
"##
        ));
    }

    let carrier = SurfaceCarrier {
        kind: "temporal-flicker".into(),
        encoded: encoded.to_owned(),
        sha256: None,
        epoch: Some(seed),
        note:
            "Timing/order is decorative for humans; decode uses embedded carrier. Not encryption."
                .into(),
        channel_b: None,
        wire: None,
    };
    let meta = xml_escape(&serde_json::to_string(&carrier).unwrap_or_else(|_| "{}".into()));

    let brand = generate_project_mark(catalog_seed)
        .ok()
        .map(|g| strip_svg_shell(&glyph_to_svg(&g)))
        .unwrap_or_default();

    Ok(format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="160" viewBox="0 0 {width} 160" role="img" aria-label="Doldskrift temporal flicker">
  <desc id="dsk-carrier">{meta}</desc>
  <rect width="100%" height="100%" fill="#121614"/>
  <g transform="translate({mark_x},8) scale(0.035)" stroke="#7aa897" fill="none" stroke-width="36" opacity="0.5">{brand}</g>
  <text x="16" y="24" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="11">temporal flicker · frame order = glyph sequence · Open</text>
  {cells}
</svg>
"##,
        mark_x = (width - 48).max(16)
    ))
}

/// Living document: same plaintext, epoch-varying MGE/4 visual variants; Open decode still works.
pub fn render_living_specimen(encoded: &str, epoch: u64) -> Result<(String, Vec<u8>)> {
    let mut svg = format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="150" viewBox="0 0 {w} 150">
  <desc id="dsk-carrier">{meta}</desc>
  <rect width="100%" height="100%" fill="#121614"/>
  <text x="12" y="138" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="10">living document · epoch {epoch} · Open recoverability preserved</text>
"##,
        w = encoded.chars().count() as i32 * 80 + 40,
        meta = xml_escape(
            &serde_json::to_string(&SurfaceCarrier {
                kind: "living-document".into(),
                encoded: encoded.to_owned(),
                sha256: None,
                epoch: Some(epoch),
                note: "Visual variants change with epoch; DeterministicBaseline / Open decode recover plaintext.".into(),
                channel_b: None,
                wire: None,
            })
            .unwrap_or_else(|_| "{}".into())
        )
    );

    for (i, ch) in encoded.chars().enumerate() {
        let cp = ch as u32;
        let symbol = if (PUA_BASE..PUA_BASE + 256).contains(&cp) {
            (cp - PUA_BASE) as u8
        } else {
            continue;
        };
        let variants = generate_variants(symbol, 1, 4)?;
        let idx = select_variant_index(symbol, i as u32, epoch, 0x4C49_5645, 4) as usize;
        let g = variants
            .get(idx)
            .map(|v| v.geometry.clone())
            .unwrap_or_else(|| generate_glyph(cp, FONT_VERSION, 1).unwrap());
        let x = 20 + i as i32 * 80;
        let inner = strip_svg_shell(&glyph_to_svg(&g));
        svg.push_str(&format!(
            r##"<g transform="translate({x},12) scale(0.08)" stroke="#D8EFE6" fill="none">{inner}</g>"##
        ));
        svg.push('\n');
    }
    svg.push_str("</svg>\n");
    Ok((svg, encoded.as_bytes().to_vec()))
}

/// Constellation-style neural mesh map from compound member ids (clickable nodes via title).
pub fn render_constellation_map(
    nodes: &[(String, f64, f64)],
    edges: &[(usize, usize)],
    title: &str,
) -> String {
    let mut marks = String::new();
    for (a, b) in edges {
        if let (Some(na), Some(nb)) = (nodes.get(*a), nodes.get(*b)) {
            marks.push_str(&format!(
                r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#0F6B5C" stroke-opacity="0.35" stroke-width="1.2"/>"##,
                na.1, na.2, nb.1, nb.2
            ));
            marks.push('\n');
        }
    }
    for (id, x, y) in nodes {
        marks.push_str(&format!(
            r##"<g transform="translate({x:.1},{y:.1})"><circle r="7" fill="#0F6B5C"/><circle r="3" fill="#E8EFE9"/><title>{tid}</title></g>"##,
            tid = xml_escape(id)
        ));
        marks.push('\n');
    }
    let brand = generate_project_mark(1)
        .ok()
        .map(|g| strip_svg_shell(&glyph_to_svg(&g)))
        .unwrap_or_default();
    let aurora = aurora_gradient_defs(0x4D45_5348 ^ (nodes.len() as u64), "mesh-aurora");
    format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="640" height="360" viewBox="0 0 640 360" role="img" aria-label="{title}">
  <defs>{aurora}</defs>
  <rect width="100%" height="100%" fill="url(#mesh-aurora)"/>
  <g transform="translate(582,12) scale(0.04)" stroke="#7aa897" fill="none" stroke-width="36" opacity="0.55">{brand}</g>
  <text x="16" y="28" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="12">{title_esc}</text>
  {marks}
</svg>
"##,
        title_esc = xml_escape(title)
    )
}

/// Machine postcard: constellation field + short glyph strip + SHA-256 echo digest (integrity, not a signature).
pub fn render_postcard_svg(encoded: &str, seed: u64, sha256_hex: &str) -> String {
    let carrier = SurfaceCarrier {
        kind: "machine-postcard".into(),
        encoded: encoded.to_owned(),
        sha256: Some(sha256_hex.to_owned()),
        epoch: Some(seed),
        note: "Machine postcard — Open payload + hash digest. Read with dold postcard --read. Not a signature.".into(),
        channel_b: None,
        wire: None,
    };
    let meta = xml_escape(&serde_json::to_string(&carrier).unwrap_or_else(|_| "{}".into()));
    let aurora = aurora_gradient_defs(seed, "pc-aurora");
    let mut stars = String::new();
    for (i, ch) in encoded.chars().enumerate().take(48) {
        let cp = ch as u32;
        let sid = if (PUA_BASE..PUA_BASE + 256).contains(&cp) {
            (cp - PUA_BASE) as u8
        } else {
            cp as u8
        };
        let h = mix64(seed, u64::from(sid) ^ ((i as u64) << 8));
        let x = 40.0 + ((h % 620) as f64);
        let y = 56.0 + (((h >> 10) % 200) as f64);
        let r = 1.8 + (((h >> 20) % 4) as f64) * 0.5;
        stars.push_str(&format!(
            r##"<circle cx="{x:.1}" cy="{y:.1}" r="{r:.1}" fill="#7ec8b3" opacity="0.85" data-dsk-cp="U+{cp:04X}"/>"##
        ));
    }
    let mut strip = String::new();
    for (i, ch) in encoded.chars().enumerate().take(24) {
        let cp = ch as u32;
        let g = generate_glyph(
            if (PUA_BASE..PUA_BASE + 256).contains(&cp) {
                cp
            } else {
                PUA_BASE + (cp % 256)
            },
            FONT_VERSION,
            seed,
        )
        .ok();
        if let Some(g) = g {
            let inner = strip_svg_shell(&glyph_to_svg(&g));
            let x = 24 + i as i32 * 28;
            strip.push_str(&format!(
                r##"<g transform="translate({x},300) scale(0.028)" stroke="#D8EFE6" fill="none" stroke-width="36">{inner}</g>"##
            ));
        }
    }
    // Digest nibbles as tiny margin ticks (visual echo of hash — not crypto proof)
    let mut digest_marks = String::new();
    for (i, c) in sha256_hex.chars().take(32).enumerate() {
        let nibble = c.to_digit(16).unwrap_or(0);
        let h = 8 + nibble * 3;
        let x = 24 + i as i32 * 10;
        digest_marks.push_str(&format!(
            r##"<rect x="{x}" y="{y}" width="6" height="{h}" fill="#0F6B5C" opacity="0.75" data-nibble="{c}"/>"##,
            y = 380 - h as i32
        ));
    }
    let brand = generate_project_mark(seed)
        .ok()
        .map(|g| strip_svg_shell(&glyph_to_svg(&g)))
        .unwrap_or_default();
    let digest_short = if sha256_hex.len() >= 12 {
        &sha256_hex[..12]
    } else {
        sha256_hex
    };
    format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="720" height="400" viewBox="0 0 720 400" role="img" aria-label="Doldskrift machine postcard">
  <desc id="dsk-carrier">{meta}</desc>
  <defs>{aurora}</defs>
  <rect width="100%" height="100%" rx="6" fill="url(#pc-aurora)"/>
  <rect x="12" y="12" width="696" height="376" rx="4" fill="none" stroke="#7aa897" stroke-opacity="0.35"/>
  <g transform="translate(640,20) scale(0.04)" stroke="#E8EFE9" fill="none" stroke-width="36">{brand}</g>
  <text x="28" y="36" fill="#c5ddd4" font-family="IBM Plex Mono, monospace" font-size="13">doldskrift · machine postcard 🇸🇪</text>
  <text x="28" y="54" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="10">Open channel · sha256:{digest_short}… · not a signature</text>
  <g>{stars}</g>
  <g>{strip}</g>
  <g>{digest_marks}</g>
  <text x="28" y="392" fill="#5a7a6e" font-family="IBM Plex Mono, monospace" font-size="9">read: dold postcard --read · Maskinskriven betydelse — inte hemlighet.</text>
</svg>
"##
    )
}

/// Two interleaved Open glyph streams (A/B) that decode independently.
pub fn render_duet_svg(encoded_a: &str, encoded_b: &str, seed: u64) -> Result<String> {
    let carrier = SurfaceCarrier {
        kind: "duet-channels".into(),
        encoded: encoded_a.to_owned(),
        sha256: None,
        epoch: Some(seed),
        note: "Duet A/B — two Open channels interleaved visually; decode independently. Not encryption.".into(),
        channel_b: Some(encoded_b.to_owned()),
        wire: None,
    };
    let meta = xml_escape(&serde_json::to_string(&carrier).unwrap_or_else(|_| "{}".into()));
    let n = encoded_a
        .chars()
        .count()
        .max(encoded_b.chars().count())
        .max(1);
    let width = (n as i32) * 56 + 48;
    let mut cells = String::new();
    for i in 0..n {
        let x = 24 + i as i32 * 56;
        if let Some(ch) = encoded_a.chars().nth(i) {
            let cp = ch as u32;
            let g = generate_glyph(
                if (PUA_BASE..PUA_BASE + 256).contains(&cp) {
                    cp
                } else {
                    PUA_BASE + (cp % 256)
                },
                FONT_VERSION,
                seed,
            )?;
            let inner = strip_svg_shell(&glyph_to_svg(&g));
            cells.push_str(&format!(
                r##"<g transform="translate({x},36) scale(0.045)" stroke="#7ec8b3" fill="none" stroke-width="36" data-channel="A" data-dsk-cp="U+{cp:04X}"><title>A</title>{inner}</g>"##
            ));
        }
        if let Some(ch) = encoded_b.chars().nth(i) {
            let cp = ch as u32;
            let g = generate_glyph(
                if (PUA_BASE..PUA_BASE + 256).contains(&cp) {
                    cp
                } else {
                    PUA_BASE + (cp % 256)
                },
                FONT_VERSION,
                seed.wrapping_add(1),
            )?;
            let inner = strip_svg_shell(&glyph_to_svg(&g));
            cells.push_str(&format!(
                r##"<g transform="translate({x},110) scale(0.045)" stroke="#c9a87c" fill="none" stroke-width="36" data-channel="B" data-dsk-cp="U+{cp:04X}"><title>B</title>{inner}</g>"##
            ));
        }
        // interleave connector
        cells.push_str(&format!(
            r##"<line x1="{x}" y1="88" x2="{x2}" y2="108" stroke="#0F6B5C" stroke-opacity="0.25" stroke-width="1"/>"##,
            x = x + 18,
            x2 = x + 18
        ));
    }
    let aurora = aurora_gradient_defs(seed ^ 0xD0E7, "duet-aurora");
    Ok(format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="200" viewBox="0 0 {width} 200" role="img" aria-label="Doldskrift duet channels">
  <desc id="dsk-carrier">{meta}</desc>
  <defs>{aurora}</defs>
  <rect width="100%" height="100%" fill="url(#duet-aurora)"/>
  <text x="16" y="22" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="11">duet · channel A (teal) · channel B (warm) · independent Open decode</text>
  {cells}
</svg>
"##
    ))
}

/// Spectrogram-style strip: symbol id → vertical frequency-like bars; carrier preserves round-trip.
pub fn render_spectrogram_svg(encoded: &str, seed: u64) -> String {
    let carrier = SurfaceCarrier {
        kind: "spectrogram-strip".into(),
        encoded: encoded.to_owned(),
        sha256: None,
        epoch: Some(seed),
        note: "Spectrogram-style Open strip — bar height from symbol id. Decode via carrier. Not audio / not encryption.".into(),
        channel_b: None,
        wire: None,
    };
    let meta = xml_escape(&serde_json::to_string(&carrier).unwrap_or_else(|_| "{}".into()));
    let n = encoded.chars().count().max(1);
    let width = (n as i32) * 10 + 48;
    let height = 160;
    let mut bars = String::new();
    for (i, ch) in encoded.chars().enumerate() {
        let cp = ch as u32;
        let sid = if (PUA_BASE..PUA_BASE + 256).contains(&cp) {
            (cp - PUA_BASE) as u8
        } else {
            cp as u8
        };
        let h = 12 + (u32::from(sid) * 120 / 255) as i32;
        let x = 24 + i as i32 * 10;
        let y = height - 28 - h;
        let hue = 150 + (mix64(seed, u64::from(sid)) % 40);
        bars.push_str(&format!(
            r#"<rect x="{x}" y="{y}" width="7" height="{h}" fill="hsl({hue},45%,55%)" opacity="0.9" data-dsk-cp="U+{cp:04X}" data-sid="{sid}"/>"#
        ));
    }
    let aurora = aurora_gradient_defs(seed, "spec-aurora");
    format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" role="img" aria-label="Doldskrift spectrogram strip">
  <desc id="dsk-carrier">{meta}</desc>
  <defs>{aurora}</defs>
  <rect width="100%" height="100%" fill="url(#spec-aurora)"/>
  <text x="16" y="18" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="10">spectrogram strip · symbol-id → bar height · Open carrier</text>
  {bars}
</svg>
"##
    )
}

/// Visual Open capability negotiation strip (agents can scan the carrier / wire text).
pub fn render_handshake_strip(wire: &str, modes_csv: &str, seed: u64) -> String {
    let encoded_modes = modes_csv
        .bytes()
        .map(|b| char::from_u32(PUA_BASE + u32::from(b)).unwrap_or('?'))
        .collect::<String>();
    let carrier = SurfaceCarrier {
        kind: "handshake-strip".into(),
        encoded: encoded_modes.clone(),
        sha256: None,
        epoch: Some(seed),
        note: "Open capability strip — representation negotiation only. Not a secure channel / not encryption.".into(),
        channel_b: None,
        wire: Some(wire.to_owned()),
    };
    let meta = xml_escape(&serde_json::to_string(&carrier).unwrap_or_else(|_| "{}".into()));
    let mut cells = String::new();
    for (i, ch) in encoded_modes.chars().enumerate().take(40) {
        let cp = ch as u32;
        if let Ok(g) = generate_glyph(cp, FONT_VERSION, seed) {
            let inner = strip_svg_shell(&glyph_to_svg(&g));
            let x = 16 + i as i32 * 22;
            cells.push_str(&format!(
                r##"<g transform="translate({x},40) scale(0.022)" stroke="#D8EFE6" fill="none" stroke-width="36">{inner}</g>"##
            ));
        }
    }
    // Cap pills as colored ticks from modes string
    let mut pills = String::new();
    for (i, part) in modes_csv.split(',').filter(|s| !s.is_empty()).enumerate() {
        let x = 16 + i as i32 * 92;
        pills.push_str(&format!(
            r##"<rect x="{x}" y="100" width="84" height="22" rx="3" fill="#0F6B5C" opacity="0.55"/><text x="{tx}" y="115" fill="#E8EFE9" font-family="IBM Plex Mono, monospace" font-size="10">{label}</text>"##,
            tx = x + 8,
            label = xml_escape(part.trim())
        ));
    }
    let width = (encoded_modes.chars().count().min(40) as i32 * 22 + 48).max(360);
    format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="140" viewBox="0 0 {width} 140" role="img" aria-label="Doldskrift handshake strip">
  <desc id="dsk-carrier">{meta}</desc>
  <rect width="100%" height="100%" fill="#121614"/>
  <text x="16" y="22" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="11">handshake strip · Open caps · not a secure channel</text>
  {cells}
  {pills}
</svg>
"##
    )
}

/// Bind payload SHA-256 into glyph margin marks (integrity notarize — not a cryptographic signature).
pub fn render_notarize_svg(encoded: &str, sha256_hex: &str, seed: u64) -> Result<String> {
    let carrier = SurfaceCarrier {
        kind: "notarize-margin".into(),
        encoded: encoded.to_owned(),
        sha256: Some(sha256_hex.to_owned()),
        epoch: Some(seed),
        note: "Round-trip notarize — SHA-256 bound into margin marks. Verify with dold notarize --verify. Not a signature.".into(),
        channel_b: None,
        wire: None,
    };
    let meta = xml_escape(&serde_json::to_string(&carrier).unwrap_or_else(|_| "{}".into()));
    let n = encoded.chars().count().max(1);
    let width = (n as i32) * 64 + 80;
    let mut glyphs = String::new();
    for (i, ch) in encoded.chars().enumerate() {
        let cp = ch as u32;
        let g = generate_glyph(
            if (PUA_BASE..PUA_BASE + 256).contains(&cp) {
                cp
            } else {
                PUA_BASE + (cp % 256)
            },
            FONT_VERSION,
            seed,
        )?;
        let inner = strip_svg_shell(&glyph_to_svg(&g));
        let x = 40 + i as i32 * 64;
        glyphs.push_str(&format!(
            r##"<g transform="translate({x},28) scale(0.055)" stroke="#D8EFE6" fill="none" stroke-width="36">{inner}</g>"##
        ));
    }
    // Margin marks: each hash nibble → small north tick
    let mut marks = String::new();
    for (i, c) in sha256_hex.chars().enumerate().take(n.clamp(16, 64)) {
        let nib = c.to_digit(16).unwrap_or(0) as i32;
        let x = 36 + (i as i32 % n.max(1) as i32) * 64 + nib;
        let y = 12 + (i as i32 / n.max(1) as i32) * 4;
        marks.push_str(&format!(
            r##"<rect x="{x}" y="{y}" width="3" height="{h}" fill="#c9a87c" data-nibble="{c}"/>"##,
            h = 6 + nib
        ));
    }
    Ok(format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="140" viewBox="0 0 {width} 140" role="img" aria-label="Doldskrift notarize specimen">
  <desc id="dsk-carrier">{meta}</desc>
  <rect width="100%" height="100%" fill="#121614"/>
  <text x="12" y="128" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="10">notarize · SHA-256 margin marks · integrity ≠ signature</text>
  {marks}
  {glyphs}
</svg>
"##
    ))
}

/// Radial / symmetric machine-glyph mandala that still carries Open PUA for decode.
pub fn render_kaleidoscope_svg(encoded: &str, seed: u64) -> Result<String> {
    let carrier = SurfaceCarrier {
        kind: "kaleidoscope-mandala".into(),
        encoded: encoded.to_owned(),
        sha256: None,
        epoch: Some(seed),
        note: "Kaleidoscope mandala — Open channel with radial glyph symmetry. Decode via dold kaleidoscope --decode. Not encryption.".into(),
        channel_b: None,
        wire: None,
    };
    let meta = xml_escape(&serde_json::to_string(&carrier).unwrap_or_else(|_| "{}".into()));
    let n = encoded.chars().count().max(1);
    let sectors = 6u32;
    let mut petals = String::new();
    for (i, ch) in encoded.chars().enumerate().take(32) {
        let cp = ch as u32;
        let g = generate_glyph(
            if (PUA_BASE..PUA_BASE + 256).contains(&cp) {
                cp
            } else {
                PUA_BASE + (cp % 256)
            },
            FONT_VERSION,
            seed ^ (i as u64),
        )?;
        let inner = strip_svg_shell(&glyph_to_svg(&g));
        let ring = 70.0 + (i as f64 % 4.0) * 42.0;
        let base_ang = (i as f64 / n as f64) * std::f64::consts::TAU;
        for s in 0..sectors {
            let ang = base_ang + (s as f64) * (std::f64::consts::TAU / f64::from(sectors));
            let x = 320.0 + ang.cos() * ring;
            let y = 320.0 + ang.sin() * ring;
            let deg = ang.to_degrees();
            petals.push_str(&format!(
                r##"<g transform="translate({x:.1},{y:.1}) rotate({deg:.1}) scale(0.032)" stroke="#D8EFE6" fill="none" stroke-width="36" data-dsk-cp="U+{cp:04X}" data-sector="{s}">{inner}</g>"##
            ));
            petals.push('\n');
        }
    }
    let brand = generate_project_mark(seed)
        .ok()
        .map(|g| strip_svg_shell(&glyph_to_svg(&g)))
        .unwrap_or_default();
    let aurora = aurora_gradient_defs(seed ^ 0x4B41_4C45, "kal-aurora");
    Ok(format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="640" height="640" viewBox="0 0 640 640" role="img" aria-label="Doldskrift kaleidoscope mandala">
  <desc id="dsk-carrier">{meta}</desc>
  <defs>{aurora}</defs>
  <rect width="100%" height="100%" fill="url(#kal-aurora)"/>
  <circle cx="320" cy="320" r="290" fill="none" stroke="#0F6B5C" stroke-opacity="0.35" stroke-width="1.5"/>
  <circle cx="320" cy="320" r="180" fill="none" stroke="#7aa897" stroke-opacity="0.2" stroke-width="1"/>
  <g>{petals}</g>
  <g transform="translate(292,292) scale(0.055)" stroke="#E8EFE9" fill="none" stroke-width="36">{brand}</g>
  <text x="24" y="28" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="11">kaleidoscope · 6-fold Open mandala · not encryption</text>
  <text x="24" y="620" fill="#5a7a6e" font-family="IBM Plex Mono, monospace" font-size="9">decode: dold kaleidoscope --decode · Maskinskriven betydelse — inte hemlighet.</text>
</svg>
"##
    ))
}

/// Multi-epoch living strip in one SVG — scrub via carrier epoch / `dold timeline --epoch`.
pub fn render_timeline_svg(encoded: &str, epochs: &[u64]) -> Result<String> {
    let epochs = if epochs.is_empty() {
        vec![1, 2, 3]
    } else {
        epochs.to_vec()
    };
    let primary = epochs[0];
    let carrier = SurfaceCarrier {
        kind: "timeline-strip".into(),
        encoded: encoded.to_owned(),
        sha256: None,
        epoch: Some(primary),
        note: "Timeline strip — multiple living epochs in one SVG. Scrub with dold timeline --epoch. Open recoverability preserved.".into(),
        channel_b: Some(
            epochs
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ),
        wire: None,
    };
    let meta = xml_escape(&serde_json::to_string(&carrier).unwrap_or_else(|_| "{}".into()));
    let n = encoded.chars().count().clamp(1, 16);
    let row_h = 110i32;
    let width = (n as i32) * 56 + 48;
    let height = 48 + epochs.len() as i32 * row_h;
    let mut rows = String::new();
    for (ri, &epoch) in epochs.iter().enumerate() {
        let y0 = 36 + ri as i32 * row_h;
        rows.push_str(&format!(
            r##"<text x="12" y="{ty}" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="10">epoch {epoch}</text>"##,
            ty = y0 + 12
        ));
        for (i, ch) in encoded.chars().enumerate().take(n) {
            let cp = ch as u32;
            let symbol = if (PUA_BASE..PUA_BASE + 256).contains(&cp) {
                (cp - PUA_BASE) as u8
            } else {
                continue;
            };
            let variants = generate_variants(symbol, 1, 4)?;
            let idx = select_variant_index(symbol, i as u32, epoch, 0x5449_4D45, 4) as usize;
            let g = variants
                .get(idx)
                .map(|v| v.geometry.clone())
                .unwrap_or_else(|| generate_glyph(cp, FONT_VERSION, 1).unwrap());
            let x = 20 + i as i32 * 56;
            let inner = strip_svg_shell(&glyph_to_svg(&g));
            rows.push_str(&format!(
                r##"<g transform="translate({x},{y}) scale(0.055)" stroke="#D8EFE6" fill="none" stroke-width="36" data-epoch="{epoch}">{inner}</g>"##,
                y = y0 + 18
            ));
        }
        rows.push('\n');
    }
    Ok(format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" role="img" aria-label="Doldskrift timeline strip">
  <desc id="dsk-carrier">{meta}</desc>
  <rect width="100%" height="100%" fill="#121614"/>
  <text x="12" y="22" fill="#c5ddd4" font-family="IBM Plex Mono, monospace" font-size="12">timeline · multi-epoch living strip · Open</text>
  {rows}
</svg>
"##
    ))
}

/// Visual “wax seal” compound mark binding payload digest (integrity only — not a crypto signature).
pub fn render_seal_svg(encoded: &str, sha256_hex: &str, seed: u64) -> Result<String> {
    let carrier = SurfaceCarrier {
        kind: "wax-seal".into(),
        encoded: encoded.to_owned(),
        sha256: Some(sha256_hex.to_owned()),
        epoch: Some(seed),
        note: "Wax seal — SHA-256 bound into seal geometry. Verify with dold seal --verify. Integrity ≠ cryptographic signature.".into(),
        channel_b: None,
        wire: None,
    };
    let meta = xml_escape(&serde_json::to_string(&carrier).unwrap_or_else(|_| "{}".into()));
    let mut ring = String::new();
    for (i, c) in sha256_hex.chars().take(32).enumerate() {
        let nib = c.to_digit(16).unwrap_or(0) as f64;
        let ang = (i as f64 / 32.0) * std::f64::consts::TAU;
        let r = 88.0 + nib * 2.2;
        let x = 200.0 + ang.cos() * r;
        let y = 200.0 + ang.sin() * r;
        ring.push_str(&format!(
            r##"<circle cx="{x:.1}" cy="{y:.1}" r="{rad:.1}" fill="#8b3a3a" opacity="0.85" data-nibble="{c}"/>"##,
            rad = 2.5 + nib * 0.35
        ));
    }
    let mut glyphs = String::new();
    for (i, ch) in encoded.chars().enumerate().take(8) {
        let cp = ch as u32;
        let g = generate_glyph(
            if (PUA_BASE..PUA_BASE + 256).contains(&cp) {
                cp
            } else {
                PUA_BASE + (cp % 256)
            },
            FONT_VERSION,
            seed,
        )?;
        let inner = strip_svg_shell(&glyph_to_svg(&g));
        let ang = (i as f64 / 8.0) * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2;
        let x = 200.0 + ang.cos() * 48.0;
        let y = 200.0 + ang.sin() * 48.0;
        glyphs.push_str(&format!(
            r##"<g transform="translate({x:.1},{y:.1}) scale(0.028)" stroke="#f0e6d8" fill="none" stroke-width="36">{inner}</g>"##
        ));
    }
    let brand = generate_project_mark(seed)
        .ok()
        .map(|g| strip_svg_shell(&glyph_to_svg(&g)))
        .unwrap_or_default();
    let digest_short = if sha256_hex.len() >= 12 {
        &sha256_hex[..12]
    } else {
        sha256_hex
    };
    Ok(format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="400" height="400" viewBox="0 0 400 400" role="img" aria-label="Doldskrift wax seal">
  <desc id="dsk-carrier">{meta}</desc>
  <defs>
    <radialGradient id="wax" cx="45%" cy="40%" r="60%">
      <stop offset="0%" stop-color="#c45c4a"/>
      <stop offset="55%" stop-color="#8b3a3a"/>
      <stop offset="100%" stop-color="#5a2424"/>
    </radialGradient>
  </defs>
  <rect width="100%" height="100%" fill="#121614"/>
  <circle cx="200" cy="200" r="120" fill="url(#wax)" stroke="#6b2e2e" stroke-width="3"/>
  <circle cx="200" cy="200" r="108" fill="none" stroke="#f0e6d8" stroke-opacity="0.25" stroke-width="1.5" stroke-dasharray="4 3"/>
  {ring}
  {glyphs}
  <g transform="translate(172,172) scale(0.055)" stroke="#f0e6d8" fill="none" stroke-width="36">{brand}</g>
  <text x="24" y="28" fill="#c9a87c" font-family="IBM Plex Mono, monospace" font-size="11">wax seal · sha256:{digest_short}… · integrity ≠ signature</text>
  <text x="24" y="384" fill="#5a7a6e" font-family="IBM Plex Mono, monospace" font-size="9">verify: dold seal --verify · Open payload bound to digest</text>
</svg>
"##
    ))
}

/// Interactive HTML one-pager for a mesh constellation (node inspection via click).
pub fn render_mesh_html(
    nodes: &[(String, f64, f64)],
    edges: &[(usize, usize)],
    title: &str,
    payload_note: &str,
) -> String {
    let svg = render_constellation_map(nodes, edges, title);
    let node_json = serde_json::to_string(
        &nodes
            .iter()
            .enumerate()
            .map(|(i, (id, x, y))| serde_json::json!({ "i": i, "id": id, "x": x, "y": y }))
            .collect::<Vec<_>>(),
    )
    .unwrap_or_else(|_| "[]".into());
    format!(
        r##"<!doctype html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{title_esc} — Doldskrift mesh</title>
  <style>
    :root {{ --ink:#E8EFE9; --mute:#7aa897; --bg:#0b0e0d; --accent:#0F6B5C; }}
    body {{ margin:0; font-family: "IBM Plex Mono", ui-monospace, monospace; background:var(--bg); color:var(--ink); }}
    header {{ padding:1.25rem 1.5rem 0.5rem; }}
    h1 {{ font-size:1.15rem; margin:0 0 0.35rem; font-weight:600; }}
    .sub {{ color:var(--mute); font-size:0.85rem; max-width:42rem; }}
    .flag {{ font-style:normal; }}
    main {{ display:grid; grid-template-columns: 1fr 280px; gap:1rem; padding:1rem 1.5rem 2rem; }}
    @media (max-width:800px) {{ main {{ grid-template-columns:1fr; }} }}
    .stage {{ border:1px solid rgba(15,107,92,0.35); border-radius:6px; overflow:auto; background:#0a0c0b; }}
    .stage svg {{ display:block; width:100%; height:auto; cursor:crosshair; }}
    aside {{ border:1px solid rgba(15,107,92,0.35); border-radius:6px; padding:1rem; min-height:12rem; }}
    aside h2 {{ margin:0 0 0.5rem; font-size:0.9rem; color:var(--mute); }}
    #inspect {{ white-space:pre-wrap; font-size:0.8rem; line-height:1.45; }}
    .note {{ margin-top:1rem; color:var(--mute); font-size:0.75rem; }}
  </style>
</head>
<body>
  <header>
    <h1>Doldskrift mesh <span class="flag" aria-label="Sweden">🇸🇪</span></h1>
    <p class="sub">{title_esc} — click a node to inspect. Neural constellation from LSG plan (Open/Deterministic path). Not encryption.</p>
  </header>
  <main>
    <div class="stage" id="stage">{svg_body}</div>
    <aside>
      <h2>Node inspect</h2>
      <div id="inspect">Click a constellation node…</div>
      <p class="note">{note_esc}</p>
    </aside>
  </main>
  <script>
    const nodes = {node_json};
    const el = document.getElementById("inspect");
    document.getElementById("stage").addEventListener("click", (ev) => {{
      const g = ev.target.closest("g");
      const title = g && g.querySelector("title");
      if (!title) return;
      const id = title.textContent || "";
      const hit = nodes.find((n) => n.id === id);
      el.textContent = hit
        ? `id: ${{hit.id}}\nindex: ${{hit.i}}\nx: ${{hit.x.toFixed(1)}}\ny: ${{hit.y.toFixed(1)}}\n\nLSG node · inspect only`
        : `id: ${{id}}\n(not in export table)`;
    }});
  </script>
</body>
</html>
"##,
        title_esc = xml_escape(title),
        note_esc = xml_escape(payload_note),
        svg_body = svg
            .lines()
            .filter(|l| !l.trim_start().starts_with("<?xml"))
            .collect::<Vec<_>>()
            .join("\n"),
        node_json = node_json
    )
}

fn strip_svg_shell(svg: &str) -> String {
    svg.lines()
        .filter(|l| {
            let t = l.trim_start();
            !t.starts_with("<?xml") && !t.starts_with("<svg") && !t.starts_with("</svg>")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn xml_unescape(s: &str) -> String {
    s.replace("&quot;", "\"")
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&amp;", "&")
}

fn mix64(seed: u64, x: u64) -> u64 {
    let mut h = seed ^ x.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
    h ^= h >> 33;
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use doldskrift::encode;

    #[test]
    fn ambient_roundtrip_carrier() {
        let enc = encode("hello ambient").unwrap();
        let svg = render_ambient_constellation(&enc, 7, Some("deadbeef"));
        let c = extract_carrier_from_svg(&svg).expect("carrier");
        assert_eq!(c.kind, "ambient-constellation");
        assert_eq!(c.encoded, enc);
        assert_eq!(c.sha256.as_deref(), Some("deadbeef"));
        assert!(svg.contains("aurora"));
    }

    #[test]
    fn flicker_has_carrier() {
        let enc = encode("flick").unwrap();
        let svg = render_flicker_svg(&enc, 3).unwrap();
        let c = extract_carrier_from_svg(&svg).unwrap();
        assert_eq!(c.kind, "temporal-flicker");
        assert_eq!(c.encoded, enc);
    }

    #[test]
    fn postcard_roundtrip() {
        let enc = encode("post").unwrap();
        let svg = render_postcard_svg(&enc, 2, "abcdef0123456789");
        let c = extract_carrier_from_svg(&svg).unwrap();
        assert_eq!(c.kind, "machine-postcard");
        assert_eq!(c.encoded, enc);
        assert_eq!(c.sha256.as_deref(), Some("abcdef0123456789"));
    }

    #[test]
    fn duet_channels_independent() {
        let a = encode("alpha").unwrap();
        let b = encode("beta").unwrap();
        let svg = render_duet_svg(&a, &b, 1).unwrap();
        let c = extract_carrier_from_svg(&svg).unwrap();
        assert_eq!(c.kind, "duet-channels");
        assert_eq!(c.encoded, a);
        assert_eq!(c.channel_b.as_deref(), Some(b.as_str()));
    }

    #[test]
    fn spectrogram_and_notarize_carriers() {
        let enc = encode("spec").unwrap();
        let s = render_spectrogram_svg(&enc, 4);
        assert_eq!(
            extract_carrier_from_svg(&s).unwrap().kind,
            "spectrogram-strip"
        );
        let n = render_notarize_svg(&enc, "deadbeef", 1).unwrap();
        let c = extract_carrier_from_svg(&n).unwrap();
        assert_eq!(c.kind, "notarize-margin");
        assert_eq!(c.sha256.as_deref(), Some("deadbeef"));
    }

    #[test]
    fn handshake_strip_embeds_wire() {
        let svg = render_handshake_strip("DSK?\nversions=1\n", "encoded,session", 1);
        let c = extract_carrier_from_svg(&svg).unwrap();
        assert_eq!(c.kind, "handshake-strip");
        assert!(c.wire.as_deref().unwrap_or("").contains("DSK?"));
    }

    #[test]
    fn mesh_html_contains_nodes() {
        let nodes = vec![("n0".into(), 100.0, 100.0), ("n1".into(), 200.0, 120.0)];
        let html = render_mesh_html(&nodes, &[(0, 1)], "test mesh", "note");
        assert!(html.contains("n0"));
        assert!(html.contains("Node inspect"));
        assert!(html.contains("🇸🇪"));
    }

    #[test]
    fn kaleidoscope_roundtrip() {
        let enc = encode("kal").unwrap();
        let svg = render_kaleidoscope_svg(&enc, 5).unwrap();
        let c = extract_carrier_from_svg(&svg).unwrap();
        assert_eq!(c.kind, "kaleidoscope-mandala");
        assert_eq!(c.encoded, enc);
        assert!(svg.contains("6-fold"));
    }

    #[test]
    fn timeline_multi_epoch() {
        let enc = encode("time").unwrap();
        let svg = render_timeline_svg(&enc, &[1, 7, 13]).unwrap();
        let c = extract_carrier_from_svg(&svg).unwrap();
        assert_eq!(c.kind, "timeline-strip");
        assert_eq!(c.encoded, enc);
        assert_eq!(c.channel_b.as_deref(), Some("1,7,13"));
        assert!(svg.contains("epoch 7"));
    }

    #[test]
    fn seal_binds_digest() {
        let enc = encode("seal").unwrap();
        let svg = render_seal_svg(&enc, "abcdef0123456789deadbeef", 2).unwrap();
        let c = extract_carrier_from_svg(&svg).unwrap();
        assert_eq!(c.kind, "wax-seal");
        assert_eq!(c.sha256.as_deref(), Some("abcdef0123456789deadbeef"));
        assert!(svg.contains("integrity ≠ signature") || svg.contains("integrity"));
    }

    #[test]
    fn ambient_animate_carrier() {
        let enc = encode("twinkle").unwrap();
        let svg = render_ambient_constellation_ex(&enc, 1, None, true);
        let c = extract_carrier_from_svg(&svg).unwrap();
        assert_eq!(c.kind, "ambient-constellation-animated");
        assert!(svg.contains("<animate"));
    }
}
