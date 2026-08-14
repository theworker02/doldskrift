//! Latent Structure Grammar (LSG) types.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::epoch::GrammarEpoch;

/// A node in a latent structure graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatentNode {
    /// Stable id within the graph.
    pub id: String,
    /// Structural kind.
    pub kind: LatentNodeKind,
    /// Optional typed payload (JSON-compatible).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    /// Free-form attributes for render/reader hints.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attrs: BTreeMap<String, String>,
}

/// Structural role of a latent node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatentNodeKind {
    /// Root of the composed graph.
    Root,
    /// Scalar leaf.
    Atom,
    /// Ordered sequence.
    Sequence,
    /// Keyed map / record.
    Record,
    /// Field name within a record.
    Field,
    /// Explicit null.
    Null,
    /// Opaque / reserved for future LSG extensions.
    Opaque,
}

/// Directed edge between latent nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatentEdge {
    /// Source node id.
    pub from: String,
    /// Target node id.
    pub to: String,
    /// Relation label (`child`, `key`, `item`, …).
    pub rel: String,
}

/// Graph-level metadata (epoch, profile, content hash hints).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatentMetadata {
    /// Grammar epoch (e.g. `LSG/1-E0001`).
    pub epoch: GrammarEpoch,
    /// Optional content id (SHA-256 hex of canonical semantic bytes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    /// Composer id that produced this graph.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composer: Option<String>,
}

/// Latent Structure Graph — intermediate between semantics and visual form.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatentGraph {
    /// Metadata including grammar epoch.
    pub meta: LatentMetadata,
    /// Nodes (order is not semantic; use edges).
    pub nodes: Vec<LatentNode>,
    /// Directed edges.
    pub edges: Vec<LatentEdge>,
}

impl LatentGraph {
    /// Find a node by id.
    pub fn node(&self, id: &str) -> Option<&LatentNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Canonical JSON bytes for hashing / fixtures (pretty-independent).
    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }
}
