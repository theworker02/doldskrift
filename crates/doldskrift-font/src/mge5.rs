//! MGE/5 Neural scaffold — latent-aware glyph profile stubs.
//!
//! Honest status: structure only. Does not produce a trained neural alphabet.
//! Profile `MGE5-NEURAL` aims to minimize Latin/digit/punctuation/word-spacing resemblance.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use doldskrift::neural::{LatentGraph, LatentNode, LatentNodeKind};

/// MGE/5 engine label.
pub const MGE5_ENGINE: &str = "MGE/5";

/// Visual profile for neural rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Mge5Profile {
    /// Neural-optimized machine grammar (default for DSK/3 Neural).
    #[default]
    Neural,
    /// Reserved for Protected visual objects (sibling track).
    Protected,
}

impl Mge5Profile {
    /// Wire label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Neural => "MGE5-NEURAL",
            Self::Protected => "MGE5-PROTECTED",
        }
    }
}

/// Compound glyph group stub (multiple latent nodes → one visual unit).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompoundGlyph {
    /// Group id.
    pub id: String,
    /// Member latent node ids.
    pub members: Vec<String>,
    /// Deterministic seed for future geometry.
    pub seed: u64,
}

/// Contextual variant selection stub.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextualVariant {
    /// Latent node id.
    pub node_id: String,
    /// Variant index (0 = base).
    pub variant_index: u32,
    /// Reason code (`repetition_suppress`, `neighbor_context`, …).
    pub reason: String,
}

/// Forge 3 neural-aware objective stub (scores are placeholders / NaN until measured).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Forge3NeuralObjectives {
    /// Machine separability of latent-derived marks (TBD).
    pub machine_separability: Option<f64>,
    /// Latin/digit familiarity proxy (lower is better for Neural; TBD).
    pub latin_familiarity: Option<f64>,
    /// Repetition exposure under contextual variants (TBD).
    pub repetition_exposure: Option<f64>,
    /// Reader compatibility retention across epochs (TBD).
    pub epoch_retention: Option<f64>,
}

/// Result of an MGE/5 plan (no real path outlines yet).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mge5Plan {
    /// Profile used.
    pub profile: String,
    /// Engine version label.
    pub engine: String,
    /// Compound groups derived from the latent graph.
    pub compounds: Vec<CompoundGlyph>,
    /// Contextual variants.
    pub variants: Vec<ContextualVariant>,
    /// Forge 3 objectives (unmeasured).
    pub forge3: Forge3NeuralObjectives,
    /// Explicit TODOs for operators.
    pub todos: Vec<String>,
}

/// Plan MGE/5 rendering from a latent graph (deterministic stubs).
pub fn plan_from_latent(graph: &LatentGraph, seed: u64) -> Mge5Plan {
    let compounds = compound_groups(graph, seed);
    let variants = contextual_variants(graph, seed);
    Mge5Plan {
        profile: Mge5Profile::Neural.label().into(),
        engine: MGE5_ENGINE.into(),
        compounds,
        variants,
        forge3: Forge3NeuralObjectives::default(),
        todos: vec![
            "Expand path geometry beyond deterministic SVG carrier marks".into(),
            "Minimize Latin/digit/punctuation/word-spacing resemblance under MGE5-NEURAL".into(),
            "Wire repetition suppression with measured Forge 3 scores".into(),
        ],
    }
}

