//! Unexpected CLI surfaces: radio, echo, museum, ambient, flicker, living,
//! postcard, duet, spectrogram, notarize, handshake visual, mesh HTML.

use doldskrift::neural::{
    compose_deterministic, DeterministicBaseline, GrammarEpoch, ObservationGraph, ReaderBackend,
};
use doldskrift::semantic::Value;
use doldskrift::{decode, encode, DskDocument, Mode, Session, DSK_PROJECT_MARK};
use doldskrift_font::{
    brand_kit_manifest_json, extract_carrier_from_svg, generate_alphabet, generate_glyph,
    generate_project_mark, glyph_to_svg, plan_from_latent, render_ambient_constellation_ex,
    render_constellation_map, render_duet_svg, render_flicker_svg, render_handshake_strip,
    render_kaleidoscope_svg, render_living_specimen, render_mesh_html, render_notarize_svg,
    render_plan_svg, render_postcard_svg, render_seal_svg, render_spectrogram_svg,
    render_timeline_svg, render_tty_specimen, BRAND_MARK_SYNC_PATHS, BRAND_TOKENS,
};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Extra unexpected surfaces flattened into the top-level `dold` CLI.
/// Kept separate so the main Commands enum stays within Windows debug stack limits.
#[derive(clap::Subcommand, Debug)]
pub enum ExtraSurfaceCommands {
    /// Radial / symmetric machine-glyph mandala (still Open-decodable)
    Kaleidoscope {
        input: Option<PathBuf>,
        #[arg(short, long, default_value = "kaleidoscope.svg")]
        output: PathBuf,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        #[arg(long)]
        decode: Option<PathBuf>,
    },
    /// Multi-epoch living strip in one SVG (scrub with --epoch on decode)
    Timeline {
        input: Option<PathBuf>,
        #[arg(short, long, default_value = "timeline.svg")]
        output: PathBuf,
        /// Comma-separated epochs to render (default: 1,2,3)
        #[arg(long, default_value = "1,2,3")]
        epochs: String,
        /// On decode: select which epoch label to print (cosmetic scrub)
        #[arg(long)]
        epoch: Option<u64>,
        #[arg(long)]
        decode: Option<PathBuf>,
    },
    /// Wax-seal compound mark binding payload digest (integrity ≠ signature)
    Seal {
        input: Option<PathBuf>,
        #[arg(short, long, default_value = "seal.svg")]
        output: PathBuf,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Verify a seal SVG (hash match; not a signature)
        #[arg(long)]
        verify: Option<PathBuf>,
    },
    /// Compare two postcard/surface SVGs for Open payload equality
    Compare {
        /// First SVG (postcard, ambient, seal, …)
        a: PathBuf,
        /// Second SVG
        b: PathBuf,
        #[arg(long)]
        json: bool,
    },
}

pub fn run_extra(cmd: ExtraSurfaceCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        ExtraSurfaceCommands::Kaleidoscope {
            input,
            output,
            seed,
            decode,
        } => cmd_kaleidoscope(input, output, seed, decode),
        ExtraSurfaceCommands::Timeline {
            input,
            output,
            epochs,
            epoch,
            decode,
        } => cmd_timeline(input, output, &epochs, epoch, decode),
        ExtraSurfaceCommands::Seal {
            input,
            output,
            seed,
            verify,
        } => cmd_seal(input, output, seed, verify),
        ExtraSurfaceCommands::Compare { a, b, json } => cmd_compare(a, b, json),
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum RadioCommands {
    /// Encode a message into a visual packet directory (α → β)
    Send {
        /// Plaintext message (or read stdin if omitted)
        message: Option<String>,
        #[arg(long, default_value = "radio-shared-seed")]
        seed: String,
        #[arg(long, default_value = "radio-42")]
        session_id: String,
        #[arg(short, long, default_value = "radio-packet")]
        out: PathBuf,
    },
    /// Receive / verify a packet directory (β reconstructs α)
    Recv {
        /// Packet directory from `dold radio send`
        dir: PathBuf,
    },
    /// Round-trip exchange in one shot (send + recv)
    Exchange {
        message: Option<String>,
        #[arg(long, default_value = "radio-shared-seed")]
        seed: String,
        #[arg(long, default_value = "radio-42")]
        session_id: String,
        #[arg(short, long, default_value = "radio-packet")]
        out: PathBuf,
    },
}

pub fn run_radio(cmd: RadioCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        RadioCommands::Send {
            message,
            seed,
            session_id,
            out,
        } => {
            let text = message_or_stdin(message)?;
            radio_send(&text, &seed, &session_id, &out)?;
            println!("OK — packet → {}", out.display());
        }
        RadioCommands::Recv { dir } => {
            let text = radio_recv(&dir)?;
            println!("OK — reconstructed:\n{text}");
        }
        RadioCommands::Exchange {
            message,
            seed,
            session_id,
            out,
        } => {
            let text = message_or_stdin(message)?;
            radio_send(&text, &seed, &session_id, &out)?;
            let back = radio_recv(&out)?;
            if back != text {
                return Err(format!("round-trip mismatch: {back:?} != {text:?}").into());
            }
            println!("OK — radio round-trip verified ({session_id})");
            println!("{back}");
        }
    }
    Ok(())
}

