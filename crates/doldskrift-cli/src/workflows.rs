//! Workflow / catalog CLI surface — kept separate so the main Commands enum
//! stays within Windows debug stack limits (see ExtraSurfaceCommands).

use clap::{Subcommand, ValueEnum};
use doldskrift::{DskDocument, Mode};
use doldskrift_font::{
    extract_carrier_from_svg, generate_alphabet, generate_glyph, generate_project_mark,
    glyph_to_svg, render_ambient_constellation, render_postcard_svg, render_seal_svg,
};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

/// Flattened into top-level `dold` (pipeline, batch, self-test, …).
#[derive(Subcommand, Debug)]
pub enum WorkflowCommands {
    /// Curated grouped listing of all dold commands
    #[command(alias = "topics")]
    Commands,

    /// In-CLI onboarding (honesty model, quickstart, Swedish identity)
    Guide {
        /// Topic: quickstart | honesty | surfaces | agents | swedish | architecture
        #[arg(default_value = "quickstart")]
        topic: String,
    },

    /// Project defaults (`.doldskrift/config.json`)
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },

    /// Convert between Open containers and visual surfaces (no crypto rewrite)
    Convert {
        /// Source format: text | dsk | postcard | ambient | seal
        #[arg(long = "from", value_enum)]
        from: ConvertFormat,
        /// Target format: text | dsk | postcard | ambient | seal
        #[arg(long = "to", value_enum)]
        to: ConvertFormat,
        /// Input file (stdin if omitted; ignored when --text is set)
        #[arg(short = 'i', long)]
        input: Option<PathBuf>,
        /// Inline plaintext (text→* only)
        #[arg(long)]
        text: Option<String>,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, default_value_t = 1)]
        seed: u64,
    },

    /// One-shot product demo pack (Open .dsk + postcard + mark)
    Demo {
        /// Output directory
        #[arg(short, long, default_value = "dold-demo")]
        out: PathBuf,
        /// Demo plaintext
        #[arg(long, default_value = "Hello from Doldskrift.")]
        text: String,
    },

    /// Encode → validate → inspect → optional render (one-shot Open workflow)
    Pipeline {
        /// Plaintext file (stdin if omitted; or use --text)
        input: Option<PathBuf>,
        /// Inline plaintext (overrides input/stdin)
        #[arg(long)]
        text: Option<String>,
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Also write a specimen SVG next to the .dsk (or --render-out)
        #[arg(long)]
        render: bool,
        #[arg(long)]
        render_out: Option<PathBuf>,
        /// Print inspect JSON instead of human summary
        #[arg(long)]
        json: bool,
    },

    /// Batch encode `.txt` or validate `.dsk` files in a directory
    Batch {
        #[command(subcommand)]
        command: BatchCommands,
    },

    /// Built-in smoke: encode/decode/validate round-trip + protected refuse
    #[command(name = "self-test")]
    SelfTest,

    /// Visual surface catalog helpers
    Surfaces {
        #[command(subcommand)]
        command: SurfacesCommands,
    },

    /// Print inspect/validate JSON envelope names and field shapes
    Schema {
        #[arg(long)]
        json: bool,
    },

    /// Scaffold `.doldskrift/` (and optional sample files) in a directory
    Init {
        /// Target directory (default: current)
        #[arg(default_value = ".")]
        dir: PathBuf,
        /// Also write sample.txt + encode a sample.dsk
        #[arg(long)]
        sample: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Print resolved config (JSON)
    Show {
        #[arg(long)]
        json: bool,
    },
    /// Print path to `.doldskrift/config.json`
    Path {
        #[arg(default_value = ".")]
        dir: PathBuf,
    },
    /// Write a default config file
    Init {
        #[arg(default_value = ".")]
        dir: PathBuf,
        /// Overwrite if present
        #[arg(long)]
        force: bool,
    },
    /// Set a key (`default_mode`, `seed`, `json`)
    Set {
        key: String,
        value: String,
        #[arg(default_value = ".")]
        dir: PathBuf,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum ConvertFormat {
    Text,
    Dsk,
    Postcard,
    Ambient,
    Seal,
}

#[derive(Subcommand, Debug)]
pub enum BatchCommands {
    /// Encode every `*.txt` in DIR to `.dsk` (same stem)
    Encode {
        dir: PathBuf,
        /// Output directory (default: DIR)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Validate every `*.dsk` in DIR
    Validate {
        dir: PathBuf,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum SurfacesCommands {
    /// List Open visual surfaces with one-liners
    List,
}

pub fn run(cmd: WorkflowCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        WorkflowCommands::Commands => {
            print!("{}", catalog_text());
            Ok(())
        }
        WorkflowCommands::Guide { topic } => cmd_guide(&topic),
        WorkflowCommands::Config { command } => cmd_config(command),
        WorkflowCommands::Convert {
            from,
            to,
            input,
            text,
            output,
            seed,
        } => cmd_convert(from, to, input, text, output, seed),
        WorkflowCommands::Demo { out, text } => cmd_demo(out, text),
        WorkflowCommands::Pipeline {
            input,
            text,
            output,
            render,
            render_out,
            json,
        } => cmd_pipeline(input, text, output, render, render_out, json),
        WorkflowCommands::Batch { command } => match command {
            BatchCommands::Encode { dir, output } => cmd_batch_encode(dir, output),
            BatchCommands::Validate { dir, json } => cmd_batch_validate(dir, json),
        },
        WorkflowCommands::SelfTest => cmd_self_test(),
        WorkflowCommands::Surfaces { command } => match command {
            SurfacesCommands::List => {
                print!("{}", surfaces_list_text());
                Ok(())
            }
        },
        WorkflowCommands::Schema { json } => cmd_schema(json),
        WorkflowCommands::Init { dir, sample } => cmd_init(dir, sample),
    }
}

/// Category-oriented catalog for `dold commands` / `dold topics`.
pub fn catalog_text() -> String {
    let mut out = String::new();
    out.push_str("dold — command catalog (curated)\n");
    out.push_str(
        "Open / Neural ≠ encryption. Protected = refuse stubs until AEAD. See SECURITY.md.\n\n",
    );

    section(
        &mut out,
        "Document lifecycle",
        &[
            ("encode (e)", "text → .dsk (or --raw PUA)"),
            ("decode (d)", ".dsk / raw → text (Protected refuses)"),
            ("inspect", "metadata; --json → doldskrift.inspect/1"),
            ("validate", "structure + decode expectations; --json"),
            ("fmt", "canonical rewrite (--check)"),
            ("verify", "lighter checksum sanity"),
            ("explain", "educational breakdown"),
            ("pack / unpack", "binary payloads"),
            ("open", "Gate stub → AccessDenied without AEAD"),
            ("query", "experimental JSON/semantic path"),
        ],
    );
    section(
        &mut out,
        "Workflows",
        &[
            ("guide", "in-CLI onboarding (honesty · swedish · …)"),
            (
                "demo",
                "one-shot Open product pack (.dsk + postcard + mark)",
            ),
            ("convert", "bridge text/dsk ↔ postcard/ambient/seal"),
            ("config", "project defaults (.doldskrift/config.json)"),
            ("pipeline", "encode → validate → inspect [→ render]"),
            ("batch encode|validate", "directory of .txt / .dsk"),
            (
                "self-test",
                "built-in encode/decode/validate + protected refuse",
            ),
            ("commands / topics", "this listing"),
            ("schema", "JSON envelope shapes for agents"),
            ("init", "scaffold .doldskrift/ (+ --sample)"),
            ("surfaces list", "Open visual surfaces one-liners"),
        ],
    );
    section(
        &mut out,
        "Render & fonts",
        &[
            ("render", "mode transform / --tty braille"),
            ("specimen", "glyph-strip SVG"),
            ("logo / brand", "project mark + brand kit"),
            ("font build|inspect", "MGE/2 font toolchain"),
            ("forge", "alphabet scoring (experimental)"),
        ],
    );
    section(
        &mut out,
        "Vision & Neural",
        &[
            ("scan / vision", "structural / SVG geometry"),
            ("bench vision", "synthetic recognition bench"),
            ("neural *", "DSK/3 encode/render/reconstruct (Mock path)"),
            ("dataset / reader / evaluate", "Neural foundation stubs"),
        ],
    );
    section(
        &mut out,
        "Protocol & identity",
        &[
            ("doctor", "install health"),
            ("about / funding / museum", "identity + sustainability"),
            ("handshake", "capability negotiation (+ visual)"),
            ("conformance", "golden vectors"),
            ("version / completion", "build info + shell completions"),
        ],
    );
    section(
        &mut out,
        "Unexpected surfaces (Open)",
        &[
            ("radio / echo / ambient / flicker", "packets & carriers"),
            ("live / mesh / postcard / duet", "living + multi-channel"),
            ("spectrogram / notarize", "strips + digest marks"),
            (
                "kaleidoscope / timeline / seal",
                "mandala / epochs / wax seal",
            ),
            ("compare", "Open payload equality across SVGs"),
        ],
    );
    out.push_str("Full reference: docs/reference/cli.md\n");
    out.push_str("Tip: dold --help · dold <cmd> --help\n");
    out
}

fn section(out: &mut String, title: &str, rows: &[(&str, &str)]) {
    out.push_str(title);
    out.push('\n');
    for (cmd, desc) in rows {
        out.push_str(&format!("  {cmd:<28} {desc}\n"));
    }
    out.push('\n');
}

pub fn surfaces_list_text() -> String {
    let rows = [
        (
            "ambient",
            "constellation SVG Open channel (--decode; --animate)",
        ),
        ("flicker", "temporal animated SVG with embedded carrier"),
        ("live", "epoch-varying MGE/4 living document"),
        ("mesh", "Neural constellation map (--html)"),
        ("postcard", "constellation + digest + glyph strip (--read)"),
        ("duet", "interleaved A/B Open channels (--decode)"),
        ("spectrogram", "symbol-id frequency-like strip (--decode)"),
        ("echo", "self-describing SVG + SHA-256 strip (--verify)"),
        ("notarize", "SHA-256 margin marks (--verify; ≠ signature)"),
        ("kaleidoscope", "six-fold Open mandala (--decode)"),
        ("timeline", "multi-epoch living strip (--epochs)"),
        ("seal", "wax-seal digest mark (--verify; ≠ signature)"),
        (
            "radio",
            "agent visual packet directories (send|recv|exchange)",
        ),
        ("handshake visual", "Open capability negotiation strip"),
        ("compare", "Open payload equality for two surface SVGs"),
        ("render --tty", "braille/block terminal specimen"),
        ("specimen", "static glyph-strip SVG from text"),
    ];
    let mut out = String::from(
        "dold surfaces — Open visual surfaces (not steganography; digests ≠ signatures)\n\n",
    );
    for (name, desc) in rows {
        out.push_str(&format!("  {name:<20} {desc}\n"));
    }
    out.push('\n');
    out
}

fn cmd_pipeline(
    input: Option<PathBuf>,
    text: Option<String>,
    output: Option<PathBuf>,
    render: bool,
    render_out: Option<PathBuf>,
    json: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let plaintext = if let Some(t) = text {
        t
    } else {
        let bytes = read_bytes(input)?;
        String::from_utf8(bytes)?
    };
    if plaintext.is_empty() {
        return Err("pipeline: empty input".into());
    }

    let doc = DskDocument::encode_text(&plaintext, Mode::Encoded)?;
    let dsk = doc.to_bytes()?;

    let report = doc.validate();
    if !report.ok {
        eprint!("{}", report.display());
        return Err("pipeline: validation failed".into());
    }

    let out_path = output.unwrap_or_else(|| PathBuf::from("pipeline.dsk"));
    if let Some(parent) = out_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(&out_path, &dsk)?;

    let inspect = doc.inspect();
    if json {
        println!("{}", inspect.to_json()?);
    } else {
        println!("pipeline — OK");
        println!("  wrote:     {}", out_path.display());
        println!("  validate:  OK ({})", report.profile);
        println!("{}", inspect.display());
    }

    if render || render_out.is_some() {
        let svg_path = render_out.unwrap_or_else(|| {
            let mut p = out_path.clone();
            p.set_extension("svg");
            p
        });
        let encoded = doldskrift::encode(&plaintext)?;
        let svg = render_simple_strip(&encoded)?;
        if let Some(parent) = svg_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::write(&svg_path, svg)?;
        if !json {
            println!("  render:    {}", svg_path.display());
        }
    }

    // Round-trip sanity (library path)
    let decoded = doc.decode_text()?;
    if decoded != plaintext {
        return Err("pipeline: decode round-trip mismatch".into());
    }
    Ok(())
}

fn render_simple_strip(encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
    let catalog = generate_alphabet(doldskrift::FONT_VERSION, 1)?;
    let mark = generate_project_mark(1)?;
    let cell = 80;
    let width = (encoded.chars().count().min(48) as i32 + 1) * cell + 24;
    let mut svg = format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="140" viewBox="0 0 {width} 140">
  <rect width="100%" height="100%" fill="#121614"/>
  <text x="12" y="128" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="10">dold pipeline · Open specimen · MGE/2</text>
"##
    );
    let mark_svg = glyph_to_svg(&mark)
        .replace(r#"<?xml version="1.0" encoding="UTF-8"?>"#, "")
        .replace("<svg", "<g")
        .replace("</svg>", "</g>");
    svg.push_str(&format!(
        "<g transform=\"translate(12,8) scale(0.08)\" stroke=\"#E8EFE9\">{mark_svg}</g>\n"
    ));
    for (i, ch) in encoded.chars().take(48).enumerate() {
        let cp = ch as u32;
        let g = catalog
            .iter()
            .find(|g| g.codepoint == cp)
            .cloned()
            .unwrap_or(generate_glyph(cp, doldskrift::FONT_VERSION, 1)?);
        let x = (i as i32 + 1) * cell + 12;
        let inner = glyph_to_svg(&g)
            .replace(r#"<?xml version="1.0" encoding="UTF-8"?>"#, "")
            .replace("<svg", "<g")
            .replace("</svg>", "</g>");
        svg.push_str(&format!(
            "<g transform=\"translate({x},8) scale(0.08)\" stroke=\"#D8EFE6\">{inner}</g>\n"
        ));
    }
    svg.push_str("</svg>\n");
    Ok(svg)
}

fn cmd_batch_encode(
    dir: PathBuf,
    output: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = output.unwrap_or_else(|| dir.clone());
    fs::create_dir_all(&out_dir)?;
    let mut n = 0usize;
    let mut entries: Vec<_> = fs::read_dir(&dir)?.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());
    for ent in entries {
        let path = ent.path();
        if path.extension().and_then(|e| e.to_str()) != Some("txt") {
            continue;
        }
        let text = fs::read_to_string(&path)?;
        let doc = DskDocument::encode_text(text.trim_end(), Mode::Encoded)?;
        let stem = path.file_stem().unwrap_or_default();
        let dest = out_dir.join(stem).with_extension("dsk");
        fs::write(&dest, doc.to_bytes()?)?;
        println!("{} → {}", path.display(), dest.display());
        n += 1;
    }
    println!("batch encode — {n} file(s)");
    if n == 0 {
        return Err("batch encode: no .txt files found".into());
    }
    Ok(())
}

fn cmd_batch_validate(dir: PathBuf, json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut n = 0usize;
    let mut failed = 0usize;
    let mut entries: Vec<_> = fs::read_dir(&dir)?.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());
    let mut reports = Vec::new();
    for ent in entries {
        let path = ent.path();
        if path.extension().and_then(|e| e.to_str()) != Some("dsk") {
            continue;
        }
        n += 1;
        let raw = fs::read(&path)?;
        let doc = DskDocument::parse(&raw)?;
        let report = doc.validate();
        if json {
            reports.push(serde_json::json!({
                "path": path.display().to_string(),
                "report": serde_json::from_str::<serde_json::Value>(&report.to_json()?)?,
            }));
        } else {
            let mark = if report.ok { "OK" } else { "FAIL" };
            println!("{mark}  {}", path.display());
        }
        if !report.ok {
            failed += 1;
        }
    }
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "schema": "doldskrift.batch.validate/1",
                "count": n,
                "failed": failed,
                "ok": failed == 0 && n > 0,
                "files": reports,
            }))?
        );
    } else {
        println!("batch validate — {n} file(s), {failed} failed");
    }
    if n == 0 {
        return Err("batch validate: no .dsk files found".into());
    }
    if failed > 0 {
        return Err("batch validate: one or more files failed".into());
    }
    Ok(())
}