/// Deterministic SVG carrier from an [`Mge5Plan`] + latent graph.
///
/// Not a trained alphabet and not camera-ready DVE/2. Provides a stable visual
/// artifact for demos / fixtures so the Neural path is more than carrier JSON.
pub fn render_plan_svg(graph: &LatentGraph, plan: &Mge5Plan, seed: u64) -> String {
    let width = 640u32;
    let height = 240u32;
    let mut marks = String::new();
    let cells: Vec<&LatentNode> = graph
        .nodes
        .iter()
        .filter(|n| {
            matches!(
                n.kind,
                LatentNodeKind::Atom
                    | LatentNodeKind::Sequence
                    | LatentNodeKind::Record
                    | LatentNodeKind::Field
            )
        })
        .collect();
    let n = cells.len().max(1);
    let cols = ((n as f64).sqrt().ceil() as usize).max(1);
    let cell_w = (width as f64 - 40.0) / cols as f64;
    let rows = n.div_ceil(cols);
    let cell_h = (height as f64 - 48.0) / rows.max(1) as f64;

    for (i, node) in cells.iter().enumerate() {
        let col = i % cols;
        let row = i / cols;
        let cx = 20.0 + cell_w * (col as f64 + 0.5);
        let cy = 28.0 + cell_h * (row as f64 + 0.5);
        let h = mix64(seed, hash_str(&node.id));
        let variant = plan
            .variants
            .iter()
            .find(|v| v.node_id == node.id)
            .map(|v| v.variant_index)
            .unwrap_or(0);
        let r = 10.0 + ((h % 17) as f64) + f64::from(variant);
        let rot = (h % 360) as f64;
        let sides = 3 + (h % 5) as i32;
        let stroke = if variant > 0 { "#7ec8b8" } else { "#d8efe6" };
        marks.push_str(&polygon_mark(cx, cy, r, sides, rot, stroke, h));
        // Small integrity tick from content hash nibble
        let tick = (h >> 8) % 8;
        marks.push_str(&format!(
            r##"<circle cx="{:.1}" cy="{:.1}" r="2.2" fill="#0F6B5C" opacity="0.85"/>"##,
            cx + r * 0.55,
            cy - r * 0.55 - tick as f64
        ));
    }

    // Compound group outlines
    for (gi, cg) in plan.compounds.iter().enumerate() {
        let member_idx: Vec<usize> = cg
            .members
            .iter()
            .filter_map(|id| cells.iter().position(|n| n.id == *id))
            .collect();
        if member_idx.len() < 2 {
            continue;
        }
        let mut path = String::from("M");
        for (k, idx) in member_idx.iter().enumerate() {
            let col = idx % cols;
            let row = idx / cols;
            let cx = 20.0 + cell_w * (col as f64 + 0.5);
            let cy = 28.0 + cell_h * (row as f64 + 0.5);
            if k == 0 {
                path.push_str(&format!(" {:.1},{:.1}", cx, cy));
            } else {
                path.push_str(&format!(" L {:.1},{:.1}", cx, cy));
            }
        }
        path.push_str(" Z");
        let op = 0.12 + ((gi as f64) * 0.03).min(0.2);
        marks.push_str(&format!(
            r##"<path d="{path}" fill="none" stroke="#0F6B5C" stroke-width="1.5" opacity="{op:.2}" stroke-dasharray="4 3"/>"##
        ));
    }

    let epoch = graph.meta.epoch.label();
    let content = graph.meta.content_id.as_deref().unwrap_or("-");
    format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" role="img" aria-label="MGE/5 Neural carrier (deterministic stub)">
  <rect width="100%" height="100%" fill="#101412"/>
  <text x="12" y="16" fill="#7aa897" font-family="IBM Plex Mono, monospace" font-size="10">{engine} · {profile} · seed={seed}</text>
  <text x="12" y="{footer_y}" fill="#5a7a70" font-family="IBM Plex Mono, monospace" font-size="9">epoch={epoch} · content_id={content} · not a trained alphabet · Neural ≠ encryption</text>
  {marks}
</svg>
"##,
        engine = plan.engine,
        profile = plan.profile,
        footer_y = height - 8,
    )
}

/// Convenience: plan + SVG in one call.
pub fn render_latent_svg(graph: &LatentGraph, seed: u64) -> (Mge5Plan, String) {
    let plan = plan_from_latent(graph, seed);
    let svg = render_plan_svg(graph, &plan, seed);
    (plan, svg)
}

