//! Compact semantic value type system.

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// DSK/2 semantic value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    /// Null.
    Null,
    /// Boolean.
    Bool(bool),
    /// Signed 64-bit integer.
    Integer(i64),
    /// IEEE-754 f64 (stored as bits in canonical form).
    Float(f64),
    /// UTF-8 string.
    String(String),
    /// Raw bytes.
    Bytes(Vec<u8>),
    /// Ordered array.
    Array(Vec<Value>),
    /// Map with deterministically ordered string keys.
    Map(BTreeMap<String, Value>),
    /// Unix timestamp in milliseconds.
    Timestamp(i64),
    /// UUID as 16 bytes.
    Uuid([u8; 16]),
    /// URI string.
    Uri(String),
    /// Identifier (symbol-like name).
    Identifier(String),
}

/// Borrowed view helper (placeholder for future zero-copy).
pub type ValueRef<'a> = &'a Value;

impl Value {
    /// Convert from `serde_json::Value`.
    pub fn from_json(v: &serde_json::Value) -> Result<Self> {
        Ok(match v {
            serde_json::Value::Null => Self::Null,
            serde_json::Value::Bool(b) => Self::Bool(*b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Self::Integer(i)
                } else if let Some(u) = n.as_u64() {
                    if u > i64::MAX as u64 {
                        return Err(Error::Semantic("integer out of i64 range".into()));
                    }
                    Self::Integer(u as i64)
                } else if let Some(f) = n.as_f64() {
                    Self::Float(f)
                } else {
                    return Err(Error::Semantic("unsupported number".into()));
                }
            }
            serde_json::Value::String(s) => Self::String(s.clone()),
            serde_json::Value::Array(a) => {
                let mut out = Vec::with_capacity(a.len());
                for x in a {
                    out.push(Self::from_json(x)?);
                }
                Self::Array(out)
            }
            serde_json::Value::Object(o) => {
                let mut map = BTreeMap::new();
                for (k, val) in o {
                    map.insert(k.clone(), Self::from_json(val)?);
                }
                Self::Map(map)
            }
        })
    }

    /// Convert to JSON (UUID/bytes become hex strings; timestamps become numbers).
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Self::Null => serde_json::Value::Null,
            Self::Bool(b) => serde_json::Value::Bool(*b),
            Self::Integer(i) => serde_json::json!(*i),
            Self::Float(f) => serde_json::json!(*f),
            Self::String(s) | Self::Uri(s) | Self::Identifier(s) => {
                serde_json::Value::String(s.clone())
            }
            Self::Bytes(b) => serde_json::Value::String(hex::encode(b)),
            Self::Array(a) => serde_json::Value::Array(a.iter().map(Self::to_json).collect()),
            Self::Map(m) => {
                let mut o = serde_json::Map::new();
                for (k, v) in m {
                    o.insert(k.clone(), v.to_json());
                }
                serde_json::Value::Object(o)
            }
            Self::Timestamp(t) => serde_json::json!(*t),
            Self::Uuid(u) => serde_json::Value::String(hex::encode(u)),
        }
    }

    /// Lookup a child by map key.
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Self::Map(m) => m.get(key),
            _ => None,
        }
    }

    /// Index into an array.
    pub fn index(&self, i: usize) -> Option<&Value> {
        match self {
            Self::Array(a) => a.get(i),
            _ => None,
        }
    }
}

/// Canonical binary encoding of a value (deterministic map order via BTreeMap).
pub fn canonical_encode(value: &Value) -> Result<Vec<u8>> {
    super::tokens::encode_tokens(value)
}
