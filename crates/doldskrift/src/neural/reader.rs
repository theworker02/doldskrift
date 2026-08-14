//! Reader backends — reconstruction from observation graphs.
//!
//! Wrong or unknown grammar fails cleanly. Never fabricate convincing false plaintext.

use crate::semantic::Value;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::epoch::{GrammarEpoch, ReaderCompatibility};
use super::lsg::{LatentGraph, LatentNodeKind};
use super::observation::ObservationGraph;

/// Stable reader identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReaderId(pub String);

impl ReaderId {
    /// Construct from string.
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl std::fmt::Display for ReaderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Reconstruction outcome status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReaderStatus {
    /// Semantics reconstructed successfully.
    Reconstructed,
    /// Grammar epoch / reader mismatch.
    Incompatible,
    /// Detected Doldskrift but confidence too low (reserved).
    LowConfidence,
    /// Grammar epoch unknown to this reader.
    UnknownGrammar,
    /// Reconstruction attempted but failed structurally.
    ReconstructionFailed,
    /// Input not recognized as Doldskrift.
    NotDoldskrift,
}

/// Output of a reader attempt.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReaderOutput {
    /// Status (always set; check before trusting `value`).
    pub status: ReaderStatus,
    /// Confidence in \[0, 1\] when applicable.
    pub confidence: f64,
    /// Reconstructed value only when `status == Reconstructed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    /// Content id when available (SHA-256 hex).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    /// Human-readable reason (no security theater).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Reader that produced this output.
    pub reader_id: String,
}

impl ReaderOutput {
    /// True when semantics are trustworthy.
    pub fn ok(&self) -> bool {
        self.status == ReaderStatus::Reconstructed && self.value.is_some()
    }
}

/// Trait for Neural readers (DSK-R).
pub trait ReaderBackend {
    /// Reader id.
    fn id(&self) -> &str;

    /// Compatibility declaration.
    fn compatibility(&self) -> ReaderCompatibility;

    /// Reconstruct semantics from an observation graph.
    ///
    /// MUST NOT invent plausible plaintext on failure — return a non-Reconstructed status.
    fn reconstruct(&self, observation: &ObservationGraph) -> Result<ReaderOutput>;

    /// Reset session state (isolation stub).
    fn reset(&mut self) {
        // Default: stateless.
    }
}

/// Deterministic baseline reader for CI — exact reverse of [`DeterministicComposer`].
#[derive(Debug, Default, Clone)]
pub struct DeterministicBaseline {
    /// Supported epochs.
    supported: Vec<GrammarEpoch>,
}

impl DeterministicBaseline {
    /// Create with E0001 support.
    pub fn new() -> Self {
        Self {
            supported: vec![GrammarEpoch::E0001],
        }
    }

    /// Create with an explicit epoch allow-list.
    pub fn with_epochs(supported: Vec<GrammarEpoch>) -> Self {
        Self { supported }
    }
}

impl ReaderBackend for DeterministicBaseline {
    fn id(&self) -> &str {
        "deterministic-baseline/1"
    }

    fn compatibility(&self) -> ReaderCompatibility {
        ReaderCompatibility {
            reader_id: self.id().into(),
            supported_epochs: self.supported.clone(),
        }
    }

    fn reconstruct(&self, observation: &ObservationGraph) -> Result<ReaderOutput> {
        if !observation.is_doldskrift {
            return Ok(ReaderOutput {
                status: ReaderStatus::NotDoldskrift,
                confidence: 0.0,
                value: None,
                content_id: None,
                message: Some("observation is not Doldskrift".into()),
                reader_id: self.id().into(),
            });
        }

        let Some(graph) = observation.latent_carrier.as_ref() else {
            return Ok(ReaderOutput {
                status: ReaderStatus::ReconstructionFailed,
                confidence: 0.0,
                value: None,
                content_id: None,
                message: Some(
                    "DeterministicBaseline requires latent_carrier (camera path not implemented)"
                        .into(),
                ),
                reader_id: self.id().into(),
            });
        };

        let epoch = &graph.meta.epoch;
        if !self.compatibility().supports(epoch) {
            let status = if epoch.number == 0 {
                ReaderStatus::UnknownGrammar
            } else {
                ReaderStatus::Incompatible
            };
            return Ok(ReaderOutput {
                status,
                confidence: 0.0,
                value: None,
                content_id: graph.meta.content_id.clone(),
                message: Some(format!(
                    "reader {} does not support epoch {}",
                    self.id(),
                    epoch
                )),
                reader_id: self.id().into(),
            });
        }

        match latent_to_value(graph) {
            Ok(value) => Ok(ReaderOutput {
                status: ReaderStatus::Reconstructed,
                confidence: 1.0,
                content_id: graph.meta.content_id.clone(),
                value: Some(value),
                message: None,
                reader_id: self.id().into(),
            }),
            Err(e) => Ok(ReaderOutput {
                status: ReaderStatus::ReconstructionFailed,
                confidence: 0.0,
                value: None,
                content_id: graph.meta.content_id.clone(),
                message: Some(e.to_string()),
                reader_id: self.id().into(),
            }),
        }
    }
}