fn cmd_self_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut passed = 0usize;
    let mut failed = 0usize;

    // 1) Open encode → validate → decode
    {
        let plain = "self-test hello";
        match (|| -> Result<(), Box<dyn std::error::Error>> {
            let doc = DskDocument::encode_text(plain, Mode::Encoded)?;
            let v = doc.validate();
            if !v.ok {
                return Err("validate failed".into());
            }
            let back = doc.decode_text()?;
            if back != plain {
                return Err("round-trip mismatch".into());
            }
            Ok(())
        })() {
            Ok(()) => {
                println!("✓ encode/decode/validate round-trip");
                passed += 1;
            }
            Err(e) => {
                println!("✗ encode/decode/validate round-trip: {e}");
                failed += 1;
            }
        }
    }

    // 2) Protected stub refuses plaintext decode
    {
        match (|| -> Result<(), Box<dyn std::error::Error>> {
            let doc = DskDocument::encode_protected_stub("secret")?;
            match doc.decode_text() {
                Err(_) => Ok(()),
                Ok(_) => Err("Protected decode unexpectedly succeeded".into()),
            }
        })() {
            Ok(()) => {
                println!("✓ protected stub refuses decode");
                passed += 1;
            }
            Err(e) => {
                println!("✗ protected refuse: {e}");
                failed += 1;
            }
        }
    }

    // 3) inspect JSON schema id
    {
        match (|| -> Result<(), Box<dyn std::error::Error>> {
            let doc = DskDocument::encode_text("x", Mode::Encoded)?;
            let j = doc.inspect().to_json()?;
            let v: serde_json::Value = serde_json::from_str(&j)?;
            if v["schema"] != "doldskrift.inspect/1" {
                return Err(format!("unexpected schema {}", v["schema"]).into());
            }
            Ok(())
        })() {
            Ok(()) => {
                println!("✓ inspect JSON envelope doldskrift.inspect/1");
                passed += 1;
            }
            Err(e) => {
                println!("✗ inspect schema: {e}");
                failed += 1;
            }
        }
    }

    println!("self-test — {passed} passed, {failed} failed");
    if failed > 0 {
        return Err("self-test failed".into());
    }
    Ok(())
}