fn radio_send(
    text: &str,
    seed: &str,
    session_id: &str,
    out: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(out)?;
    let session = Session::builder()
        .seed(seed.as_bytes())
        .session_id(session_id)
        .build()?;
    let doc = DskDocument::encode_session(text, &session)?;
    let encoded = session.encode(text)?;
    let catalog = generate_alphabet(doldskrift::FONT_VERSION, 1)?;
    let mut svg = format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="120" viewBox="0 0 {w} 120">
  <rect width="100%" height="100%" fill="#121614"/>
  <text x="12" y="16" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="10">Agent Radio · session {session_id} · Open (not encryption)</text>
"##,
        w = encoded.chars().count() as i32 * 64 + 24
    );
    for (i, ch) in encoded.chars().enumerate() {
        let cp = ch as u32;
        let g = catalog
            .iter()
            .find(|g| g.codepoint == cp)
            .cloned()
            .unwrap_or(generate_glyph(cp, doldskrift::FONT_VERSION, 1)?);
        let x = 12 + i as i32 * 64;
        let inner = glyph_to_svg(&g)
            .lines()
            .filter(|l| {
                let t = l.trim_start();
                !t.starts_with("<?xml") && !t.starts_with("<svg") && !t.starts_with("</svg>")
            })
            .collect::<Vec<_>>()
            .join("\n");
        svg.push_str(&format!(
            r##"<g transform="translate({x},28) scale(0.06)" stroke="#D8EFE6" fill="none">{inner}</g>"##
        ));
        svg.push('\n');
    }
    svg.push_str("</svg>\n");

    let digest = hex::encode(Sha256::digest(text.as_bytes()));
    let manifest = serde_json::json!({
        "kind": "dold-radio-packet",
        "version": 1,
        "session_id": session_id,
        "seed_hint": "shared out-of-band (not a secret)",
        "sha256": digest,
        "mode": "session",
        "note": "Visual packet for agent exchange. Not encryption.",
        "files": ["payload.dsk", "visual.svg", "manifest.json"]
    });

    fs::write(out.join("payload.dsk"), doc.to_bytes()?)?;
    fs::write(out.join("visual.svg"), svg)?;
    fs::write(
        out.join("manifest.json"),
        serde_json::to_string_pretty(&manifest)?,
    )?;
    Ok(())
}

fn radio_recv(dir: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join("manifest.json"))?)?;
    let doc = DskDocument::parse(&fs::read(dir.join("payload.dsk"))?)?;
    let text = doc.decode_text()?;
    if let Some(expected) = manifest["sha256"].as_str() {
        let got = hex::encode(Sha256::digest(text.as_bytes()));
        if got != expected {
            return Err(format!("SHA-256 mismatch: {got} != {expected}").into());
        }
    }
    let _ = dir.join("visual.svg");
    Ok(text)
}

/// Self-describing echo document: payload + SHA-256 as a machine glyph strip.
pub fn cmd_echo(
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    verify: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = verify {
        let svg = fs::read_to_string(path)?;
        let carrier =
            extract_carrier_from_svg(&svg).ok_or("echo verify: missing dsk-carrier in SVG")?;
        let plaintext = decode(&carrier.encoded)?;
        let got = hex::encode(Sha256::digest(plaintext.as_bytes()));
        let expected = carrier
            .sha256
            .as_deref()
            .ok_or("echo verify: carrier missing sha256")?;
        if got != expected {
            return Err(format!("FAIL — hash mismatch {got} != {expected}").into());
        }
        println!("OK — echo integrity verified");
        println!("sha256: {got}");
        println!("payload: {plaintext}");
        return Ok(());
    }

    let bytes = read_input(input)?;
    let text = std::str::from_utf8(&bytes)?;
    let digest = hex::encode(Sha256::digest(text.as_bytes()));
    let encoded = encode(text)?;
    // Hash strip: encode hex digest as Open glyphs after a separator mark
    let hash_encoded = encode(&digest)?;
    let catalog = generate_alphabet(doldskrift::FONT_VERSION, 1)?;
    let mark = generate_project_mark(1)?;
    let total = encoded.chars().count() + 1 + hash_encoded.chars().count();
    let cell = 56;
    let width = total as i32 * cell + 32;
    let carrier = doldskrift_font::SurfaceCarrier {
        kind: "echo-integrity".into(),
        encoded: encoded.clone(),
        sha256: Some(digest.clone()),
        epoch: None,
        note: "Self-describing Open document. Hash is of semantic plaintext. Not a MAC/signature."
            .into(),
        channel_b: None,
        wire: None,
    };
    let meta = serde_json::to_string(&carrier)?;
    let mut svg = format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="150" viewBox="0 0 {width} 150">
  <desc id="dsk-carrier">{}</desc>
  <rect width="100%" height="100%" fill="#121614"/>
  <text x="12" y="18" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="10">echo · payload glyphs · mark · SHA-256 glyph strip</text>
  <text x="12" y="140" fill="#5a7a70" font-family="IBM Plex Mono, monospace" font-size="9">sha256:{digest}</text>
"##,
        xml_escape(&meta)
    );

    let mut x = 12i32;
    for ch in encoded.chars() {
        paint_glyph(&mut svg, &catalog, ch as u32, x, "#D8EFE6")?;
        x += cell;
    }
    // separator: project mark
    {
        let inner = strip_svg(&glyph_to_svg(&mark));
        svg.push_str(&format!(
            r##"<g transform="translate({x},32) scale(0.05)" stroke="#0F6B5C" fill="none">{inner}</g>"##
        ));
        x += cell;
    }
    for ch in hash_encoded.chars() {
        paint_glyph(&mut svg, &catalog, ch as u32, x, "#8fc4b4")?;
        x += cell;
    }
    svg.push_str("</svg>\n");

    let out = output.unwrap_or_else(|| PathBuf::from("echo.svg"));
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&out, svg)?;
    println!("Wrote echo document → {}", out.display());
    println!("sha256: {digest}");
    println!("verify: dold echo --verify {}", out.display());
    Ok(())
}

