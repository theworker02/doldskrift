//! DVE/2 observation graphs — perception only, no semantic reconstruction.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::epoch::GrammarEpoch;
use super::lsg::LatentGraph;

/// Perceived node after vision (geometry / topology hints).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservationNode {
    /// Local observation id.
    pub id: String,
    /// Perceived kind (may be coarse).
    pub kind: ObservationNodeKind,
    /// Optional fingerprint / glyph code hint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Bounding box stub `[x, y, w, h]` in normalized coords.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bbox: Option<[f64; 4]>,
    /// Extra perception attrs.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attrs: BTreeMap<String, String>,
}

/// Coarse observation taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationNodeKind {
    /// Detected mark / glyph-like region.
    Mark,
    /// Group / compound.
    Group,
    /// Relation anchor.
    Anchor,
    /// Unknown / noise.
    Unknown,
}

/// Perceived relationship between observations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservationEdge {
    /// Source observation id.
    pub from: String,
    /// Target observation id.
    pub to: String,
    /// Relation (`adjacent`, `contains`, `aligns`, …).
    pub rel: String,
    /// Confidence in \[0, 1\].
    #[serde(default = "one")]
    pub confidence: f64,
}

fn one() -> f64 {
    1.0
}

/// Observation graph produced by DVE/2 (perception only).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservationGraph {
    /// Declared or detected grammar epoch (if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epoch: Option<GrammarEpoch>,
    /// True when the input was recognized as a Doldskrift neural specimen.
    pub is_doldskrift: bool,
    /// Nodes.
    pub nodes: Vec<ObservationNode>,
    /// Edges.
    pub edges: Vec<ObservationEdge>,
    /// Optional carrier payload for DeterministicBaseline / Mock (not a camera path).
    ///
    /// Foundation: embeds the latent graph JSON so CI can round-trip without pixels.
    /// Real DVE/2 must not rely on this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latent_carrier: Option<LatentGraph>,
    /// Notes for operators (honesty).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl ObservationGraph {
    /// Build a synthetic observation that carries a latent graph for deterministic CI.
    pub fn from_latent_carrier(graph: LatentGraph) -> Self {
        let epoch = graph.meta.epoch;
        Self {
            epoch: Some(epoch),
            is_doldskrift: true,
            nodes: vec![ObservationNode {
                id: "obs0".into(),
                kind: ObservationNodeKind::Group,
                fingerprint: Some("DSK-NEURAL-CARRIER".into()),
                bbox: Some([0.0, 0.0, 1.0, 1.0]),
                attrs: BTreeMap::new(),
            }],
            edges: vec![],
            latent_carrier: Some(graph),
            notes: Some("Synthetic carrier observation — not a camera reconstruction".into()),
        }
    }

    /// Empty non-Doldskrift observation (random image stub).
    pub fn not_doldskrift() -> Self {
        Self {
            epoch: None,
            is_doldskrift: false,
            nodes: vec![],
            edges: vec![],
            latent_carrier: None,
            notes: Some("No Doldskrift structure detected".into()),
        }
    }
}