fn cmd_schema(json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let envelopes = schema_envelopes();
    if json {
        println!("{}", serde_json::to_string_pretty(&envelopes)?);
    } else {
        println!("dold schema — agent JSON envelopes\n");
        for env in &envelopes["envelopes"].as_array().unwrap().clone() {
            println!(
                "  {} — {}",
                env["name"].as_str().unwrap_or("?"),
                env["command"].as_str().unwrap_or("?")
            );
            if let Some(fields) = env["fields"].as_array() {
                for f in fields {
                    println!(
                        "      · {} ({})",
                        f["name"].as_str().unwrap_or("?"),
                        f["type"].as_str().unwrap_or("?")
                    );
                }
            }
            println!();
        }
        println!("Honesty: inspect never embeds plaintext; digests ≠ signatures.");
    }
    Ok(())
}

pub fn schema_envelopes() -> serde_json::Value {
    serde_json::json!({
        "schema": "doldskrift.cli.schema/1",
        "envelopes": [
            {
                "name": "doldskrift.inspect/1",
                "command": "dold inspect --json",
                "fields": [
                    {"name": "schema", "type": "string"},
                    {"name": "magic", "type": "string"},
                    {"name": "protocol", "type": "string"},
                    {"name": "protocol_version", "type": "u8"},
                    {"name": "mode", "type": "string"},
                    {"name": "profile", "type": "string (open|neural|protected)"},
                    {"name": "payload_bytes", "type": "usize"},
                    {"name": "checksum_valid", "type": "bool"},
                    {"name": "meta_keys", "type": "string[]"},
                    {"name": "agent_hints", "type": "string[]"},
                    {"name": "protected_stub", "type": "bool?"},
                    {"name": "readable", "type": "string?"}
                ]
            },
            {
                "name": "doldskrift.validate/1",
                "command": "dold validate --json",
                "fields": [
                    {"name": "schema", "type": "string"},
                    {"name": "ok", "type": "bool"},
                    {"name": "profile", "type": "string"},
                    {"name": "protocol", "type": "string"},
                    {"name": "mode", "type": "string"},
                    {"name": "checks", "type": "{id, detail, ok, severity?}[]"},
                    {"name": "notes", "type": "string[]"}
                ]
            },
            {
                "name": "doldskrift.batch.validate/1",
                "command": "dold batch validate --json",
                "fields": [
                    {"name": "schema", "type": "string"},
                    {"name": "count", "type": "usize"},
                    {"name": "failed", "type": "usize"},
                    {"name": "ok", "type": "bool"},
                    {"name": "files", "type": "{path, report}[]"}
                ]
            },
            {
                "name": "doldskrift.error/1",
                "command": "any (stderr when DOLD_JSON_ERRORS=1)",
                "fields": [
                    {"name": "schema", "type": "string"},
                    {"name": "code", "type": "string"},
                    {"name": "message", "type": "string"},
                    {"name": "hint", "type": "string?"},
                    {"name": "command", "type": "string?"}
                ]
            },
            {
                "name": "doldskrift.config/1",
                "command": "dold config show --json",
                "fields": [
                    {"name": "schema", "type": "string"},
                    {"name": "default_mode", "type": "string"},
                    {"name": "seed", "type": "string?"},
                    {"name": "json", "type": "bool"},
                    {"name": "path", "type": "string"}
                ]
            }
        ]
    })
}