/// Mock reader — alias behavior of DeterministicBaseline with a distinct id for demos.
#[derive(Debug, Default, Clone)]
pub struct MockReader {
    inner: DeterministicBaseline,
}

impl MockReader {
    /// Create mock reader.
    pub fn new() -> Self {
        Self {
            inner: DeterministicBaseline::new(),
        }
    }
}

impl ReaderBackend for MockReader {
    fn id(&self) -> &str {
        "mock-reader/1"
    }

    fn compatibility(&self) -> ReaderCompatibility {
        let mut c = self.inner.compatibility();
        c.reader_id = self.id().into();
        c
    }

    fn reconstruct(&self, observation: &ObservationGraph) -> Result<ReaderOutput> {
        let mut out = self.inner.reconstruct(observation)?;
        out.reader_id = self.id().into();
        Ok(out)
    }
}

/// Session-isolated wrapper that clears ephemeral state on reset.
#[derive(Debug, Clone)]
pub struct SessionReader<R: ReaderBackend> {
    inner: R,
    /// Call counter (stub isolation evidence).
    pub calls: u64,
}

impl<R: ReaderBackend> SessionReader<R> {
    /// Wrap a backend.
    pub fn new(inner: R) -> Self {
        Self { inner, calls: 0 }
    }
}

impl<R: ReaderBackend> ReaderBackend for SessionReader<R> {
    fn id(&self) -> &str {
        self.inner.id()
    }

    fn compatibility(&self) -> ReaderCompatibility {
        self.inner.compatibility()
    }

    fn reconstruct(&self, observation: &ObservationGraph) -> Result<ReaderOutput> {
        // Note: calls bump requires &mut — use reset path for isolation demos.
        self.inner.reconstruct(observation)
    }

    fn reset(&mut self) {
        self.calls = 0;
        self.inner.reset();
    }
}

/// Stub for future Gemma-based reader. Enabled only with `gemma-reader` feature.
/// Does **not** download or run weights; returns `ReconstructionFailed`.
#[cfg(feature = "gemma-reader")]
#[derive(Debug, Default, Clone)]
pub struct GemmaReaderStub;

#[cfg(feature = "gemma-reader")]
impl ReaderBackend for GemmaReaderStub {
    fn id(&self) -> &str {
        "gemma-reader/stub"
    }

    fn compatibility(&self) -> ReaderCompatibility {
        ReaderCompatibility {
            reader_id: self.id().into(),
            supported_epochs: vec![GrammarEpoch::E0001],
        }
    }

    fn reconstruct(&self, observation: &ObservationGraph) -> Result<ReaderOutput> {
        let _ = observation;
        Ok(ReaderOutput {
            status: ReaderStatus::ReconstructionFailed,
            confidence: 0.0,
            value: None,
            content_id: None,
            message: Some(
                "GemmaReader stub: trained inference not wired; use DeterministicBaseline/MockReader"
                    .into(),
            ),
            reader_id: self.id().into(),
        })
    }
}

/// Reader that only supports a future epoch — used in wrong-reader tests.
#[derive(Debug, Clone)]
pub struct EpochLockedReader {
    /// Epochs this reader accepts.
    pub supported: Vec<GrammarEpoch>,
    id: String,
}

impl EpochLockedReader {
    /// Lock to a specific epoch.
    pub fn only(epoch: GrammarEpoch) -> Self {
        Self {
            supported: vec![epoch],
            id: format!("epoch-locked/{}", epoch.label()),
        }
    }
}

impl ReaderBackend for EpochLockedReader {
    fn id(&self) -> &str {
        &self.id
    }

    fn compatibility(&self) -> ReaderCompatibility {
        ReaderCompatibility {
            reader_id: self.id.clone(),
            supported_epochs: self.supported.clone(),
        }
    }

    fn reconstruct(&self, observation: &ObservationGraph) -> Result<ReaderOutput> {
        DeterministicBaseline::with_epochs(self.supported.clone())
            .reconstruct(observation)
            .map(|mut o| {
                o.reader_id = self.id.clone();
                o
            })
    }
}