fn paint_glyph(
    svg: &mut String,
    catalog: &[doldskrift_font::GlyphGeometry],
    cp: u32,
    x: i32,
    stroke: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let g = catalog
        .iter()
        .find(|g| g.codepoint == cp)
        .cloned()
        .unwrap_or(generate_glyph(cp, doldskrift::FONT_VERSION, 1)?);
    let inner = strip_svg(&glyph_to_svg(&g));
    svg.push_str(&format!(
        r#"<g transform="translate({x},32) scale(0.05)" stroke="{stroke}" fill="none">{inner}</g>"#
    ));
    svg.push('\n');
    Ok(())
}

/// Swedish etymology + profile identity (also surfaced via `dold museum`).
pub fn cmd_about(json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mark = generate_project_mark(1)?;
    let about = serde_json::json!({
        "name": "Doldskrift",
        "flag": "🇸🇪",
        "etymology": {
            "language": "Swedish",
            "parts": [
                { "form": "dold", "gloss": "hidden / concealed" },
                { "form": "skrift", "gloss": "writing / script" }
            ],
            "sense_en": "hidden writing / concealed script",
            "tagline_sv": "Maskinskriven betydelse — inte hemlighet.",
            "tagline_en": "Machine-written meaning — not secrecy."
        },
        "project_mark": format!("U+{:04X}", DSK_PROJECT_MARK),
        "fingerprint": mark.fingerprint(),
        "profiles": [
            {
                "id": "open",
                "label": "Open",
                "protocols": ["DSK/1", "DSK/2"],
                "encryption": false,
                "note": "Deterministic visual representation. Not encryption."
            },
            {
                "id": "neural",
                "label": "Neural",
                "protocols": ["DSK/3 Neural"],
                "encryption": false,
                "note": "Learned grammar + compatible reader. Neural ≠ encryption."
            },
            {
                "id": "protected",
                "label": "Protected",
                "protocols": ["DSK/3 Protected"],
                "encryption": true,
                "shipping": false,
                "note": "AEAD + Gate when it ships; refuse-path stubs today."
            }
        ],
        "see_also": ["dold museum", "dold funding", "dold brand", "docs/brand.md", "SECURITY.md", "TRADEMARKS.md"]
    });

    if json {
        println!("{}", serde_json::to_string_pretty(&about)?);
    } else {
        println!("Doldskrift 🇸🇪");
        println!("  Swedish: dold (hidden/concealed) + skrift (writing/script)");
        println!("  → “hidden writing” / “concealed script”");
        println!("  Maskinskriven betydelse — inte hemlighet.");
        println!("  (Machine-written meaning — not secrecy.)\n");
        println!(
            "Project mark: U+{:04X}  fingerprint: {}",
            DSK_PROJECT_MARK,
            mark.fingerprint()
        );
        println!("\nProfiles:");
        println!("  · Open       — DSK/1–2  · representation only (not encryption)");
        println!("  · Neural     — DSK/3    · learned grammar (not encryption)");
        println!("  · Protected  — DSK/3    · AEAD + Gate (not shipping; stubs refuse)");
        println!("\nUnexpected surfaces: postcard · duet · spectrogram · notarize · kaleidoscope");
        println!(
            "  timeline · seal · compare · ambient[--animate] · mesh --html · handshake visual"
        );
        println!("Tip: dold about --json · dold funding · dold brand · dold museum");
    }
    Ok(())
}

/// Canonical thanks.dev path for the maintainer GitHub login (personal setup).
pub const THANKS_DEV_URL: &str = "https://thanks.dev/u/gh/theworker02";
/// Repository funding file (GitHub Sponsors UI).
pub const FUNDING_YML_PATH: &str = ".github/FUNDING.yml";
/// Project GitHub Pages site.
pub const SITE_URL: &str = "https://theworker02.github.io/doldskrift/";
/// Funding page on GitHub Pages.
pub const SITE_FUNDING_URL: &str = "https://theworker02.github.io/doldskrift/funding.html";