fn cmd_init(dir: PathBuf, sample: bool) -> Result<(), Box<dyn std::error::Error>> {
    let root = dir.join(".doldskrift");
    fs::create_dir_all(&root)?;
    let readme = root.join("README.md");
    if !readme.exists() {
        fs::write(
            &readme,
            "# .doldskrift\n\n\
Local Doldskrift project scaffold.\n\n\
- Open encode/decode is **not encryption**.\n\
- Protected containers are refuse stubs until AEAD ships.\n\
- Defaults: `config.json` (`dold config show`).\n\
- See https://github.com/doldskrift/doldskrift and SECURITY.md.\n\n\
Quick start:\n\n\
```bash\ndold guide\ndold demo -o ./out\ndold pipeline --text \"hello agent\" -o message.dsk\ndold validate message.dsk\ndold decode message.dsk\n```\n",
        )?;
    }
    let ignore = root.join("gitignore.sample");
    if !ignore.exists() {
        fs::write(&ignore, "*.dsk\n!fixtures/**/*.dsk\n")?;
    }
    // Ensure a config exists alongside the scaffold.
    let _ = write_default_config(&dir, false);

    if sample {
        let sample_txt = dir.join("sample.txt");
        if !sample_txt.exists() {
            fs::write(&sample_txt, "Hello from Doldskrift sample project.\n")?;
        }
        let text = fs::read_to_string(&sample_txt)?;
        let doc = DskDocument::encode_text(text.trim_end(), Mode::Encoded)?;
        let sample_dsk = dir.join("sample.dsk");
        fs::write(&sample_dsk, doc.to_bytes()?)?;
        println!("wrote {}", sample_txt.display());
        println!("wrote {}", sample_dsk.display());
    }

    println!("initialized {}", root.display());
    let _ = io::stdout().flush();
    Ok(())
}