fn latent_to_value(graph: &LatentGraph) -> Result<Value> {
    let root = graph
        .nodes
        .iter()
        .find(|n| n.kind == LatentNodeKind::Root)
        .ok_or_else(|| Error::Semantic("latent graph missing root".into()))?;
    let child = graph
        .edges
        .iter()
        .find(|e| e.from == root.id && e.rel == "child")
        .ok_or_else(|| Error::Semantic("root missing child edge".into()))?;
    decode_node(graph, &child.to)
}

fn decode_node(graph: &LatentGraph, id: &str) -> Result<Value> {
    let node = graph
        .node(id)
        .ok_or_else(|| Error::Semantic(format!("missing node {id}")))?;
    match node.kind {
        LatentNodeKind::Null => Ok(Value::Null),
        LatentNodeKind::Atom | LatentNodeKind::Opaque => decode_atom(node),
        LatentNodeKind::Sequence => {
            let mut items: Vec<(usize, Value)> = Vec::new();
            for e in &graph.edges {
                if e.from == id {
                    if let Some(rest) = e.rel.strip_prefix("item:") {
                        let idx: usize = rest
                            .parse()
                            .map_err(|_| Error::Semantic("bad item index".into()))?;
                        items.push((idx, decode_node(graph, &e.to)?));
                    }
                }
            }
            items.sort_by_key(|(i, _)| *i);
            Ok(Value::Array(items.into_iter().map(|(_, v)| v).collect()))
        }
        LatentNodeKind::Record => {
            let mut map = BTreeMap::new();
            for e in &graph.edges {
                if e.from == id && e.rel == "field" {
                    let field = graph
                        .node(&e.to)
                        .ok_or_else(|| Error::Semantic("missing field node".into()))?;
                    let name = field
                        .attrs
                        .get("name")
                        .cloned()
                        .or_else(|| {
                            field
                                .value
                                .as_ref()
                                .and_then(|v| v.as_str().map(str::to_string))
                        })
                        .ok_or_else(|| Error::Semantic("field missing name".into()))?;
                    let value_edge = graph
                        .edges
                        .iter()
                        .find(|x| x.from == field.id && x.rel == "value")
                        .ok_or_else(|| Error::Semantic("field missing value".into()))?;
                    map.insert(name, decode_node(graph, &value_edge.to)?);
                }
            }
            Ok(Value::Map(map))
        }
        LatentNodeKind::Field | LatentNodeKind::Root => Err(Error::Semantic(
            "unexpected node kind at decode root of subtree".into(),
        )),
    }
}

fn decode_atom(node: &super::lsg::LatentNode) -> Result<Value> {
    let v = node
        .value
        .as_ref()
        .ok_or_else(|| Error::Semantic("atom missing value".into()))?;
    if let Some(t) = node.attrs.get("type") {
        match t.as_str() {
            "timestamp_ms" => {
                let n = v
                    .as_i64()
                    .ok_or_else(|| Error::Semantic("bad timestamp".into()))?;
                return Ok(Value::Timestamp(n));
            }
            "uuid" => {
                let s = v
                    .as_str()
                    .ok_or_else(|| Error::Semantic("bad uuid".into()))?;
                let bytes = hex::decode(s).map_err(|e| Error::Semantic(e.to_string()))?;
                if bytes.len() != 16 {
                    return Err(Error::Semantic("uuid must be 16 bytes".into()));
                }
                let mut arr = [0u8; 16];
                arr.copy_from_slice(&bytes);
                return Ok(Value::Uuid(arr));
            }
            "uri" => {
                return Ok(Value::Uri(
                    v.as_str()
                        .ok_or_else(|| Error::Semantic("bad uri".into()))?
                        .into(),
                ));
            }
            "identifier" => {
                return Ok(Value::Identifier(
                    v.as_str()
                        .ok_or_else(|| Error::Semantic("bad identifier".into()))?
                        .into(),
                ));
            }
            "bytes-hex" => {
                let s = v
                    .as_str()
                    .ok_or_else(|| Error::Semantic("bad bytes".into()))?;
                let bytes = hex::decode(s).map_err(|e| Error::Semantic(e.to_string()))?;
                return Ok(Value::Bytes(bytes));
            }
            _ => {}
        }
    }
    match v {
        serde_json::Value::Null => Ok(Value::Null),
        serde_json::Value::Bool(b) => Ok(Value::Bool(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::Float(f))
            } else {
                Err(Error::Semantic("unsupported number".into()))
            }
        }
        serde_json::Value::String(s) => Ok(Value::String(s.clone())),
        other => Err(Error::Semantic(format!("unsupported atom JSON: {other}"))),
    }
}
