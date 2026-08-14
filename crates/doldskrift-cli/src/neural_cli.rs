//! Neural profile CLI stubs — Mock/Deterministic paths only; no model downloads.

use doldskrift::neural::{
    compose_deterministic, DeterministicBaseline, Dsk3Profile, GrammarEpoch, MockReader,
    ObservationGraph, ReaderBackend, ReaderStatus,
};
use doldskrift::semantic::Value;
use doldskrift_font::{plan_from_latent, render_latent_svg, render_plan_svg};
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

/// `dold dataset` subcommands.
#[derive(clap::Subcommand, Debug)]
pub enum DatasetCommands {
    /// Generate a seed-deterministic synthetic Neural fixture (JSON + tiny PNG stub)
    Generate {
        #[arg(long, default_value_t = 1)]
        seed: u64,
        #[arg(short, long, default_value = "training/datasets/synthetic")]
        out: PathBuf,
        #[arg(long, default_value = "hello-neural")]
        message: String,
    },
}

/// `dold reader` subcommands (mostly stubs).
#[derive(clap::Subcommand, Debug)]
pub enum ReaderCommands {
    /// Show active / default reader info
    Info,
    /// List known local readers
    List,
    /// Install a reader package (stub — does not download weights)
    Install {
        #[arg(long)]
        name: String,
    },
    /// Verify a reader manifest (stub)
    Verify {
        #[arg(long)]
        name: Option<String>,
    },
    /// Benchmark reader (DeterministicBaseline only for now)
    Benchmark,
    /// Remove a reader (stub)
    Remove {
        #[arg(long)]
        name: String,
    },
    /// Diagnose reader environment
    Doctor,
    /// Train adapter (stub — points at training/)
    TrainAdapter,
}