fn cmd_guide(topic: &str) -> Result<(), Box<dyn std::error::Error>> {
    let t = topic.trim().to_ascii_lowercase();
    let body = match t.as_str() {
        "quickstart" | "start" | "q" => GUIDE_QUICKSTART,
        "honesty" | "security" | "profiles" => GUIDE_HONESTY,
        "surfaces" | "demos" => GUIDE_SURFACES,
        "agents" | "json" => GUIDE_AGENTS,
        "swedish" | "name" | "etymology" => GUIDE_SWEDISH,
        "architecture" | "platform" | "arch" => GUIDE_ARCHITECTURE,
        "help" | "topics" | "list" => GUIDE_TOPICS,
        other => {
            return Err(format!(
                "unknown guide topic '{other}' — try: quickstart, honesty, surfaces, agents, swedish, architecture"
            )
            .into());
        }
    };
    print!("{body}");
    Ok(())
}

const GUIDE_TOPICS: &str = "\
dold guide — topics
  quickstart      First encode / decode / doctor
  honesty         Open · Neural · Protected (not crypto theater)
  surfaces        Unexpected Open visual demos
  agents          JSON envelopes + DOLD_JSON_ERRORS
  swedish         Name etymology and brand voice
  architecture    Platform layers (DSK · MGE · DVE · Gate · LSG)

Tip: dold commands · dold demo · docs/README.md
";

const GUIDE_QUICKSTART: &str = "\
dold guide quickstart
═════════════════════
Doldskrift (Swedish dold + skrift → concealed script) encodes structured
information as machine glyphs. Open mode is representation — not encryption.

  cargo install --path crates/doldskrift-cli
  dold doctor
  dold self-test
  dold pipeline --text \"Hello from Doldskrift\" -o message.dsk
  dold validate message.dsk
  dold decode message.dsk
  dold demo -o ./dold-demo

Next: dold guide honesty · dold convert --help · docs/getting-started.md
";

const GUIDE_HONESTY: &str = "\
dold guide honesty
══════════════════
One platform · three profiles:

  Open       DSK/1–2 codec + demos. Visual opacity ≠ confidentiality.
  Neural     LSG + compatible reader (foundation stubs). Still not encryption.
  Protected  AEAD + Gate when it ships. Today: refuse stubs only.

Do not claim OCR bake-offs or \"AI-only secrecy\" without measured evidence.
Hashes on surfaces are integrity marks, not signatures.
See SECURITY.md and docs/product/protected-mode.md.
";

