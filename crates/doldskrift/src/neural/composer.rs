//! Semantic → LatentGraph composition.

use crate::semantic::Value;
use crate::Result;
use std::collections::BTreeMap;

use super::epoch::GrammarEpoch;
use super::lsg::{LatentEdge, LatentGraph, LatentMetadata, LatentNode, LatentNodeKind};

/// Compose a semantic value into a latent structure graph.
pub trait SemanticComposer {
    /// Composer identifier (for metadata / manifests).
    fn id(&self) -> &str;

    /// Grammar epoch this composer emits.
    fn epoch(&self) -> GrammarEpoch;

    /// Compose `value` into an LSG graph.
    fn compose(&self, value: &Value) -> Result<LatentGraph>;
}

/// Deterministic tree composer for CI and foundation demos (LSG/1-E0001).
#[derive(Debug, Default, Clone, Copy)]
pub struct DeterministicComposer;

impl SemanticComposer for DeterministicComposer {
    fn id(&self) -> &str {
        "deterministic-composer/1"
    }

    fn epoch(&self) -> GrammarEpoch {
        GrammarEpoch::E0001
    }

    fn compose(&self, value: &Value) -> Result<LatentGraph> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut next = 0u32;
        let root = alloc_id(&mut next, "n");
        nodes.push(LatentNode {
            id: root.clone(),
            kind: LatentNodeKind::Root,
            value: None,
            attrs: BTreeMap::new(),
        });
        let child = compose_value(value, &mut nodes, &mut edges, &mut next)?;
        edges.push(LatentEdge {
            from: root,
            to: child,
            rel: "child".into(),
        });

        let content_id = crate::semantic::content_id(value).ok();
        Ok(LatentGraph {
            meta: LatentMetadata {
                epoch: self.epoch(),
                content_id,
                composer: Some(self.id().into()),
            },
            nodes,
            edges,
        })
    }
}

/// Convenience: compose with [`DeterministicComposer`].
pub fn compose_deterministic(value: &Value) -> Result<LatentGraph> {
    DeterministicComposer.compose(value)
}

fn alloc_id(next: &mut u32, prefix: &str) -> String {
    let id = format!("{prefix}{next}");
    *next += 1;
    id
}

fn compose_value(
    value: &Value,
    nodes: &mut Vec<LatentNode>,
    edges: &mut Vec<LatentEdge>,
    next: &mut u32,
) -> Result<String> {
    match value {
        Value::Null => {
            let id = alloc_id(next, "n");
            nodes.push(LatentNode {
                id: id.clone(),
                kind: LatentNodeKind::Null,
                value: Some(serde_json::Value::Null),
                attrs: BTreeMap::new(),
            });
            Ok(id)
        }
        Value::Bool(b) => push_atom(nodes, next, serde_json::Value::Bool(*b)),
        Value::Integer(i) => push_atom(nodes, next, serde_json::json!(*i)),
        Value::Float(f) => push_atom(nodes, next, serde_json::json!(*f)),
        Value::String(s) => push_atom(nodes, next, serde_json::Value::String(s.clone())),
        Value::Bytes(b) => {
            let mut attrs = BTreeMap::new();
            attrs.insert("type".into(), "bytes-hex".into());
            attrs.insert("len".into(), b.len().to_string());
            push_atom_attrs(
                nodes,
                next,
                serde_json::Value::String(hex::encode(b)),
                attrs,
            )
        }
        Value::Timestamp(t) => {
            let mut attrs = BTreeMap::new();
            attrs.insert("type".into(), "timestamp_ms".into());
            let id = alloc_id(next, "n");
            nodes.push(LatentNode {
                id: id.clone(),
                kind: LatentNodeKind::Atom,
                value: Some(serde_json::json!(*t)),
                attrs,
            });
            Ok(id)
        }
        Value::Uuid(u) => {
            let hex = hex::encode(u);
            let mut attrs = BTreeMap::new();
            attrs.insert("type".into(), "uuid".into());
            push_atom_attrs(nodes, next, serde_json::Value::String(hex), attrs)
        }
        Value::Uri(s) | Value::Identifier(s) => {
            let mut attrs = BTreeMap::new();
            attrs.insert(
                "type".into(),
                if matches!(value, Value::Uri(_)) {
                    "uri"
                } else {
                    "identifier"
                }
                .into(),
            );
            push_atom_attrs(nodes, next, serde_json::Value::String(s.clone()), attrs)
        }
        Value::Array(items) => {
            let id = alloc_id(next, "n");
            nodes.push(LatentNode {
                id: id.clone(),
                kind: LatentNodeKind::Sequence,
                value: None,
                attrs: BTreeMap::new(),
            });
            for (i, item) in items.iter().enumerate() {
                let child = compose_value(item, nodes, edges, next)?;
                edges.push(LatentEdge {
                    from: id.clone(),
                    to: child,
                    rel: format!("item:{i}"),
                });
            }
            Ok(id)
        }
        Value::Map(map) => {
            let id = alloc_id(next, "n");
            nodes.push(LatentNode {
                id: id.clone(),
                kind: LatentNodeKind::Record,
                value: None,
                attrs: BTreeMap::new(),
            });
            for (k, v) in map {
                let field_id = alloc_id(next, "n");
                let mut attrs = BTreeMap::new();
                attrs.insert("name".into(), k.clone());
                nodes.push(LatentNode {
                    id: field_id.clone(),
                    kind: LatentNodeKind::Field,
                    value: Some(serde_json::Value::String(k.clone())),
                    attrs,
                });
                edges.push(LatentEdge {
                    from: id.clone(),
                    to: field_id.clone(),
                    rel: "field".into(),
                });
                let child = compose_value(v, nodes, edges, next)?;
                edges.push(LatentEdge {
                    from: field_id,
                    to: child,
                    rel: "value".into(),
                });
            }
            Ok(id)
        }
    }
}

fn push_atom(
    nodes: &mut Vec<LatentNode>,
    next: &mut u32,
    value: serde_json::Value,
) -> Result<String> {
    push_atom_attrs(nodes, next, value, BTreeMap::new())
}

fn push_atom_attrs(
    nodes: &mut Vec<LatentNode>,
    next: &mut u32,
    value: serde_json::Value,
    attrs: BTreeMap<String, String>,
) -> Result<String> {
    let id = alloc_id(next, "n");
    nodes.push(LatentNode {
        id: id.clone(),
        kind: LatentNodeKind::Atom,
        value: Some(value),
        attrs,
    });
    Ok(id)
}
