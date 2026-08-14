//! Semantic value layer for DSK/2 — structured machine information.

mod schema;
mod tokens;
mod value;

pub use schema::{FieldDef, Schema, SchemaRegistry, TypeName};
pub use tokens::{SemanticToken, TokenKind};
pub use value::{Value, ValueRef};

use crate::{Error, Result};
use tokens::encode_tokens;
use value::canonical_encode;

/// Encode a semantic [`Value`] to DSK/2 token bytes (canonical when `canonical` is true).
pub fn encode_value(value: &Value, canonical: bool) -> Result<Vec<u8>> {
    if canonical {
        canonical_encode(value)
    } else {
        encode_tokens(value)
    }
}

/// Decode DSK/2 token bytes into a [`Value`].
pub fn decode_value(bytes: &[u8]) -> Result<Value> {
    tokens::decode_tokens(bytes)
}

/// Encode a JSON value as Doldskrift semantic tokens (convenience).
pub fn encode_json(json: &serde_json::Value) -> Result<Vec<u8>> {
    let v = Value::from_json(json)?;
    encode_value(&v, true)
}

/// Decode semantic tokens to JSON.
pub fn decode_to_json(bytes: &[u8]) -> Result<serde_json::Value> {
    Ok(decode_value(bytes)?.to_json())
}

/// Content-address a canonical semantic payload.
pub fn content_id(value: &Value) -> Result<String> {
    let bytes = encode_value(value, true)?;
    Ok(crate::address::dsk_sha256(&bytes))
}

/// Validate that a value conforms to an optional schema.
pub fn validate(value: &Value, schema: Option<&Schema>) -> Result<()> {
    match schema {
        None => Ok(()),
        Some(s) => s.validate(value).map_err(Error::Semantic),
    }
}