const GUIDE_SURFACES: &str = "\
dold guide surfaces
═══════════════════
Open visual surfaces still carry a recoverable DSK channel:

  dold postcard / ambient / seal / mesh / radio / echo / …
  dold surfaces list
  dold convert --from text --to postcard --text \"hi\" -o card.svg
  dold convert --from postcard --to dsk -i card.svg -o card.dsk

Site gallery: site/surfaces.html (GitHub Pages).
";

const GUIDE_AGENTS: &str = "\
dold guide agents
═════════════════
Stable JSON envelopes (dold schema):

  dold inspect --json     → doldskrift.inspect/1
  dold validate --json    → doldskrift.validate/1
  dold config show --json → doldskrift.config/1

Structured failures (opt-in):

  set DOLD_JSON_ERRORS=1
  # stderr → doldskrift.error/1 { code, message, hint? }

Never treat Open/Neural artifacts as secrets.
";

const GUIDE_SWEDISH: &str = "\
dold guide swedish
══════════════════
Name: dold (hidden / concealed) + skrift (writing / script)
     → \"concealed script\" — machine-written meaning, not secrecy.

Tagline: Maskinskriven betydelse — inte hemlighet.
The logo is glyph DSK_PROJECT_MARK (U+E1F0), not decoration.

  dold about
  dold museum
  dold logo
";

const GUIDE_ARCHITECTURE: &str = "\
dold guide architecture
═══════════════════════
DATA → DSK → MGE → VISUAL MEDIUM → DVE → DSK → DATA

Layers: Semantic · Protocol · Visual · Recognition
Names:  DSK (protocol) · MGE (glyphs) · DVE (vision)
        LSG (Neural grammar) · Gate (Protected runtime)
        Forge (alphabet research) · Lens / Lab (demos & CLI)

Rust owns the truth (5 crates). JS adapters speak the same constants.
See ARCHITECTURE.md and docs/README.md.
";

fn config_path(dir: &Path) -> PathBuf {
    dir.join(".doldskrift").join("config.json")
}

fn default_config_value(path: &Path) -> serde_json::Value {
    serde_json::json!({
        "schema": "doldskrift.config/1",
        "default_mode": "encoded",
        "seed": null,
        "json": false,
        "path": path.display().to_string(),
        "notes": [
            "Defaults only — explicit CLI flags always win.",
            "Open/Neural ≠ encryption. Protected AEAD not shipping."
        ]
    })
}

fn load_config(dir: &Path) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let path = config_path(dir);
    if path.is_file() {
        let raw = fs::read_to_string(&path)?;
        let mut v: serde_json::Value = serde_json::from_str(&raw)?;
        v["path"] = serde_json::Value::String(path.display().to_string());
        if v.get("schema").is_none() {
            v["schema"] = serde_json::Value::String("doldskrift.config/1".into());
        }
        Ok(v)
    } else {
        Ok(default_config_value(&path))
    }
}

fn write_default_config(dir: &Path, force: bool) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let root = dir.join(".doldskrift");
    fs::create_dir_all(&root)?;
    let path = config_path(dir);
    if path.exists() && !force {
        return Ok(path);
    }
    let mut v = default_config_value(&path);
    // Don't persist absolute path noise as the source of truth.
    if let Some(obj) = v.as_object_mut() {
        obj.remove("path");
        obj.remove("notes");
    }
    fs::write(&path, serde_json::to_string_pretty(&v)? + "\n")?;
    Ok(path)
}

fn cmd_config(command: ConfigCommands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        ConfigCommands::Show { json } => {
            let v = load_config(Path::new("."))?;
            if json {
                println!("{}", serde_json::to_string_pretty(&v)?);
            } else {
                println!("dold config — project defaults\n");
                println!("  path:          {}", v["path"].as_str().unwrap_or("?"));
                println!(
                    "  default_mode:  {}",
                    v["default_mode"].as_str().unwrap_or("encoded")
                );
                println!(
                    "  seed:          {}",
                    v["seed"].as_str().unwrap_or("(none)")
                );
                println!("  json:          {}", v["json"].as_bool().unwrap_or(false));
                println!("\nExplicit CLI flags always override these defaults.");
                println!("Open/Neural ≠ encryption.");
            }
            Ok(())
        }
        ConfigCommands::Path { dir } => {
            println!("{}", config_path(&dir).display());
            Ok(())
        }
        ConfigCommands::Init { dir, force } => {
            let path = write_default_config(&dir, force)?;
            println!("wrote {}", path.display());
            Ok(())
        }
        ConfigCommands::Set { key, value, dir } => {
            let path = write_default_config(&dir, false)?;
            let mut v = load_config(&dir)?;
            match key.as_str() {
                "default_mode" | "mode" => {
                    let mode = value.to_ascii_lowercase();
                    if !matches!(mode.as_str(), "visual" | "encoded" | "session") {
                        return Err(
                            "config set: default_mode must be visual|encoded|session (not protected)"
                                .into(),
                        );
                    }
                    v["default_mode"] = serde_json::Value::String(mode);
                }
                "seed" => {
                    if value.is_empty() || value == "null" || value == "-" {
                        v["seed"] = serde_json::Value::Null;
                    } else {
                        v["seed"] = serde_json::Value::String(value);
                    }
                }
                "json" => {
                    let b = matches!(
                        value.to_ascii_lowercase().as_str(),
                        "1" | "true" | "yes" | "on"
                    );
                    v["json"] = serde_json::Value::Bool(b);
                }
                other => {
                    return Err(format!(
                        "config set: unknown key '{other}' (default_mode|seed|json)"
                    )
                    .into());
                }
            }
            if let Some(obj) = v.as_object_mut() {
                obj.remove("path");
                obj.remove("notes");
            }
            fs::write(&path, serde_json::to_string_pretty(&v)? + "\n")?;
            println!("updated {} → {}", key, path.display());
            Ok(())
        }
    }
}

