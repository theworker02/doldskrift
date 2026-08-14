//! `dold` — Doldskrift Lab CLI (Open · Neural · Protected platform surface).

mod cli_error;
mod conformance;
mod explain;
mod neural_cli;
mod surfaces;
mod vision;
mod workflows;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Shell};
use doldskrift::neural::GateCapability;
use doldskrift::DSK_PROJECT_MARK;
use doldskrift::{
    gate_open, CapabilitySet, DskDocument, HandshakeOffer, Mode, Session, DEFAULT_CAPABILITIES,
};
use doldskrift_font::{
    build_font_source, generate_alphabet, generate_glyph, generate_project_mark, glyph_to_svg,
    Density, FontBuildOptions, FontFamily, FONT_UNITS,
};
use doldskrift_vision::render_glyph_png_feature_map;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "dold",
    version,
    about = "Doldskrift — machine-native typography and encoding (Swedish: concealed script)",
    long_about = "Doldskrift (Swedish dold + skrift → concealed script) is NOT encryption. See SECURITY.md.\n\n\
Command groups (see also `dold commands` / `dold guide`):\n  \
  Document:   encode (e) · decode (d) · inspect · validate · fmt · verify · open\n  \
  Workflows:  guide · demo · convert · config · pipeline · batch · self-test · init\n  \
  Render:     render · specimen · logo · brand · font\n  \
  Vision:     scan · vision · bench\n  \
  Neural:     neural · dataset · reader · evaluate\n  \
  Identity:   doctor · about · funding · museum · handshake · completion\n  \
  Surfaces:   radio · ambient · postcard · kaleidoscope · …\n\n\
Examples:\n  \
  dold guide\n  \
  dold demo -o ./out\n  \
  dold commands\n  \
  echo \"hello agent\" | dold encode > message.dsk\n  \
  dold pipeline --text \"hello\" -o message.dsk\n  \
  dold convert --from text --to postcard --text \"hi\" -o card.svg\n  \
  dold self-test\n  \
  dold doctor"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode text into a `.dsk` container (or raw PUA text with --raw)
    #[command(visible_alias = "e")]
    Encode {
        input: Option<PathBuf>,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long, default_value = "encoded")]
        mode: ModeArg,
        #[arg(long)]
        seed: Option<String>,
        #[arg(long)]
        session_id: Option<String>,
        #[arg(long)]
        raw: bool,
    },
    /// Decode a `.dsk` container or raw encoded text
    #[command(visible_alias = "d")]
    Decode {
        input: Option<PathBuf>,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long)]
        seed: Option<String>,
    },
    /// Gate-mediated open of a Protected object (stub → AccessDenied without AEAD)
    Open {
        input: Option<PathBuf>,
        /// Capability id (e.g. `dsk.capability.export.plaintext`); still AccessDenied until AEAD
        #[arg(long)]
        capability: Option<String>,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Inspect a `.dsk` document without revealing the payload
    Inspect {
        input: Option<PathBuf>,
        #[arg(long)]
        reveal: bool,
        #[arg(long)]
        json: bool,
    },
    /// Explain how a document works (educational; no payload by default)
    Explain {
        input: Option<PathBuf>,
        #[arg(long)]
        reveal: bool,
    },
    /// Verify container checksum and structure
    Verify { input: Option<PathBuf> },
    /// Validate `.dsk` structure, mode/version, and decode expectations (`--json` for agents)
    Validate {
        input: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    /// Rewrite a `.dsk` in canonical form (stable header JSON + checksum)
    Fmt {
        input: Option<PathBuf>,
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Check only — exit non-zero if reformatting would change bytes
        #[arg(long)]
        check: bool,
    },
    /// Print version (use `--verbose` for build metadata)
    Version {
        #[arg(long)]
        verbose: bool,
    },
    /// Generate shell completions (bash, zsh, fish, powershell, elvish)
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Render text through a mode
    Render {
        input: Option<PathBuf>,
        #[arg(short, long, default_value = "encoded")]
        mode: ModeArg,
        #[arg(long)]
        seed: Option<String>,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, default_value = "normal")]
        density: String,
        #[arg(long)]
        debug: bool,
        /// Unicode braille / block art for terminals (no browser required)
        #[arg(long)]
        tty: bool,
    },
    /// Pack arbitrary binary into `.dsk`
    Pack {
        input: Option<PathBuf>,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Unpack raw payload bytes from `.dsk`
    Unpack {
        input: Option<PathBuf>,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Font toolchain
    Font {
        #[command(subcommand)]
        command: FontCommands,
    },
    /// Experimental vision (alias of scan)
    Vision { image: PathBuf },
    /// Structural / vision scan pipeline
    Scan {
        image: PathBuf,
        #[arg(long, default_value = "structural")]
        engine: String,
        /// Compare observation graphs of two images / feature maps
        #[arg(long)]
        diff: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    /// Benchmarks
    Bench {
        #[command(subcommand)]
        command: BenchCommands,
    },
    /// Run DOLDSKRIFT/1 conformance checks
    Conformance {
        #[arg(long, default_value = "spec/vectors")]
        vectors: PathBuf,
    },
    /// Agent handshake helpers (text wire + optional visual strip)
    Handshake {
        #[command(subcommand)]
        command: HandshakeCommands,
    },
    /// Export the official project mark logo (`DSK_PROJECT_MARK` / U+E1F0)
    Logo {
        #[arg(long, short = 'o', default_value = "assets/brand/logo-mark.svg")]
        out: PathBuf,
        #[arg(long, default_value_t = 1)]
        seed: u64,
    },
    /// Render a glyph strip specimen (SVG) from text
    Specimen {
        input: Option<PathBuf>,
        #[arg(short, long, default_value = "encoded")]
        mode: ModeArg,
        #[arg(long)]
        seed: Option<String>,
        #[arg(short, long, default_value = "specimen.svg")]
        output: PathBuf,
        #[arg(long, default_value = "normal")]
        density: String,
    },
    /// Diagnose local toolchain / feature availability
    Doctor,
    /// Run Forge alphabet scoring (experimental)
    Forge {
        #[arg(long, default_value_t = 1)]
        seed: u64,
        #[arg(long, default_value = "standard")]
        profile: String,
        #[arg(long)]
        json: bool,
    },
    /// Path query into a semantic / JSON payload inside a document (experimental)
    Query {
        /// Path like `$.priority` or `/priority`
        path: String,
        /// Document / JSON file (stdin if omitted)
        #[arg(short, long)]
        input: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    /// Synthetic Neural dataset fixtures (Phase V foundation)
    Dataset {
        #[command(subcommand)]
        command: neural_cli::DatasetCommands,
    },
    /// Neural reader management (stubs; no weight auto-download)
    Reader {
        #[command(subcommand)]
        command: neural_cli::ReaderCommands,
    },
    /// DSK/3 Neural encode / render / reconstruct (Mock/Deterministic)
    Neural {
        #[command(subcommand)]
        command: neural_cli::NeuralCommands,
    },
    /// Evaluation harness stubs
    Evaluate {
        #[command(subcommand)]
        command: neural_cli::EvaluateCommands,
    },
    /// Agent Radio — exchange a tiny visual DSK packet between two agents
    Radio {
        #[command(subcommand)]
        command: surfaces::RadioCommands,
    },
    /// Self-describing document with SHA-256 glyph strip (integrity, not a signature)
    Echo {
        input: Option<PathBuf>,
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Verify an echo SVG produced by this command
        #[arg(long)]
        verify: Option<PathBuf>,
    },
    /// Protocol archaeology — historical modes / profiles with sample glyphs
    Museum {
        #[arg(long)]
        json: bool,
    },
    /// Name etymology, profiles, and project identity (Swedish-rooted)
    About {
        #[arg(long)]
        json: bool,
    },
    /// Sponsor / thanks.dev funding URLs (open-source sustainability)
    Funding {
        #[arg(long)]
        json: bool,
    },
    /// Export a brand kit (mark SVG, tokens, sync checklist)
    Brand {
        /// Output directory for kit files
        #[arg(long, short = 'o', default_value = "brand-kit")]
        out: PathBuf,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Print manifest JSON only (no files written)
        #[arg(long)]
        json: bool,
    },
    /// Ambient constellation SVG (Open visual channel — not steganography)
    Ambient {
        input: Option<PathBuf>,
        #[arg(short, long, default_value = "ambient.svg")]
        output: PathBuf,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Decode plaintext from an ambient / flicker SVG carrier
        #[arg(long)]
        decode: Option<PathBuf>,
        /// SMIL twinkle animation on stars (decorative; decode still uses carrier)
        #[arg(long)]
        animate: bool,
    },
    /// Temporal flicker animated SVG (glyph order as frame sequence)
    Flicker {
        input: Option<PathBuf>,
        #[arg(short, long, default_value = "flicker.svg")]
        output: PathBuf,
        #[arg(long, default_value_t = 1)]
        seed: u64,
    },
    /// Living document — epoch-varying MGE/4 visuals; Open + baseline still recover
    Live {
        input: Option<PathBuf>,
        #[arg(short, long, default_value = "living.svg")]
        output: PathBuf,
        #[arg(long, default_value_t = 1)]
        epoch: u64,
    },
    /// Neural constellation / mesh map from LSG plan
    Mesh {
        input: Option<PathBuf>,
        #[arg(short, long, default_value = "mesh.svg")]
        output: PathBuf,
        /// Export interactive HTML one-pager with node inspection
        #[arg(long)]
        html: bool,
    },
    /// Machine postcard SVG (constellation + digest + glyph strip)
    Postcard {
        input: Option<PathBuf>,
        #[arg(short, long, default_value = "postcard.svg")]
        output: PathBuf,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Recover plaintext (+ verify sha256) from a postcard SVG
        #[arg(long)]
        read: Option<PathBuf>,
    },
    /// Two interleaved Open glyph streams (A/B) that decode independently
    Duet {
        /// Channel A input (file or stdin)
        #[arg(long = "a")]
        channel_a: Option<PathBuf>,
        /// Channel B input file (default: ack of A length)
        #[arg(long = "b")]
        channel_b: Option<PathBuf>,
        #[arg(short, long, default_value = "duet.svg")]
        output: PathBuf,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Decode both channels from a duet SVG
        #[arg(long)]
        decode: Option<PathBuf>,
    },
    /// Spectrogram-style Open strip (symbol id → bar height)
    Spectrogram {
        input: Option<PathBuf>,
        #[arg(short, long, default_value = "spectrogram.svg")]
        output: PathBuf,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        #[arg(long)]
        decode: Option<PathBuf>,
    },
    /// Bind payload SHA-256 into glyph margin marks (integrity ≠ signature)
    Notarize {
        input: Option<PathBuf>,
        #[arg(short, long, default_value = "notarize.svg")]
        output: PathBuf,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Verify a notarize SVG (hash match; not a signature)
        #[arg(long)]
        verify: Option<PathBuf>,
    },
    /// Newer Open surfaces (flattened so `dold kaleidoscope` stays top-level)
    #[command(flatten)]
    Extra(surfaces::ExtraSurfaceCommands),
    /// Workflows / catalog (flattened — keeps main enum stack-safe)
    #[command(flatten)]
    Workflow(workflows::WorkflowCommands),
}

#[derive(Subcommand, Debug)]
enum FontCommands {
    Build {
        #[arg(long, default_value = "fonts/generated")]
        out: PathBuf,
        #[arg(long, default_value_t = 1)]
        seed: u64,
    },
    Inspect {
        #[arg(long, default_value = "fonts/generated/glyphs.json")]
        catalog: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
enum BenchCommands {
    /// Real structural recognition benchmarks
    Vision {
        #[arg(long, default_value_t = 32)]
        samples: usize,
    },
}

#[derive(Subcommand, Debug)]
enum HandshakeCommands {
    /// Print reference `DSK?` offer (ASCII wire)
    Offer,
    /// Negotiate against an offer file (or reference)
    Negotiate { offer: Option<PathBuf> },
    /// Visual Open capability strip SVG agents can scan (+ prints wire)
    Visual {
        #[arg(short, long, default_value = "handshake.svg")]
        output: PathBuf,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Optional offer wire file
        offer: Option<PathBuf>,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ModeArg {
    Visual,
    Encoded,
    Session,
    /// Protected stub container (opaque placeholder; not real AEAD)
    Protected,
}

impl From<ModeArg> for Mode {
    fn from(value: ModeArg) -> Self {
        match value {
            ModeArg::Visual => Mode::Visual,
            ModeArg::Encoded => Mode::Encoded,
            ModeArg::Session => Mode::Session,
            ModeArg::Protected => Mode::Protected,
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            if cli_error::json_errors_enabled() {
                let classified = cli_error::classify(&e.to_string());
                eprintln!("{}", classified.to_json());
            } else {
                eprintln!("dold: {e}");
            }
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let Some(command) = cli.command else {
        return surfaces::quiet_mode_fingerprint();
    };
    match command {
        Commands::Encode {
            input,
            output,
            mode,
            seed,
            session_id,
            raw,
        } => cmd_encode(input, output, mode.into(), seed, session_id, raw)?,
        Commands::Decode {
            input,
            output,
            seed,
        } => cmd_decode(input, output, seed)?,
        Commands::Open {
            input,
            capability,
            output,
        } => cmd_open(input, capability, output)?,
        Commands::Inspect {
            input,
            reveal,
            json,
        } => {
            let doc = DskDocument::parse(&read_input(input)?)?;
            if json {
                println!("{}", doc.inspect().to_json()?);
            } else {
                println!("{}", doc.inspect().display());
                if reveal {
                    if doc.is_protected() {
                        eprintln!(
                            "\n--reveal refused: Protected objects have no plaintext via inspect. Use `dold open` (Gate stub)."
                        );
                        return Err("Protected reveal refused".into());
                    } else if doc.is_binary_payload() {
                        println!("\nPayload: <binary {} bytes>", doc.payload_len());
                    } else {
                        println!("\nPayload (decoded):\n{}", doc.decode_text()?);
                    }
                }
            }
        }
        Commands::Explain { input, reveal } => {
            let doc = DskDocument::parse(&read_input(input)?)?;
            println!("{}", explain::explain_document(&doc, reveal)?);
        }
        Commands::Verify { input } => {
            let doc = DskDocument::parse(&read_input(input)?)?;
            doc.verify()?;
            println!("OK — checksum valid, structure well-formed");
        }
        Commands::Validate { input, json } => {
            let doc = DskDocument::parse(&read_input(input)?)?;
            let report = doc.validate();
            if json {
                println!("{}", report.to_json()?);
            } else {
                print!("{}", report.display());
            }
            if !report.ok {
                return Err("validation failed".into());
            }
        }
        Commands::Fmt {
            input,
            output,
            check,
        } => {
            let raw = read_input(input)?;
            let doc = DskDocument::parse(&raw)?;
            let formatted = doc.format_canonical()?;
            if check {
                if formatted == raw {
                    println!("OK — already canonical");
                } else {
                    return Err("would reformat (run `dold fmt` without --check)".into());
                }
            } else {
                write_output(output, &formatted)?;
                if formatted != raw {
                    eprintln!(
                        "rewrote canonical .dsk ({} → {} bytes)",
                        raw.len(),
                        formatted.len()
                    );
                } else {
                    eprintln!("already canonical ({} bytes)", formatted.len());
                }
            }
        }
        Commands::Version { verbose } => cmd_version(verbose)?,
        Commands::Completion { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(shell, &mut cmd, name, &mut io::stdout());
        }
        Commands::Render {
            input,
            mode,
            seed,
            output,
            density,
            debug,
            tty,
        } => {
            if tty {
                let bytes = read_input(input)?;
                let text = std::str::from_utf8(&bytes)?;
                let art = surfaces::encode_for_tty(text, mode.into(), seed.as_deref())?;
                write_output(output, art.as_bytes())?;
                if !art.ends_with('\n') {
                    println!();
                }
            } else {
                cmd_render(input, mode.into(), seed, output, &density, debug)?;
            }
        }
        Commands::Pack { input, output } => {
            let bytes = read_input(input)?;
            let doc = DskDocument::pack_binary(&bytes)?;
            write_output(output, &doc.to_bytes()?)?;
        }
        Commands::Unpack { input, output } => {
            let doc = DskDocument::parse(&read_input(input)?)?;
            write_output(output, doc.unpack_bytes())?;
        }
        Commands::Font { command } => match command {
            FontCommands::Build { out, seed } => {
                let result = build_font_source(&FontBuildOptions {
                    out_dir: out,
                    seed,
                    ..FontBuildOptions::default()
                })?;
                println!(
                    "Built {} glyphs (MGE/2) → {}",
                    result.glyph_count,
                    result.svg_font.display()
                );
            }
            FontCommands::Inspect { catalog } => {
                let v: serde_json::Value = serde_json::from_str(&fs::read_to_string(catalog)?)?;
                println!(
                    "Doldskrift Font Catalog\nFamily:        {}\nFont Version:  {}\nGlyph Engine:  MGE/{}\nSeed:          {}\nProject Mark:  U+{:04X}\nAlphabet:      {}",
                    v["family"].as_str().unwrap_or("?"),
                    v["font_version"],
                    v["glyph_engine"],
                    v["seed"],
                    v["project_mark"].as_u64().unwrap_or(0) as u32,
                    v["alphabet"].as_array().map(|a| a.len()).unwrap_or(0),
                );
            }
        },
        Commands::Vision { image } => println!("{}", vision::analyze_image(&image)?),
        Commands::Scan {
            image,
            engine,
            diff,
            json,
        } => {
            if let Some(other) = diff {
                println!("{}", vision::diff_paths(&image, &other, &engine, json)?);
            } else {
                println!("{}", vision::scan_path(&image, &engine, json)?);
            }
        }
        Commands::Bench { command } => match command {
            BenchCommands::Vision { samples } => println!("{}", vision::run_bench(samples)?),
        },
        Commands::Conformance { vectors } => conformance::run(&vectors)?,
        Commands::Handshake { command } => match command {
            HandshakeCommands::Offer => print!("{}", HandshakeOffer::reference().to_wire()),
            HandshakeCommands::Negotiate { offer } => {
                let offer = if let Some(path) = offer {
                    HandshakeOffer::from_wire(&fs::read_to_string(path)?)?
                } else {
                    HandshakeOffer::reference()
                };
                let reply = doldskrift::negotiate(&offer, &CapabilitySet::reference())?;
                print!("{}", reply.to_wire());
                let _ = DEFAULT_CAPABILITIES;
            }
            HandshakeCommands::Visual {
                output,
                seed,
                offer,
            } => surfaces::cmd_handshake_visual(output, seed, offer)?,
        },
        Commands::Logo { out, seed } => {
            let glyph = generate_project_mark(seed)?;
            let svg = brand_logo_svg(&glyph_to_svg(&glyph));
            if let Some(parent) = out.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&out, svg)?;
            println!(
                "Wrote project mark U+{:04X} → {}",
                DSK_PROJECT_MARK,
                out.display()
            );
        }
        Commands::Specimen {
            input,
            mode,
            seed,
            output,
            density,
        } => cmd_specimen(input, mode.into(), seed, output, &density)?,
        Commands::Doctor => cmd_doctor()?,
        Commands::Forge {
            seed,
            profile,
            json,
        } => {
            let report = doldskrift_font::run_forge_generation(&doldskrift_font::ForgeConfig {
                seed,
                generations: 1,
                symbols: 256,
                profile,
            })?;
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("Alphabet: {}", report.alphabet_id);
                println!("Lifecycle: {:?}", report.lifecycle);
                println!(
                    "Machine Separation   {:.4}",
                    report.scores.machine_separation
                );
                println!(
                    "Visual Familiarity   {:.4}  (lower = less letter-like)",
                    report.scores.visual_familiarity
                );
                println!(
                    "Repetition Exposure  {:.4}",
                    report.scores.repetition_exposure
                );
                println!("Complexity           {:.2}", report.scores.complexity);
                println!("Seed                 {}", report.seed);
            }
        }
        Commands::Query { input, path, json } => {
            let bytes = read_input(input)?;
            let text = if bytes.starts_with(doldskrift::MAGIC) {
                let doc = DskDocument::parse(&bytes)?;
                doc.decode_text()?
            } else {
                String::from_utf8(bytes)?
            };
            // Prefer JSON path query when payload is JSON; else semantic decode
            let value: serde_json::Value =
                serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({ "text": text }));
            let found = query_json_path(&value, &path);
            if json {
                println!("{}", serde_json::to_string_pretty(&found)?);
            } else {
                println!("{found}");
            }
        }
        Commands::Dataset { command } => neural_cli::run_dataset(command)?,
        Commands::Reader { command } => neural_cli::run_reader(command)?,
        Commands::Neural { command } => neural_cli::run_neural(command)?,
        Commands::Evaluate { command } => neural_cli::run_evaluate(command)?,
        Commands::Radio { command } => surfaces::run_radio(command)?,
        Commands::Echo {
            input,
            output,
            verify,
        } => surfaces::cmd_echo(input, output, verify)?,
        Commands::Museum { json } => surfaces::cmd_museum(json)?,
        Commands::About { json } => surfaces::cmd_about(json)?,
        Commands::Funding { json } => surfaces::cmd_funding(json)?,
        Commands::Brand { out, seed, json } => surfaces::cmd_brand(out, seed, json)?,
        Commands::Ambient {
            input,
            output,
            seed,
            decode,
            animate,
        } => surfaces::cmd_ambient(input, output, decode, seed, animate)?,
        Commands::Flicker {
            input,
            output,
            seed,
        } => surfaces::cmd_flicker(input, output, seed)?,
        Commands::Live {
            input,
            output,
            epoch,
        } => surfaces::cmd_live(input, output, epoch)?,
        Commands::Mesh {
            input,
            output,
            html,
        } => surfaces::cmd_mesh(input, output, html)?,
        Commands::Postcard {
            input,
            output,
            seed,
            read,
        } => surfaces::cmd_postcard(input, output, seed, read)?,
        Commands::Duet {
            channel_a,
            channel_b,
            output,
            seed,
            decode,
        } => surfaces::cmd_duet(channel_a, channel_b, output, seed, decode)?,
        Commands::Spectrogram {
            input,
            output,
            seed,
            decode,
        } => surfaces::cmd_spectrogram(input, output, seed, decode)?,
        Commands::Notarize {
            input,
            output,
            seed,
            verify,
        } => surfaces::cmd_notarize(input, output, seed, verify)?,
        Commands::Extra(extra) => surfaces::run_extra(extra)?,
        Commands::Workflow(wf) => workflows::run(wf)?,
    }
    Ok(())
}

fn cmd_version(verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("dold {}", env!("CARGO_PKG_VERSION"));
    if verbose {
        let info = doldskrift::runtime::version_info();
        println!(
            "target:   {}",
            option_env!("DOLDSKRIFT_TARGET").unwrap_or("unknown")
        );
        println!(
            "profile:  {}",
            option_env!("DOLDSKRIFT_PROFILE").unwrap_or("unknown")
        );
        println!(
            "git:      {}",
            option_env!("DOLDSKRIFT_GIT").unwrap_or("unknown")
        );
        println!("rustc:    {}", rustc_version_line());
        println!(
            "core:     {} (protocol {}, font {}, mge {}, ffi={})",
            info.crate_version, info.protocol, info.font, info.glyph_engine, info.ffi
        );
        println!("features: default (no protected-crypto AEAD feature)");
        println!("honesty:  Open/Neural ≠ encryption; Protected AEAD not shipping");
        println!("funding:  dold funding · https://thanks.dev/u/gh/theworker02");
    }
    Ok(())
}

fn rustc_version_line() -> String {
    option_env!("DOLDSKRIFT_RUSTC")
        .unwrap_or("unknown")
        .to_string()
}

fn cmd_doctor() -> Result<(), Box<dyn std::error::Error>> {
    use doldskrift::neural::{
        compose_deterministic, DeterministicBaseline, ObservationGraph, ReaderBackend, ReaderStatus,
    };
    use doldskrift::semantic::Value;

    println!("Doldskrift doctor — is this install healthy?");
    println!("  version:          {}", env!("CARGO_PKG_VERSION"));
    println!(
        "  build:            {} / {}",
        option_env!("DOLDSKRIFT_PROFILE").unwrap_or("unknown"),
        option_env!("DOLDSKRIFT_GIT").unwrap_or("unknown")
    );
    println!("  workspace:        5 crates (doldskrift, cli, vision, font, bindings)");
    println!("  platform:         Open · Neural · Protected (one product architecture)");
    println!("  protocol:         DSK/1 · DSK/2 experimental · DSK/3 Neural+Protected stubs");
    println!("  glyph engine:     MGE/2 + MGE/4 research + MGE/5 SVG carrier");
    println!("  vision:           DVE/1 structural + DVE/2 scaffolds + scan --diff");
    println!(
        "  constants sync:   {}",
        doctor_constants_sync()
    );

    // Neural readiness — DeterministicBaseline exact path
    let neural = (|| -> Result<String, Box<dyn std::error::Error>> {
        let g = compose_deterministic(&Value::String("doctor".into()))?;
        let obs = ObservationGraph::from_latent_carrier(g);
        let out = DeterministicBaseline::new().reconstruct(&obs)?;
        Ok(match out.status {
            ReaderStatus::Reconstructed => {
                "ready (deterministic-baseline/1, no trained weights)".into()
            }
            other => format!("degraded ({other:?})"),
        })
    })();
    println!(
        "  neural:           {}",
        neural.unwrap_or_else(|e| format!("error ({e})"))
    );

    // Protected refuse path
    let prot = DskDocument::encode_protected_stub("probe")?;
    let refuse = prot.decode_text().is_err();
    let open = gate_open(&prot, None).is_err();
    println!(
        "  protected:        refuse paths {} (decode={}, open=AccessDenied stub; no AEAD)",
        if refuse && open { "OK" } else { "UNEXPECTED" },
        if refuse { "refused" } else { "LEAKED" },
    );

    // Brand / WASM / site assets (best-effort relative to CWD)
    let brand = PathBuf::from("assets/brand/logo-mark.svg");
    println!(
        "  brand mark:       {}",
        if brand.is_file() {
            "assets/brand/logo-mark.svg present"
        } else {
            "logo-mark.svg not in CWD (run from repo root or `dold logo`)"
        }
    );
    let wasm_pkg = PathBuf::from("site/public/wasm/doldskrift.js");
    println!(
        "  wasm site asset:  {}",
        if wasm_pkg.is_file() {
            "site/public/wasm/doldskrift.js present"
        } else {
            "missing (build bindings / sync site public)"
        }
    );
    let site_logo = PathBuf::from("site/public/logo-mark.svg");
    println!(
        "  site brand:       {}",
        if site_logo.is_file() {
            "site/public/logo-mark.svg present"
        } else {
            "missing"
        }
    );

    println!(
        "  unexpected:       radio · echo · ambient[--animate] · flicker · live · mesh[--html]"
    );
    println!(
        "                    postcard · duet · spectrogram · notarize · kaleidoscope · timeline"
    );
    println!(
        "                    seal · compare · handshake visual · museum · about · funding · brand"
    );
    println!("                    render --tty · scan --engine svg");
    let funding = PathBuf::from(".github/FUNDING.yml");
    println!(
        "  funding:          {}",
        if funding.is_file() {
            "FUNDING.yml + https://thanks.dev/u/gh/theworker02"
        } else {
            "missing .github/FUNDING.yml (see dold funding)"
        }
    );
    let surfaces_dir = PathBuf::from("site/public/surfaces");
    let fixture_count = if surfaces_dir.is_dir() {
        fs::read_dir(&surfaces_dir)
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .filter(|e| e.path().extension().map(|x| x == "svg").unwrap_or(false))
                    .count()
            })
            .unwrap_or(0)
    } else {
        0
    };
    println!(
        "  surface fixtures: {}",
        if fixture_count > 0 {
            format!(
                "{fixture_count} SVGs in site/public/surfaces (regen: npm run surfaces:fixtures)"
            )
        } else {
            "missing (npm run surfaces:fixtures)".into()
        }
    );
    println!("  vision report:    scan includes ASCII confidence histogram");
    // Open encode → validate → inspect --json smoke
    let open_ok = (|| -> Result<(), Box<dyn std::error::Error>> {
        let doc = DskDocument::encode_text("doctor-open", Mode::Encoded)?;
        let v = doc.validate();
        if !v.ok {
            return Err("open validate failed".into());
        }
        let _ = doc.inspect().to_json()?;
        let canon = doc.format_canonical()?;
        let _ = DskDocument::parse(&canon)?;
        Ok(())
    })();
    println!(
        "  open pipeline:    {}",
        match open_ok {
            Ok(()) => "encode → validate → inspect --json → fmt OK".into(),
            Err(e) => format!("FAILED ({e})"),
        }
    );
    println!("  cli extras:       guide · demo · convert · config · validate · fmt · completion");
    println!("  security stance:  Open/Neural ≠ encryption; Protected = AEAD+Gate (not shipping)");
    println!("  delight:          `dold` with no args → braille mark fingerprint");
    println!("OK — install looks healthy for local Open / Neural foundation work");
    Ok(())
}

/// Compare Rust constants against `spec/protocol-constants.json` when present.
fn doctor_constants_sync() -> String {
    use doldskrift::{
        ALPHABET_SIZE, DSK_PROJECT_MARK, FONT_VERSION, GLYPH_ENGINE_VERSION, PUA_BASE,
        PROTOCOL_VERSION, PROTOCOL_VERSION_PROTECTED,
    };
    let path = PathBuf::from("spec/protocol-constants.json");
    if !path.is_file() {
        return "spec/protocol-constants.json not in CWD (skipped)".into();
    }
    let Ok(raw) = fs::read_to_string(&path) else {
        return "unreadable spec/protocol-constants.json".into();
    };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) else {
        return "invalid JSON in protocol-constants.json".into();
    };
    let checks = [
        (
            "protocolVersion",
            v["protocolVersion"].as_u64(),
            Some(PROTOCOL_VERSION as u64),
        ),
        (
            "protocolVersionProtected",
            v["protocolVersionProtected"].as_u64(),
            Some(PROTOCOL_VERSION_PROTECTED as u64),
        ),
        ("fontVersion", v["fontVersion"].as_u64(), Some(FONT_VERSION as u64)),
        (
            "glyphEngineVersion",
            v["glyphEngineVersion"].as_u64(),
            Some(GLYPH_ENGINE_VERSION as u64),
        ),
        ("puaBase", v["puaBase"].as_u64(), Some(PUA_BASE as u64)),
        (
            "alphabetSize",
            v["alphabetSize"].as_u64(),
            Some(ALPHABET_SIZE as u64),
        ),
        (
            "projectMark",
            v["projectMark"].as_u64(),
            Some(DSK_PROJECT_MARK as u64),
        ),
    ];
    let mut mismatches = Vec::new();
    for (name, got, expect) in checks {
        if got != expect {
            mismatches.push(format!("{name}: json={got:?} rust={expect:?}"));
        }
    }
    if let Some(magic) = v["magic"].as_str() {
        if magic != "DSK1" {
            mismatches.push(format!("magic: json={magic} rust=DSK1"));
        }
    }
    // Optional JS mirror
    let js = PathBuf::from("packages/doldskrift-js/src/protocol-constants.json");
    if js.is_file() {
        if let Ok(js_raw) = fs::read_to_string(&js) {
            if let Ok(js_v) = serde_json::from_str::<serde_json::Value>(&js_raw) {
                if js_v["protocolVersion"] != v["protocolVersion"]
                    || js_v["fontVersion"] != v["fontVersion"]
                    || js_v["projectMark"] != v["projectMark"]
                {
                    mismatches.push("JS protocol-constants.json drifts from spec/".into());
                }
            }
        }
    }
    if mismatches.is_empty() {
        "OK (Rust ↔ spec/protocol-constants.json ↔ JS mirror)".into()
    } else {
        format!("DRIFT ({})", mismatches.join("; "))
    }
}

fn query_json_path(value: &serde_json::Value, path: &str) -> serde_json::Value {
    let path = path.trim().trim_start_matches('$');
    let parts: Vec<&str> = path
        .split(['.', '/'])
        .filter(|p| !p.is_empty())
        .collect();
    let mut cur = value;
    for part in parts {
        let key = part.trim_start_matches('[').trim_end_matches(']');
        if let Ok(idx) = key.parse::<usize>() {
            match cur {
                serde_json::Value::Array(a) => {
                    if let Some(v) = a.get(idx) {
                        cur = v;
                    } else {
                        return serde_json::Value::Null;
                    }
                }
                _ => return serde_json::Value::Null,
            }
        } else {
            match cur {
                serde_json::Value::Object(m) => {
                    if let Some(v) = m.get(key) {
                        cur = v;
                    } else {
                        return serde_json::Value::Null;
                    }
                }
                _ => return serde_json::Value::Null,
            }
        }
    }
    cur.clone()
}

fn brand_logo_svg(inner_glyph_svg: &str) -> String {
    // FONT_UNITS is 1000 — fit into a 128×128 branded tile.
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

fn cmd_specimen(
    input: Option<PathBuf>,
    mode: Mode,
    seed: Option<String>,
    output: PathBuf,
    density: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = read_input(input)?;
    let text = std::str::from_utf8(&bytes)?;
    let dens = density.parse::<Density>().map_err(|e| e.to_string())?;
    let _ = dens;
    let out = match mode {
        Mode::Visual => text.to_owned(),
        Mode::Encoded => doldskrift::encode(text)?,
        Mode::Session => {
            let seed = seed.unwrap_or_else(|| "specimen".into());
            Session::from_seed(seed.as_bytes(), "specimen")?.encode(text)?
        }
        Mode::Protected => {
            return Err(
                "specimen: Protected mode has no Open glyph specimen path (use encode --mode protected for stub container)"
                    .into(),
            );
        }
    };
    let catalog = generate_alphabet(doldskrift::FONT_VERSION, 1)?;
    let mark = generate_project_mark(1)?;
    let cell = 80;
    let width = (out.chars().count() as i32 + 1) * cell + 24;
    let mut svg = format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="140" viewBox="0 0 {width} 140">
  <rect width="100%" height="100%" fill="#121614"/>
  <text x="12" y="128" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="10">Doldskrift specimen · MGE/2 · {mode:?}</text>
"##
    );
    let mark_svg = glyph_to_svg(&mark)
        .replace(r#"<?xml version="1.0" encoding="UTF-8"?>"#, "")
        .replace("<svg", "<g")
        .replace("</svg>", "</g>");
    svg.push_str(&format!(
        "<g transform=\"translate(12,8) scale(0.08)\" stroke=\"#E8EFE9\">{mark_svg}</g>\n"
    ));
    for (i, ch) in out.chars().enumerate() {
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
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, svg)?;
    println!("Wrote specimen → {}", output.display());
    Ok(())
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

fn write_output(path: Option<PathBuf>, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    match path {
        None => {
            io::stdout().write_all(data)?;
            Ok(())
        }
        Some(p) => Ok(fs::write(p, data)?),
    }
}

fn cmd_encode(
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    mode: Mode,
    seed: Option<String>,
    session_id: Option<String>,
    raw: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = read_input(input)?;
    let text = std::str::from_utf8(&bytes)?;
    if raw {
        let encoded = match mode {
            Mode::Visual => text.to_owned(),
            Mode::Encoded => doldskrift::encode(text)?,
            Mode::Session => {
                let seed = seed.unwrap_or_default();
                let mut b = Session::builder().seed(seed.as_bytes());
                if let Some(id) = session_id {
                    b = b.session_id(id);
                }
                b.build()?.encode(text)?
            }
            Mode::Protected => {
                return Err(
                    "encode --raw: Protected has no Open PUA path; omit --raw for stub .dsk".into(),
                );
            }
        };
        write_output(output, encoded.as_bytes())?;
        return Ok(());
    }
    let doc = match mode {
        Mode::Session => {
            let seed = seed.unwrap_or_default();
            let mut b = Session::builder().seed(seed.as_bytes());
            if let Some(id) = session_id {
                b = b.session_id(id);
            }
            DskDocument::encode_session(text, &b.build()?)?
        }
        Mode::Protected => {
            eprintln!(
                "note: writing Protected *stub* container (placeholder bytes, not real AEAD)"
            );
            DskDocument::encode_protected_stub(text)?
        }
        other => DskDocument::encode_text(text, other)?,
    };
    write_output(output, &doc.to_bytes()?)?;
    Ok(())
}

fn cmd_open(
    input: Option<PathBuf>,
    capability: Option<String>,
    output: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = read_input(input)?;
    if !bytes.starts_with(doldskrift::MAGIC) {
        return Err("dold open expects a .dsk container".into());
    }
    let doc = DskDocument::parse(&bytes)?;
    let cap = capability.map(|id| GateCapability { id });
    match gate_open(&doc, cap.as_ref()) {
        Ok(text) => {
            write_output(output, text.as_bytes())?;
            Ok(())
        }
        Err(e) => {
            eprintln!("dold open: {e}");
            eprintln!(
                "Hint: Protected objects need Gate + capability; AEAD is not implemented yet."
            );
            eprintln!("      Use `dold inspect` for Readable: No metadata.");
            Err(e.into())
        }
    }
}

fn cmd_decode(
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    seed: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = read_input(input)?;
    let text = if bytes.starts_with(doldskrift::MAGIC) {
        let doc = DskDocument::parse(&bytes)?;
        if doc.is_protected() {
            eprintln!("dold decode: {}", doc.decode_text().unwrap_err());
            eprintln!("Hint: use `dold inspect` (Readable: No) or `dold open --capability …` (Gate stub).");
            return Err("Protected decode refused".into());
        }
        if doc.is_binary_payload() {
            write_output(output, doc.unpack_bytes())?;
            return Ok(());
        }
        doc.decode_text()?
    } else {
        let s = std::str::from_utf8(&bytes)?;
        if let Some(seed) = seed {
            Session::from_seed(seed.as_bytes(), "cli")?.decode(s)?
        } else {
            doldskrift::decode(s)?
        }
    };
    let to_stdout = output.is_none();
    write_output(output, text.as_bytes())?;
    if to_stdout && !text.ends_with('\n') {
        println!();
    }
    Ok(())
}

fn cmd_render(
    input: Option<PathBuf>,
    mode: Mode,
    seed: Option<String>,
    output: Option<PathBuf>,
    density: &str,
    debug: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = read_input(input)?;
    let text = std::str::from_utf8(&bytes)?;
    let dens = density.parse::<Density>().map_err(|e| e.to_string())?;
    let _ = dens;
    let _ = FontFamily::Text;
    let out = match mode {
        Mode::Visual => text.to_owned(),
        Mode::Encoded => doldskrift::encode(text)?,
        Mode::Session => {
            let seed = seed.unwrap_or_else(|| "specimen".into());
            Session::from_seed(seed.as_bytes(), "render")?.encode(text)?
        }
        Mode::Protected => {
            return Err("render: Protected mode has no Open render path".into());
        }
    };

    if debug {
        let catalog = generate_alphabet(doldskrift::FONT_VERSION, 1)?;
        let mut dbg = String::from("<!-- dold render --debug -->\n");
        dbg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\">\n",
            out.chars().count() as i32 * 80,
            FONT_UNITS / 8
        ));
        for (i, ch) in out.chars().enumerate() {
            let cp = ch as u32;
            let g = catalog
                .iter()
                .find(|g| g.codepoint == cp)
                .cloned()
                .unwrap_or(generate_glyph(cp, doldskrift::FONT_VERSION, 1)?);
            let x = i as i32 * 80;
            dbg.push_str(&format!(
                "<g transform=\"translate({x},0) scale(0.08)\">{}</g>\n",
                glyph_to_svg(&g)
                    .replace(r#"<?xml version="1.0" encoding="UTF-8"?>"#, "")
                    .replace("<svg", "<g")
                    .replace("</svg>", "</g>")
            ));
            dbg.push_str(&format!(
                "<text x=\"{}\" y=\"140\" font-size=\"8\" font-family=\"monospace\">U+{:04X} {}</text>\n",
                x,
                cp,
                g.fingerprint()
            ));
            dbg.push_str(&format!(
                "<rect x=\"{x}\" y=\"0\" width=\"72\" height=\"100\" fill=\"none\" stroke=\"#0F6B5C\" stroke-width=\"0.5\" opacity=\"0.4\"/>\n"
            ));
        }
        dbg.push_str("</svg>\n");
        // Also emit a feature map beside debug SVG when writing a file ending in .svg.json
        let map = render_glyph_png_feature_map(text, &catalog)?;
        if let Some(path) = &output {
            write_output(Some(path.clone()), dbg.as_bytes())?;
            let map_path = path.with_extension("features.json");
            fs::write(map_path, map)?;
        } else {
            print!("{dbg}");
        }
        return Ok(());
    }

    write_output(output, out.as_bytes())?;
    Ok(())
}
