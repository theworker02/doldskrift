//! Doldskrift Forge — multi-objective alphabet scoring (experimental).

use crate::mge4::{familiarity_score, repetition_exposure};
use crate::{generate_alphabet_ex, machine_separability, Density, FontFamily, GenerateOptions};
use doldskrift::{Result, FONT_VERSION};
use serde::{Deserialize, Serialize};

/// Alphabet lifecycle stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlphabetLifecycle {
    /// Research only.
    Experimental,
    /// Under evaluation.
    Candidate,
    /// Frozen mapping — must not silently change.
    Stable,
    /// Superseded but still decodable.
    Legacy,
}

/// Forge run configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeConfig {
    /// RNG / generator seed.
    pub seed: u64,
    /// Generations (scoring iterations; full evolution is future work).
    pub generations: u32,
    /// Symbol count (currently capped at 256 alphabet).
    pub symbols: usize,
    /// Profile label.
    pub profile: String,
}

impl Default for ForgeConfig {
    fn default() -> Self {
        Self {
            seed: 1,
            generations: 1,
            symbols: 256,
            profile: "standard".into(),
        }
    }
}

/// Measured scorecard fields (real measurements only).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeScores {
    /// Mean machine separability across sample pairs.
    pub machine_separation: f64,
    /// Mean familiarity proxy (lower better for machine profiles).
    pub visual_familiarity: f64,
    /// Repetition exposure along identity sequence sample.
    pub repetition_exposure: f64,
    /// Mean primitive complexity.
    pub complexity: f64,
}

/// Forge report for one alphabet candidate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeReport {
    /// Alphabet id.
    pub alphabet_id: String,
    /// Lifecycle.
    pub lifecycle: AlphabetLifecycle,
    /// Scores.
    pub scores: ForgeScores,
    /// Seed used.
    pub seed: u64,
    /// Profile.
    pub profile: String,
}

/// Run a single-generation Forge evaluation (measurable, reproducible).
pub fn run_forge_generation(cfg: &ForgeConfig) -> Result<ForgeReport> {
    let opts = GenerateOptions {
        font_version: FONT_VERSION,
        seed: cfg.seed,
        density: Density::Normal,
        family: FontFamily::Text,
    };
    let glyphs = generate_alphabet_ex(&opts)?;
    let n = cfg.symbols.min(glyphs.len());
    let sample = &glyphs[..n];

    let mut sep = 0.0;
    let mut fam = 0.0;
    let mut complexity = 0.0;
    let take_n = n.min(64);
    for g in sample.iter().take(take_n) {
        sep += machine_separability(g);
        fam += familiarity_score(g);
        complexity += g.primitives.len() as f64;
    }
    let take = take_n as f64;
    sep /= take;
    fam /= take;
    complexity /= take;

    let rep = repetition_exposure(&sample[..sample.len().min(48)]);

    let alphabet_id = format!(
        "MGE2-{}-{:04X}",
        cfg.profile.to_ascii_uppercase(),
        (cfg.seed as u16)
    );

    Ok(ForgeReport {
        alphabet_id,
        lifecycle: AlphabetLifecycle::Experimental,
        scores: ForgeScores {
            machine_separation: sep,
            visual_familiarity: fam,
            repetition_exposure: rep,
            complexity,
        },
        seed: cfg.seed,
        profile: cfg.profile.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forge_runs() {
        let report = run_forge_generation(&ForgeConfig {
            seed: 3,
            generations: 1,
            symbols: 256,
            profile: "vision".into(),
        })
        .unwrap();
        assert!(report.alphabet_id.contains("VISION"));
        assert!(report.scores.machine_separation >= 0.0);
    }
}