/// Print sponsor / thanks.dev URLs (`dold funding`).
pub fn cmd_funding(json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let funding = serde_json::json!({
        "schema": "doldskrift.funding/1",
        "project": "Doldskrift",
        "thanks_dev": THANKS_DEV_URL,
        "github_login": "theworker02",
        "funding_yml": FUNDING_YML_PATH,
        "site_funding": SITE_FUNDING_URL,
        "repository": "https://github.com/theworker02/doldskrift",
        "site": SITE_URL,
        "note": "Open-source sustainability — not a crypto product. Open/Neural ≠ encryption.",
        "cli": "dold funding [--json]"
    });
    if json {
        println!("{}", serde_json::to_string_pretty(&funding)?);
    } else {
        println!("Doldskrift funding");
        println!("  thanks.dev:   {THANKS_DEV_URL}");
        println!("  GitHub file:  {FUNDING_YML_PATH}");
        println!("  Site page:    {SITE_FUNDING_URL}");
        println!("  Repository:   https://github.com/theworker02/doldskrift");
        println!();
        println!("Support sustains the open core. This is not encryption sponsorship.");
        println!("Tip: dold funding --json · see SUPPORT.md · site /funding.html");
    }
    Ok(())
}

/// Export brand kit: mark SVG, tokens JSON, sync checklist (`dold brand`).
pub fn cmd_brand(
    out: PathBuf,
    seed: u64,
    json_only: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let manifest = brand_kit_manifest_json(seed)?;
    if json_only {
        print!("{manifest}");
        return Ok(());
    }
    fs::create_dir_all(&out)?;
    let mark = generate_project_mark(seed)?;
    let logo = kit_logo_svg(&glyph_to_svg(&mark));
    let logo_path = out.join("logo-mark.svg");
    fs::write(&logo_path, logo)?;
    let tokens = format!(
        "{{\n  \"ink\": \"{}\",\n  \"teal\": \"{}\",\n  \"paper\": \"{}\",\n  \"paper_alt\": \"{}\"\n}}\n",
        BRAND_TOKENS.ink, BRAND_TOKENS.teal, BRAND_TOKENS.paper, BRAND_TOKENS.paper_alt
    );
    fs::write(out.join("tokens.json"), tokens)?;
    fs::write(out.join("brand-manifest.json"), &manifest)?;
    let mut checklist = String::from("# Doldskrift brand kit\n\n");
    checklist.push_str(
        "Regenerated with `dold brand`. Canonical mark = `DSK_PROJECT_MARK` (U+E1F0).\n\n",
    );
    checklist.push_str("## Sync checklist\n\n");
    for p in BRAND_MARK_SYNC_PATHS {
        checklist.push_str(&format!("- [ ] `{p}`\n"));
    }
    checklist.push_str("\nSee `docs/brand.md` and `TRADEMARKS.md`.\n");
    checklist.push_str(&format!("Funding: {THANKS_DEV_URL}\n"));
    fs::write(out.join("README.md"), checklist)?;
    println!("Brand kit → {}", out.display());
    println!("  logo-mark.svg · tokens.json · brand-manifest.json · README.md");
    println!(
        "  mark U+{:04X}  ink {}  teal {}",
        DSK_PROJECT_MARK, BRAND_TOKENS.ink, BRAND_TOKENS.teal
    );
    println!("Tip: dold logo · dold brand --json · docs/brand.md");
    Ok(())
}

fn kit_logo_svg(inner_glyph_svg: &str) -> String {
    let body = inner_glyph_svg
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            !t.starts_with("<?xml") && !t.starts_with("<svg") && !t.starts_with("</svg>")
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128" role="img" aria-label="Doldskrift project mark U+E1F0">
  <rect width="128" height="128" rx="20" fill="{paper}"/>
  <g transform="translate(14,14) scale(0.1)" fill="none" stroke="{ink}" stroke-width="36" stroke-linecap="square" stroke-linejoin="miter">
{body}
  </g>
  <circle cx="104" cy="24" r="4" fill="{teal}"/>
</svg>
"##,
        paper = BRAND_TOKENS.paper,
        ink = BRAND_TOKENS.ink,
        teal = BRAND_TOKENS.teal,
    )
}