fn cmd_convert(
    from: ConvertFormat,
    to: ConvertFormat,
    input: Option<PathBuf>,
    text: Option<String>,
    output: Option<PathBuf>,
    seed: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    if from == to {
        return Err("convert: --from and --to must differ".into());
    }
    let plaintext = match from {
        ConvertFormat::Text => {
            if let Some(t) = text {
                t
            } else {
                String::from_utf8(read_bytes(input)?)?
            }
        }
        ConvertFormat::Dsk => {
            let raw = read_bytes(input)?;
            let doc = DskDocument::parse(&raw)?;
            if doc.header.mode == Mode::Protected {
                return Err(
                    "convert: Protected containers refuse Open conversion (no AEAD plaintext path)"
                        .into(),
                );
            }
            doc.decode_text()?
        }
        ConvertFormat::Postcard | ConvertFormat::Ambient | ConvertFormat::Seal => {
            let path = input.ok_or("convert: surface --from requires -i <svg>")?;
            let svg = fs::read_to_string(path)?;
            let carrier = extract_carrier_from_svg(&svg).ok_or("convert: no dsk-carrier in SVG")?;
            doldskrift::decode(&carrier.encoded)?
        }
    };
    if plaintext.is_empty() {
        return Err("convert: empty payload".into());
    }

    let out_path = match to {
        ConvertFormat::Text => output.clone(),
        ConvertFormat::Dsk => output
            .clone()
            .or_else(|| Some(PathBuf::from("converted.dsk"))),
        ConvertFormat::Postcard => output
            .clone()
            .or_else(|| Some(PathBuf::from("converted-postcard.svg"))),
        ConvertFormat::Ambient => output
            .clone()
            .or_else(|| Some(PathBuf::from("converted-ambient.svg"))),
        ConvertFormat::Seal => output
            .clone()
            .or_else(|| Some(PathBuf::from("converted-seal.svg"))),
    };

    match to {
        ConvertFormat::Text => {
            write_out(out_path.clone(), plaintext.as_bytes())?;
        }
        ConvertFormat::Dsk => {
            let doc = DskDocument::encode_text(&plaintext, Mode::Encoded)?;
            write_out(out_path.clone(), &doc.to_bytes()?)?;
        }
        ConvertFormat::Postcard => {
            let encoded = doldskrift::encode(&plaintext)?;
            let digest = hex::encode(Sha256::digest(plaintext.as_bytes()));
            let svg = render_postcard_svg(&encoded, seed, &digest);
            write_out(out_path.clone(), svg.as_bytes())?;
        }
        ConvertFormat::Ambient => {
            let encoded = doldskrift::encode(&plaintext)?;
            let digest = hex::encode(Sha256::digest(plaintext.as_bytes()));
            let svg = render_ambient_constellation(&encoded, seed, Some(&digest));
            write_out(out_path.clone(), svg.as_bytes())?;
        }
        ConvertFormat::Seal => {
            let encoded = doldskrift::encode(&plaintext)?;
            let digest = hex::encode(Sha256::digest(plaintext.as_bytes()));
            let svg = render_seal_svg(&encoded, &digest, seed)?;
            write_out(out_path.clone(), svg.as_bytes())?;
        }
    }
    if let Some(ref p) = out_path {
        eprintln!(
            "convert — {} → {} → {}",
            format_name(from),
            format_name(to),
            p.display()
        );
    } else {
        eprintln!("convert — {} → {} OK", format_name(from), format_name(to));
    }
    Ok(())
}

fn format_name(f: ConvertFormat) -> &'static str {
    match f {
        ConvertFormat::Text => "text",
        ConvertFormat::Dsk => "dsk",
        ConvertFormat::Postcard => "postcard",
        ConvertFormat::Ambient => "ambient",
        ConvertFormat::Seal => "seal",
    }
}

fn write_out(path: Option<PathBuf>, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    match path {
        None => {
            io::stdout().write_all(bytes)?;
            if !bytes.ends_with(b"\n") && std::str::from_utf8(bytes).is_ok() {
                println!();
            }
            Ok(())
        }
        Some(p) => {
            if let Some(parent) = p.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }
            fs::write(p, bytes)?;
            Ok(())
        }
    }
}