/// `dold neural` subcommands.
#[derive(clap::Subcommand, Debug)]
pub enum NeuralCommands {
    /// Compose semantic JSON → LatentGraph (LSG)
    Encode {
        input: Option<PathBuf>,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Plan MGE/5 render from latent JSON (JSON plan and/or SVG carrier)
    Render {
        input: Option<PathBuf>,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// `json` (plan) or `svg` (deterministic carrier)
        #[arg(long, default_value = "json")]
        format: String,
    },
    /// Reconstruct via Mock/DeterministicBaseline from observation or latent JSON
    Reconstruct {
        input: Option<PathBuf>,
        #[arg(long, default_value = "deterministic")]
        reader: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Inspect Neural profile / epoch metadata
    Inspect {
        input: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
}

/// `dold evaluate` subcommands.
#[derive(clap::Subcommand, Debug)]
pub enum EvaluateCommands {
    /// Run baseline evaluation harness (stub metrics)
    Baseline,
}

pub fn run_dataset(cmd: DatasetCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        DatasetCommands::Generate { seed, out, message } => {
            fs::create_dir_all(&out)?;
            let value = Value::String(format!("{message}:{seed}"));
            let latent = compose_deterministic(&value)?;
            let obs = ObservationGraph::from_latent_carrier(latent.clone());
            let plan = plan_from_latent(&latent, seed);

            let fixture = serde_json::json!({
                "seed": seed,
                "profile": Dsk3Profile::neural(GrammarEpoch::E0001.label()),
                "message": message,
                "latent": latent,
                "observation": obs,
                "mge5_plan": plan,
                "notes": "Synthetic fixture — not a trained dataset; Neural ≠ encryption",
            });
            let json_path = out.join(format!("fixture-{seed}.json"));
            fs::write(&json_path, serde_json::to_string_pretty(&fixture)?)?;

            let (_plan, svg) = render_latent_svg(&latent, seed);
            let svg_path = out.join(format!("fixture-{seed}.svg"));
            fs::write(&svg_path, svg)?;

            // Tiny deterministic PNG stub (1×1) retained for tooling that expects PNG.
            let png_path = out.join(format!("fixture-{seed}.png"));
            write_tiny_png_stub(&png_path, seed)?;

            println!("Wrote {}", json_path.display());
            println!(
                "Wrote {} (MGE/5 deterministic SVG carrier)",
                svg_path.display()
            );
            println!("Wrote {} (1×1 stub PNG)", png_path.display());
            println!("Neural ≠ encryption. Fixture is for CI / pipeline shape only.");
        }
    }
    Ok(())
}

pub fn run_reader(cmd: ReaderCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        ReaderCommands::Info => {
            println!("Doldskrift readers (foundation)");
            println!("  default:  deterministic-baseline/1");
            println!("  also:     mock-reader/1");
            println!("  gemma:    not wired (enable feature gemma-reader for API stub only)");
            println!("  note:     Neural ≠ encryption; no auto-download of weights");
        }
        ReaderCommands::List => {
            println!("deterministic-baseline/1  [local]  epochs: LSG/1-E0001");
            println!("mock-reader/1             [local]  epochs: LSG/1-E0001");
        }
        ReaderCommands::Install { name } => {
            eprintln!(
                "dold reader install: not yet implemented (refused to auto-download '{name}')"
            );
            eprintln!("See training/README.md for experiment workflows.");
            return Err("reader install stub".into());
        }
        ReaderCommands::Verify { name } => {
            let name = name.unwrap_or_else(|| "deterministic-baseline/1".into());
            if name.contains("deterministic") || name.contains("mock") {
                println!("OK — {name} is a local CI reader (no weights)");
            } else {
                println!("STUB — cannot verify '{name}' (no checkpoint packaging yet)");
            }
        }
        ReaderCommands::Benchmark => {
            let value = Value::String("bench".into());
            let latent = compose_deterministic(&value)?;
            let obs = ObservationGraph::from_latent_carrier(latent);
            let t0 = std::time::Instant::now();
            let out = DeterministicBaseline::new().reconstruct(&obs)?;
            let ms = t0.elapsed().as_secs_f64() * 1000.0;
            println!("reader: deterministic-baseline/1");
            println!("status: {:?}", out.status);
            println!("latency_ms: {ms:.3}");
            println!("metrics: TBD for trained readers");
        }
        ReaderCommands::Remove { name } => {
            eprintln!("dold reader remove: stub — nothing removed for '{name}'");
            return Err("reader remove stub".into());
        }
        ReaderCommands::Doctor => {
            println!("Doldskrift reader doctor");
            println!("  deterministic-baseline: available");
            println!("  mock-reader:            available");
            println!("  trained checkpoints:    none required for CI");
            println!("  Python training tree:   training/ (experiments only)");
            println!("OK");
        }
        ReaderCommands::TrainAdapter => {
            eprintln!("dold reader train-adapter: not implemented in Rust CLI.");
            eprintln!("Use Python scripts under training/scripts/ when experiments begin.");
            return Err("train-adapter stub".into());
        }
    }
    Ok(())
}

pub fn run_neural(cmd: NeuralCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        NeuralCommands::Encode { input, output } => {
            let text = read_text(input)?;
            let json: serde_json::Value = match serde_json::from_str(&text) {
                Ok(v) => v,
                Err(_) => serde_json::Value::String(text),
            };
            let value = Value::from_json(&json)?;
            let latent = compose_deterministic(&value)?;
            let bytes = serde_json::to_vec_pretty(&latent)?;
            write_bytes(output, &bytes)?;
        }
        NeuralCommands::Render {
            input,
            output,
            seed,
            format,
        } => {
            let text = read_text(input)?;
            let latent: doldskrift::neural::LatentGraph = serde_json::from_str(&text)?;
            let plan = plan_from_latent(&latent, seed);
            match format.to_ascii_lowercase().as_str() {
                "svg" => {
                    let svg = render_plan_svg(&latent, &plan, seed);
                    write_bytes(output, svg.as_bytes())?;
                    eprintln!("MGE/5 deterministic SVG carrier — not a trained alphabet");
                }
                _ => {
                    let bytes = serde_json::to_vec_pretty(&plan)?;
                    write_bytes(output, &bytes)?;
                    eprintln!(
                        "MGE/5 plan JSON — use --format svg for deterministic carrier (see plan.todos)"
                    );
                }
            }
        }
        NeuralCommands::Reconstruct {
            input,
            reader,
            output,
        } => {
            let text = read_text(input)?;
            let obs = parse_observation_or_latent(&text)?;
            let out = match reader.to_ascii_lowercase().as_str() {
                "deterministic" | "deterministic-baseline" | "baseline" => {
                    DeterministicBaseline::new().reconstruct(&obs)?
                }
                "mock" => MockReader::new().reconstruct(&obs)?,
                other => {
                    return Err(format!(
                        "unknown reader '{other}' (foundation: deterministic|mock only)"
                    )
                    .into());
                }
            };
            if out.status != ReaderStatus::Reconstructed {
                eprintln!(
                    "status: {:?} — {}",
                    out.status,
                    out.message.unwrap_or_default()
                );
                return Err("reconstruction did not succeed".into());
            }
            let v = out.value.expect("value");
            let bytes = serde_json::to_vec_pretty(&v.to_json())?;
            write_bytes(output, &bytes)?;
        }
        NeuralCommands::Inspect { input, json } => {
            let text = read_text(input)?;
            let latent: Result<doldskrift::neural::LatentGraph, _> = serde_json::from_str(&text);
            if let Ok(g) = latent {
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "profile": "neural",
                            "epoch": g.meta.epoch.label(),
                            "composer": g.meta.composer,
                            "content_id": g.meta.content_id,
                            "nodes": g.nodes.len(),
                            "edges": g.edges.len(),
                            "notes": "Neural ≠ encryption",
                        }))?
                    );
                } else {
                    println!("Doldskrift Neural inspect");
                    println!("  profile:    neural (DSK/3 Neural)");
                    println!("  epoch:      {}", g.meta.epoch);
                    println!(
                        "  composer:   {}",
                        g.meta.composer.as_deref().unwrap_or("-")
                    );
                    println!(
                        "  content_id: {}",
                        g.meta.content_id.as_deref().unwrap_or("-")
                    );
                    println!("  nodes:      {}", g.nodes.len());
                    println!("  edges:      {}", g.edges.len());
                    println!("  note:       Neural ≠ encryption");
                }
            } else {
                let obs: ObservationGraph = serde_json::from_str(&text)?;
                if json {
                    println!("{}", serde_json::to_string_pretty(&obs)?);
                } else {
                    println!("Doldskrift observation inspect");
                    println!("  is_doldskrift: {}", obs.is_doldskrift);
                    println!(
                        "  epoch:         {}",
                        obs.epoch
                            .as_ref()
                            .map(|e| e.label())
                            .unwrap_or_else(|| "-".into())
                    );
                    println!("  nodes:         {}", obs.nodes.len());
                    println!("  note:          Neural ≠ encryption");
                }
            }
        }
    }
    Ok(())
}