/// Protocol archaeology museum.
pub fn cmd_museum(json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mark = generate_project_mark(1)?;
    let sample = encode("DSK")?;
    let exhibits = vec![
        serde_json::json!({
            "id": "etymology",
            "era": "Name",
            "profile": "Identity",
            "flag": "🇸🇪",
            "etymology": "dold (hidden) + skrift (writing) → concealed script",
            "tagline_sv": "Maskinskriven betydelse — inte hemlighet.",
            "note": "Swedish-rooted name. Hidden ≠ encrypted (Open/Neural)."
        }),
        serde_json::json!({
            "id": "dsk1-open",
            "era": "Protocol core",
            "profile": "Open",
            "protocol": "DSK/1",
            "status": "stable prototype",
            "sample_encoded": sample,
            "note": "Deterministic PUA alphabet. Not encryption."
        }),
        serde_json::json!({
            "id": "mge2",
            "era": "Visual layer",
            "profile": "Open",
            "engine": "MGE/2",
            "status": "candidate",
            "project_mark": format!("U+{:04X}", DSK_PROJECT_MARK),
            "fingerprint": mark.fingerprint(),
            "note": "Zone anatomy + DSKG-1 fingerprints"
        }),
        serde_json::json!({
            "id": "dsk2-semantic",
            "era": "Semantic layer",
            "profile": "Open",
            "protocol": "DSK/2",
            "status": "experimental",
            "note": "Semantic values / query path"
        }),
        serde_json::json!({
            "id": "dsk3-neural",
            "era": "Neural profile",
            "profile": "Neural",
            "protocol": "DSK/3 Neural",
            "lsg": GrammarEpoch::E0001.label(),
            "status": "foundation stubs",
            "note": "LSG + DeterministicBaseline. Neural ≠ encryption."
        }),
        serde_json::json!({
            "id": "dsk3-protected",
            "era": "Protected profile",
            "profile": "Protected",
            "protocol": "DSK/3 Protected",
            "status": "refuse-path stubs (no AEAD)",
            "note": "inspect Readable:No; decode refuses; open → AccessDenied"
        }),
        serde_json::json!({
            "id": "unexpected-surfaces",
            "era": "0.5 prep",
            "profile": "Open demos",
            "status": "shipping CLI",
            "commands": ["radio", "echo", "ambient", "ambient --animate", "flicker", "live", "mesh", "mesh --html", "postcard", "duet", "spectrogram", "notarize", "kaleidoscope", "timeline", "seal", "compare", "handshake visual", "museum", "about", "render --tty", "scan --diff"],
            "note": "Delight surfaces — still Open honesty model"
        }),
    ];

    if json {
        println!("{}", serde_json::to_string_pretty(&exhibits)?);
    } else {
        println!("Doldskrift Museum — protocol archaeology 🇸🇪\n");
        println!("Name: dold (hidden) + skrift (writing) → concealed script");
        println!("Profiles: Open · Neural · Protected  (see: dold about)\n");
        for e in &exhibits {
            println!(
                "· {}  [{}]  {}",
                e["id"].as_str().unwrap_or("?"),
                e["era"].as_str().unwrap_or("?"),
                e["note"].as_str().unwrap_or("")
            );
            if let Some(fp) = e["fingerprint"].as_str() {
                println!("    mark fingerprint: {fp}");
            }
            if let Some(s) = e["sample_encoded"].as_str() {
                println!("    sample: {s}");
            }
        }
        println!("\nTip: dold museum --json | jq · dold about");
    }
    Ok(())
}

pub fn cmd_ambient(
    input: Option<PathBuf>,
    output: PathBuf,
    decode_path: Option<PathBuf>,
    seed: u64,
    animate: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = decode_path {
        let svg = fs::read_to_string(path)?;
        let carrier = extract_carrier_from_svg(&svg).ok_or("ambient: no dsk-carrier in SVG")?;
        let text = decode(&carrier.encoded)?;
        println!("{text}");
        if let Some(h) = carrier.sha256 {
            println!("# sha256: {h}");
        }
        return Ok(());
    }
    let bytes = read_input(input)?;
    let text = std::str::from_utf8(&bytes)?;
    let encoded = encode(text)?;
    let digest = hex::encode(Sha256::digest(text.as_bytes()));
    let svg = render_ambient_constellation_ex(&encoded, seed, Some(&digest), animate);
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, svg)?;
    println!(
        "Wrote ambient constellation{} → {}",
        if animate { " (animated)" } else { "" },
        output.display()
    );
    println!("decode: dold ambient --decode {}", output.display());
    Ok(())
}

pub fn cmd_flicker(
    input: Option<PathBuf>,
    output: PathBuf,
    seed: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = read_input(input)?;
    let text = std::str::from_utf8(&bytes)?;
    let encoded = encode(text)?;
    let svg = render_flicker_svg(&encoded, seed)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, &svg)?;
    println!("Wrote temporal flicker → {}", output.display());
    if let Some(c) = extract_carrier_from_svg(&svg) {
        println!("carrier kind: {}", c.kind);
        println!("decode: dold ambient --decode {}", output.display());
    }
    Ok(())
}

pub fn cmd_live(
    input: Option<PathBuf>,
    output: PathBuf,
    epoch: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = read_input(input)?;
    let text = std::str::from_utf8(&bytes)?;
    let encoded = encode(text)?;
    let (svg, _) = render_living_specimen(&encoded, epoch)?;
    // Prove Open recoverability
    let back = decode(&encoded)?;
    if back != text {
        return Err("living: Open decode failed".into());
    }
    // Neural DeterministicBaseline recovers via latent carrier (Open visual still varies)
    let graph = compose_deterministic(&Value::String(text.to_owned()))?;
    let plan = plan_from_latent(&graph, epoch);
    let _neural_svg = render_plan_svg(&graph, &plan, epoch);
    let obs = ObservationGraph::from_latent_carrier(graph);
    let reader = DeterministicBaseline::new();
    let out = reader.reconstruct(&obs)?;
    if out.value.is_none() {
        return Err(format!("living: baseline status {:?}", out.status).into());
    }

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, svg)?;
    println!("Wrote living document epoch={epoch} → {}", output.display());
    println!("Open decode: OK ({text:?})");
    println!("Visual variants change with --epoch; recoverability preserved.");
    Ok(())
}