fn polygon_mark(
    cx: f64,
    cy: f64,
    r: f64,
    sides: i32,
    rot_deg: f64,
    stroke: &str,
    h: u64,
) -> String {
    let mut d = String::new();
    let rot = rot_deg.to_radians();
    for i in 0..sides {
        let a = rot + std::f64::consts::TAU * (i as f64) / (sides as f64);
        let x = cx + r * a.cos();
        let y = cy + r * a.sin();
        if i == 0 {
            d.push_str(&format!("M {:.2},{:.2}", x, y));
        } else {
            d.push_str(&format!(" L {:.2},{:.2}", x, y));
        }
    }
    d.push_str(" Z");
    // Inner chord for machine texture
    let inner = r * (0.35 + ((h % 40) as f64) / 100.0);
    let mut d2 = String::new();
    for i in 0..sides {
        let a = rot
            + std::f64::consts::PI / (sides as f64)
            + std::f64::consts::TAU * (i as f64) / (sides as f64);
        let x = cx + inner * a.cos();
        let y = cy + inner * a.sin();
        if i == 0 {
            d2.push_str(&format!("M {:.2},{:.2}", x, y));
        } else {
            d2.push_str(&format!(" L {:.2},{:.2}", x, y));
        }
    }
    d2.push_str(" Z");
    format!(
        r##"<path d="{d}" fill="none" stroke="{stroke}" stroke-width="2" stroke-linejoin="miter"/><path d="{d2}" fill="none" stroke="{stroke}" stroke-width="1" opacity="0.55"/>"##
    )
}

fn hash_str(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x1000_0000_01b3);
    }
    h
}

fn mix64(a: u64, b: u64) -> u64 {
    let mut x = a ^ b.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

/// Deterministic compound grouping: group consecutive atoms under the same parent.
fn compound_groups(graph: &LatentGraph, seed: u64) -> Vec<CompoundGlyph> {
    let mut out = Vec::new();
    // Group all atoms under a single compound when ≤ 8 atoms (stub heuristic).
    let atoms: Vec<&LatentNode> = graph
        .nodes
        .iter()
        .filter(|n| matches!(n.kind, LatentNodeKind::Atom))
        .collect();
    if atoms.is_empty() {
        return out;
    }
    if atoms.len() <= 8 {
        out.push(CompoundGlyph {
            id: format!("cg-{}", seed ^ (atoms.len() as u64)),
            members: atoms.iter().map(|n| n.id.clone()).collect(),
            seed,
        });
    } else {
        for (i, chunk) in atoms.chunks(4).enumerate() {
            out.push(CompoundGlyph {
                id: format!("cg-{}-{}", seed, i),
                members: chunk.iter().map(|n| n.id.clone()).collect(),
                seed: seed.wrapping_add(i as u64),
            });
        }
    }
    out
}

/// Repetition suppression hook: bump variant index when identical atom values repeat.
fn contextual_variants(graph: &LatentGraph, _seed: u64) -> Vec<ContextualVariant> {
    let mut seen: BTreeMap<String, u32> = BTreeMap::new();
    let mut out = Vec::new();
    for n in &graph.nodes {
        if !matches!(n.kind, LatentNodeKind::Atom) {
            continue;
        }
        let key = n.value.as_ref().map(|v| v.to_string()).unwrap_or_default();
        let count = seen.entry(key).or_insert(0);
        if *count > 0 {
            out.push(ContextualVariant {
                node_id: n.id.clone(),
                variant_index: *count,
                reason: "repetition_suppress".into(),
            });
        }
        *count += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use doldskrift::neural::compose_deterministic;
    use doldskrift::semantic::Value;

    #[test]
    fn mge5_plan_stub() {
        let g = compose_deterministic(&Value::String("x".into())).unwrap();
        let plan = plan_from_latent(&g, 1);
        assert_eq!(plan.profile, "MGE5-NEURAL");
        assert_eq!(plan.engine, MGE5_ENGINE);
        assert!(!plan.todos.is_empty());
    }

    #[test]
    fn mge5_svg_carrier_deterministic() {
        let g = compose_deterministic(&Value::String("secret-payload-xyz".into())).unwrap();
        let (plan, svg_a) = render_latent_svg(&g, 7);
        let svg_b = render_plan_svg(&g, &plan, 7);
        assert_eq!(svg_a, svg_b);
        assert!(svg_a.contains("MGE/5"));
        assert!(svg_a.contains("<svg"));
        assert!(
            !svg_a.contains("secret-payload-xyz"),
            "SVG must not embed plaintext atom value"
        );
    }
}