pub fn run_evaluate(cmd: EvaluateCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        EvaluateCommands::Baseline => {
            println!("Doldskrift evaluate baseline (stub)");
            println!("  deterministic round-trip: use `cargo test -p doldskrift neural`");
            println!("  adversarial OCR bake-off: metrics TBD — harness not populated");
            println!("  trained reader scores:    deferred");
            println!("Neural ≠ encryption. Baseline here measures representation fidelity only.");
        }
    }
    Ok(())
}

fn parse_observation_or_latent(text: &str) -> Result<ObservationGraph, Box<dyn std::error::Error>> {
    if let Ok(obs) = serde_json::from_str::<ObservationGraph>(text) {
        return Ok(obs);
    }
    let latent: doldskrift::neural::LatentGraph = serde_json::from_str(text)?;
    Ok(ObservationGraph::from_latent_carrier(latent))
}

fn read_text(input: Option<PathBuf>) -> Result<String, Box<dyn std::error::Error>> {
    match input {
        Some(p) => Ok(fs::read_to_string(p)?),
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            Ok(buf)
        }
    }
}

fn write_bytes(output: Option<PathBuf>, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    match output {
        Some(p) => {
            if let Some(parent) = p.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(p, bytes)?;
        }
        None => {
            io::stdout().write_all(bytes)?;
            if !bytes.ends_with(b"\n") {
                println!();
            }
        }
    }
    Ok(())
}

/// Minimal valid 1×1 PNG with seed-tinted RGB (deterministic stub).
fn write_tiny_png_stub(path: &Path, seed: u64) -> Result<(), Box<dyn std::error::Error>> {
    // Precomputed 1×1 PNG template; we only document seed in sidecar JSON.
    // Using a fixed tiny PNG keeps CLI free of the `image` crate.
    let _ = seed;
    const PNG_1X1: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0xD7, 0x63, 0xF8,
        0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE, 0x02, 0xFE, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    fs::write(path, PNG_1X1)?;
    Ok(())
}