pub fn cmd_mesh(
    input: Option<PathBuf>,
    output: PathBuf,
    html: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = read_input(input)?;
    let text = std::str::from_utf8(&bytes)?;
    let graph = compose_deterministic(&Value::String(text.to_owned()))?;
    let plan = plan_from_latent(&graph, 1);
    let mut nodes = Vec::new();
    let n = plan.compounds.len().max(graph.nodes.len()).max(1);
    for (i, node) in graph.nodes.iter().enumerate().take(32) {
        let angle = (i as f64) / n as f64 * std::f64::consts::TAU;
        let r = 110.0 + (i % 5) as f64 * 18.0;
        nodes.push((
            node.id.clone(),
            320.0 + angle.cos() * r,
            180.0 + angle.sin() * r,
        ));
    }
    let mut edges = Vec::new();
    for c in plan.compounds.iter() {
        if c.members.len() >= 2 {
            let a = graph.nodes.iter().position(|n| n.id == c.members[0]);
            let b = graph.nodes.iter().position(|n| n.id == c.members[1]);
            if let (Some(ai), Some(bi)) = (a, b) {
                if ai < nodes.len() && bi < nodes.len() {
                    edges.push((ai, bi));
                }
            }
        }
    }
    for i in 0..nodes.len().saturating_sub(1) {
        edges.push((i, i + 1));
    }
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    if html {
        let html_doc = render_mesh_html(
            &nodes,
            &edges,
            "Neural constellation map (MGE/5 plan)",
            &format!(
                "payload chars={} · Open/Deterministic LSG · not encryption",
                text.len()
            ),
        );
        fs::write(&output, html_doc)?;
        println!("Wrote interactive mesh HTML → {}", output.display());
    } else {
        let svg = render_constellation_map(&nodes, &edges, "Neural constellation map (MGE/5 plan)");
        fs::write(&output, &svg)?;
        let plan_svg = render_plan_svg(&graph, &plan, 1);
        let plan_path = output.with_extension("plan.svg");
        fs::write(&plan_path, plan_svg)?;
        println!("Wrote mesh map → {}", output.display());
        println!("Wrote plan SVG → {}", plan_path.display());
        println!("tip: dold mesh --html -o mesh.html");
    }
    Ok(())
}

/// Machine postcard SVG (constellation + digest + glyph strip).
pub fn cmd_postcard(
    input: Option<PathBuf>,
    output: PathBuf,
    seed: u64,
    read: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = read {
        let svg = fs::read_to_string(path)?;
        let carrier = extract_carrier_from_svg(&svg).ok_or("postcard: no dsk-carrier in SVG")?;
        if carrier.kind != "machine-postcard" && carrier.kind != "ambient-constellation" {
            eprintln!(
                "note: carrier kind={} (still attempting Open decode)",
                carrier.kind
            );
        }
        let text = decode(&carrier.encoded)?;
        println!("{text}");
        if let Some(h) = carrier.sha256 {
            let got = hex::encode(Sha256::digest(text.as_bytes()));
            if got == h {
                println!("OK — sha256 matches ({})", &h[..h.len().min(12)]);
            } else {
                return Err(format!("sha256 mismatch: expected {h}, got {got}").into());
            }
        }
        return Ok(());
    }
    let bytes = read_input(input)?;
    let text = std::str::from_utf8(&bytes)?;
    let encoded = encode(text)?;
    let digest = hex::encode(Sha256::digest(text.as_bytes()));
    let svg = render_postcard_svg(&encoded, seed, &digest);
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, svg)?;
    println!("Wrote machine postcard → {}", output.display());
    println!("read: dold postcard --read {}", output.display());
    Ok(())
}

/// Dual interleaved Open channels.
pub fn cmd_duet(
    channel_a: Option<PathBuf>,
    channel_b: Option<PathBuf>,
    output: PathBuf,
    seed: u64,
    decode_path: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = decode_path {
        let svg = fs::read_to_string(path)?;
        let carrier = extract_carrier_from_svg(&svg).ok_or("duet: no dsk-carrier")?;
        let a = decode(&carrier.encoded)?;
        let b = carrier
            .channel_b
            .as_deref()
            .map(decode)
            .transpose()?
            .unwrap_or_default();
        println!("A: {a}");
        println!("B: {b}");
        return Ok(());
    }
    let a_bytes = read_input(channel_a)?;
    let a_text = std::str::from_utf8(&a_bytes)?.trim_end().to_owned();
    let b_text = if let Some(p) = channel_b {
        fs::read_to_string(p)?.trim_end().to_owned()
    } else {
        // Default B channel: deterministic ack of A length (no second file required)
        format!("ack:{}", a_text.chars().count())
    };
    let enc_a = encode(&a_text)?;
    let enc_b = encode(&b_text)?;
    let svg = render_duet_svg(&enc_a, &enc_b, seed)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, svg)?;
    println!("Wrote duet SVG → {}", output.display());
    println!("decode: dold duet --decode {}", output.display());
    Ok(())
}

/// Spectrogram-style Open strip.
pub fn cmd_spectrogram(
    input: Option<PathBuf>,
    output: PathBuf,
    seed: u64,
    decode_path: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = decode_path {
        let svg = fs::read_to_string(path)?;
        let carrier = extract_carrier_from_svg(&svg).ok_or("spectrogram: no dsk-carrier")?;
        println!("{}", decode(&carrier.encoded)?);
        return Ok(());
    }
    let bytes = read_input(input)?;
    let text = std::str::from_utf8(&bytes)?;
    let encoded = encode(text)?;
    let svg = render_spectrogram_svg(&encoded, seed);
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, svg)?;
    println!("Wrote spectrogram strip → {}", output.display());
    println!("decode: dold spectrogram --decode {}", output.display());
    Ok(())
}