fn cmd_demo(out: PathBuf, text: String) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(&out)?;
    let plaintext = if text.trim().is_empty() {
        "Hello from Doldskrift.".to_string()
    } else {
        text
    };

    let doc = DskDocument::encode_text(&plaintext, Mode::Encoded)?;
    let dsk_path = out.join("message.dsk");
    fs::write(&dsk_path, doc.to_bytes()?)?;

    let encoded = doldskrift::encode(&plaintext)?;
    let digest = hex::encode(Sha256::digest(plaintext.as_bytes()));
    let postcard = render_postcard_svg(&encoded, 1, &digest);
    let postcard_path = out.join("postcard.svg");
    fs::write(&postcard_path, postcard)?;

    let mark = generate_project_mark(1)?;
    let mark_svg = brand_mark_tile(&glyph_to_svg(&mark));
    let mark_path = out.join("logo-mark.svg");
    fs::write(&mark_path, mark_svg)?;

    let readme = format!(
        "# Doldskrift demo pack\n\n\
Swedish *dold* + *skrift* → concealed script.\n\
**Open mode — not encryption.** Digests ≠ signatures.\n\n\
| File | Role |\n|------|------|\n\
| `message.dsk` | Open container |\n\
| `postcard.svg` | Machine postcard (decode: `dold postcard --read`) |\n\
| `logo-mark.svg` | Project mark `DSK_PROJECT_MARK` (U+E1F0) |\n\n\
```bash\ndold validate message.dsk\ndold decode message.dsk\ndold postcard --read postcard.svg\ndold convert --from postcard --to dsk -i postcard.svg -o roundtrip.dsk\n```\n\n\
Plaintext used: `{plaintext}`\n"
    );
    fs::write(out.join("README.md"), readme)?;

    let report = doc.validate();
    if !report.ok {
        return Err("demo: validation failed".into());
    }

    println!("dold demo — Open product pack");
    println!("  out:       {}", out.display());
    println!("  wrote:     {}", dsk_path.display());
    println!("  wrote:     {}", postcard_path.display());
    println!("  wrote:     {}", mark_path.display());
    println!("  validate:  OK");
    println!("  honesty:   Open ≠ encryption · Neural ≠ encryption · Protected = refuse stubs");
    println!(
        "  next:      dold postcard --read {}",
        postcard_path.display()
    );
    Ok(())
}

fn brand_mark_tile(inner_glyph_svg: &str) -> String {
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
  <rect width="128" height="128" rx="20" fill="#F7F4EF"/>
  <g transform="translate(14,14) scale(0.1)" fill="none" stroke="#12161A" stroke-width="36" stroke-linecap="square" stroke-linejoin="miter">
{body}
  </g>
  <circle cx="104" cy="24" r="4" fill="#0F6B5C"/>
</svg>
"##
    )
}

fn read_bytes(path: Option<PathBuf>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_lists_workflow_commands() {
        let t = catalog_text();
        assert!(t.contains("pipeline"));
        assert!(t.contains("self-test"));
        assert!(t.contains("batch"));
        assert!(t.contains("guide"));
        assert!(t.contains("convert"));
        assert!(t.contains("demo"));
        assert!(t.contains("Document lifecycle"));
    }

    #[test]
    fn guide_quickstart_prints() {
        cmd_guide("quickstart").unwrap();
        cmd_guide("honesty").unwrap();
    }

    #[test]
    fn convert_text_to_postcard_and_back() {
        let tmp = std::env::temp_dir().join(format!("dold-conv-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        let svg = tmp.join("card.svg");
        let dsk = tmp.join("card.dsk");
        cmd_convert(
            ConvertFormat::Text,
            ConvertFormat::Postcard,
            None,
            Some("convert-me".into()),
            Some(svg.clone()),
            1,
        )
        .unwrap();
        assert!(svg.is_file());
        cmd_convert(
            ConvertFormat::Postcard,
            ConvertFormat::Dsk,
            Some(svg),
            None,
            Some(dsk.clone()),
            1,
        )
        .unwrap();
        let doc = DskDocument::parse(&fs::read(&dsk).unwrap()).unwrap();
        assert_eq!(doc.decode_text().unwrap(), "convert-me");
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn demo_writes_pack() {
        let tmp = std::env::temp_dir().join(format!("dold-demo-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        cmd_demo(tmp.clone(), "demo text".into()).unwrap();
        assert!(tmp.join("message.dsk").is_file());
        assert!(tmp.join("postcard.svg").is_file());
        assert!(tmp.join("logo-mark.svg").is_file());
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn config_roundtrip() {
        let tmp = std::env::temp_dir().join(format!("dold-cfg-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        cmd_config(ConfigCommands::Init {
            dir: tmp.clone(),
            force: true,
        })
        .unwrap();
        cmd_config(ConfigCommands::Set {
            key: "default_mode".into(),
            value: "session".into(),
            dir: tmp.clone(),
        })
        .unwrap();
        let v = load_config(&tmp).unwrap();
        assert_eq!(v["default_mode"], "session");
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn surfaces_list_mentions_postcard() {
        let t = surfaces_list_text();
        assert!(t.contains("postcard"));
        assert!(t.contains("kaleidoscope"));
    }

    #[test]
    fn schema_envelopes_have_stable_names() {
        let v = schema_envelopes();
        assert_eq!(v["schema"], "doldskrift.cli.schema/1");
        let names: Vec<&str> = v["envelopes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| e["name"].as_str().unwrap())
            .collect();
        assert!(names.contains(&"doldskrift.inspect/1"));
        assert!(names.contains(&"doldskrift.validate/1"));
    }

    #[test]
    fn self_test_passes() {
        cmd_self_test().expect("self-test");
    }

    #[test]
    fn init_scaffolds_dir() {
        let tmp = std::env::temp_dir().join(format!("dold-init-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        cmd_init(tmp.clone(), true).unwrap();
        assert!(tmp.join(".doldskrift/README.md").is_file());
        assert!(tmp.join("sample.dsk").is_file());
        let _ = fs::remove_dir_all(&tmp);
    }
}