/// Integrity notarize (hash margin marks — not a signature).
pub fn cmd_notarize(
    input: Option<PathBuf>,
    output: PathBuf,
    seed: u64,
    verify: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = verify {
        let svg = fs::read_to_string(path)?;
        let carrier = extract_carrier_from_svg(&svg).ok_or("notarize: no dsk-carrier")?;
        let text = decode(&carrier.encoded)?;
        let expected = carrier.sha256.ok_or("notarize: carrier missing sha256")?;
        let got = hex::encode(Sha256::digest(text.as_bytes()));
        if got != expected {
            return Err(format!(
                "NOTARIZE FAIL — sha256 mismatch\n  expected {expected}\n  got      {got}"
            )
            .into());
        }
        println!("OK — notarize integrity verified (hash match; not a signature)");
        println!("payload: {text}");
        return Ok(());
    }
    let bytes = read_input(input)?;
    let text = std::str::from_utf8(&bytes)?;
    let encoded = encode(text)?;
    let digest = hex::encode(Sha256::digest(text.as_bytes()));
    let svg = render_notarize_svg(&encoded, &digest, seed)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, svg)?;
    println!("Wrote notarize specimen → {}", output.display());
    println!("verify: dold notarize --verify {}", output.display());
    Ok(())
}

/// Radial kaleidoscope mandala.
pub fn cmd_kaleidoscope(
    input: Option<PathBuf>,
    output: PathBuf,
    seed: u64,
    decode_path: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = decode_path {
        let svg = fs::read_to_string(path)?;
        let carrier = extract_carrier_from_svg(&svg).ok_or("kaleidoscope: no dsk-carrier")?;
        println!("{}", decode(&carrier.encoded)?);
        return Ok(());
    }
    let bytes = read_input(input)?;
    let text = std::str::from_utf8(&bytes)?;
    let encoded = encode(text)?;
    let svg = render_kaleidoscope_svg(&encoded, seed)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, svg)?;
    println!("Wrote kaleidoscope mandala → {}", output.display());
    println!("decode: dold kaleidoscope --decode {}", output.display());
    Ok(())
}

/// Multi-epoch timeline strip.
pub fn cmd_timeline(
    input: Option<PathBuf>,
    output: PathBuf,
    epochs_csv: &str,
    scrub_epoch: Option<u64>,
    decode_path: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = decode_path {
        let svg = fs::read_to_string(path)?;
        let carrier = extract_carrier_from_svg(&svg).ok_or("timeline: no dsk-carrier")?;
        let text = decode(&carrier.encoded)?;
        let epochs = carrier.channel_b.as_deref().unwrap_or("");
        if let Some(e) = scrub_epoch {
            println!("# scrub epoch={e} (visual label; payload identical across epochs)");
            if !epochs.is_empty() && !epochs.split(',').any(|s| s.trim() == e.to_string()) {
                eprintln!("note: epoch {e} not in carrier list [{epochs}]");
            }
        } else if let Some(e) = carrier.epoch {
            println!("# primary epoch={e} · all=[{epochs}]");
        }
        println!("{text}");
        return Ok(());
    }
    let epochs: Vec<u64> = epochs_csv
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    if epochs.is_empty() {
        return Err("timeline: --epochs must list at least one u64".into());
    }
    let bytes = read_input(input)?;
    let text = std::str::from_utf8(&bytes)?;
    let encoded = encode(text)?;
    let svg = render_timeline_svg(&encoded, &epochs)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, svg)?;
    println!(
        "Wrote timeline strip epochs={epochs:?} → {}",
        output.display()
    );
    println!(
        "decode: dold timeline --decode {} [--epoch N]",
        output.display()
    );
    Ok(())
}

/// Wax seal integrity mark.
pub fn cmd_seal(
    input: Option<PathBuf>,
    output: PathBuf,
    seed: u64,
    verify: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = verify {
        let svg = fs::read_to_string(path)?;
        let carrier = extract_carrier_from_svg(&svg).ok_or("seal: no dsk-carrier")?;
        let text = decode(&carrier.encoded)?;
        let expected = carrier.sha256.ok_or("seal: carrier missing sha256")?;
        let got = hex::encode(Sha256::digest(text.as_bytes()));
        if got != expected {
            return Err(format!(
                "SEAL FAIL — sha256 mismatch\n  expected {expected}\n  got      {got}\n(integrity check only — not a cryptographic signature)"
            )
            .into());
        }
        println!("OK — seal integrity verified (hash match; not a signature)");
        println!("payload: {text}");
        return Ok(());
    }
    let bytes = read_input(input)?;
    let text = std::str::from_utf8(&bytes)?;
    let encoded = encode(text)?;
    let digest = hex::encode(Sha256::digest(text.as_bytes()));
    let svg = render_seal_svg(&encoded, &digest, seed)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, svg)?;
    println!("Wrote wax seal → {}", output.display());
    println!("verify: dold seal --verify {}", output.display());
    Ok(())
}

/// Compare two surface SVGs for Open payload equality (and optional digest equality).
pub fn cmd_compare(a: PathBuf, b: PathBuf, json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let svg_a = fs::read_to_string(&a)?;
    let svg_b = fs::read_to_string(&b)?;
    let ca = extract_carrier_from_svg(&svg_a).ok_or("compare: no dsk-carrier in first SVG")?;
    let cb = extract_carrier_from_svg(&svg_b).ok_or("compare: no dsk-carrier in second SVG")?;
    let text_a = decode(&ca.encoded)?;
    let text_b = decode(&cb.encoded)?;
    let payload_eq = text_a == text_b;
    let digest_eq = match (&ca.sha256, &cb.sha256) {
        (Some(ha), Some(hb)) => Some(ha == hb),
        (None, None) => None,
        _ => Some(false),
    };
    let report = serde_json::json!({
        "a": { "path": a.display().to_string(), "kind": ca.kind, "sha256": ca.sha256 },
        "b": { "path": b.display().to_string(), "kind": cb.kind, "sha256": cb.sha256 },
        "payload_equal": payload_eq,
        "digest_equal": digest_eq,
        "note": "Open payload equality — not a cryptographic signature comparison",
    });
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("payload: {}", if payload_eq { "EQUAL" } else { "DIFFER" });
        match digest_eq {
            Some(true) => println!("digest:  EQUAL"),
            Some(false) => println!("digest:  DIFFER (or one side missing)"),
            None => println!("digest:  n/a (neither carrier embeds sha256)"),
        }
        println!("A kind={}  B kind={}", ca.kind, cb.kind);
        if !payload_eq {
            println!("A: {text_a}");
            println!("B: {text_b}");
        }
    }
    if !payload_eq {
        return Err("compare: payloads differ".into());
    }
    Ok(())
}

/// Visual Open capability strip (+ optional wire file).
pub fn cmd_handshake_visual(
    output: PathBuf,
    seed: u64,
    offer_path: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    use doldskrift::{negotiate, CapabilitySet, HandshakeOffer};
    let offer = if let Some(p) = offer_path {
        HandshakeOffer::from_wire(&fs::read_to_string(p)?)?
    } else {
        HandshakeOffer::reference()
    };
    let reply = negotiate(&offer, &CapabilitySet::reference())?;
    let wire = format!("{}{}", offer.to_wire(), reply.to_wire());
    let modes = offer
        .capabilities
        .preferred_modes()
        .into_iter()
        .map(|m| m.as_str().to_owned())
        .collect::<Vec<_>>()
        .join(",");
    let svg = render_handshake_strip(&wire, &modes, seed);
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, &svg)?;
    // Also print text wire for agents that prefer ASCII
    print!("{}", offer.to_wire());
    print!("{}", reply.to_wire());
    println!("Wrote handshake strip → {}", output.display());
    println!("note: Open capability negotiation only — not a secure channel");
    Ok(())
}

/// Tiny braille / mark fingerprint delight when `dold` is run with no args.
pub fn quiet_mode_fingerprint() -> Result<(), Box<dyn std::error::Error>> {
    let mark = generate_project_mark(1)?;
    let fp = mark.fingerprint();
    let art = render_tty_specimen("quiet", &[mark]);
    println!("Doldskrift 🇸🇪  ·  dold + skrift → concealed script");
    println!("Maskinskriven betydelse — inte hemlighet.\n");
    // Animated-feeling braille frames derived from fingerprint nibbles
    let nibbles: Vec<u8> = fp
        .bytes()
        .filter_map(|b| {
            let c = b as char;
            c.to_digit(16).map(|d| d as u8)
        })
        .take(24)
        .collect();
    for frame in 0..3u8 {
        let mut line = String::new();
        for (i, n) in nibbles.iter().enumerate() {
            let v = n.wrapping_add(frame).wrapping_add(i as u8) & 0x3f;
            let braille = char::from_u32(0x2800 + u32::from(v)).unwrap_or('·');
            line.push(braille);
        }
        println!("  {line}");
    }
    println!();
    print!("{art}");
    println!("mark U+{:04X}  fingerprint {fp}", DSK_PROJECT_MARK);
    println!("\nTry: dold about · dold museum · dold postcard · dold --help");
    Ok(())
}

pub fn encode_for_tty(
    text: &str,
    mode: Mode,
    seed: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let encoded = match mode {
        Mode::Visual => text.to_owned(),
        Mode::Encoded => encode(text)?,
        Mode::Session => {
            let seed = seed.unwrap_or("tty");
            Session::from_seed(seed.as_bytes(), "tty")?.encode(text)?
        }
        Mode::Protected => {
            return Err("render --tty: Protected has no Open TTY path".into());
        }
    };
    let catalog = generate_alphabet(doldskrift::FONT_VERSION, 1)?;
    let mut glyphs = Vec::new();
    for ch in encoded.chars() {
        let cp = ch as u32;
        let g = catalog
            .iter()
            .find(|g| g.codepoint == cp)
            .cloned()
            .unwrap_or(generate_glyph(cp, doldskrift::FONT_VERSION, 1)?);
        glyphs.push(g);
    }
    Ok(render_tty_specimen(&format!("{mode:?}"), &glyphs))
}

fn message_or_stdin(message: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(m) = message {
        return Ok(m);
    }
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf.trim_end().to_owned())
}

fn read_input(path: Option<PathBuf>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match path {
        None => {
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf)?;
            if buf.ends_with(b"\n") {
                buf.pop();
                if buf.ends_with(b"\r") {
                    buf.pop();
                }
            }
            Ok(buf)
        }
        Some(p) if p.as_os_str() == "-" => {
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf)?;
            Ok(buf)
        }
        Some(p) => Ok(fs::read(p)?),
    }
}

fn strip_svg(svg: &str) -> String {
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
