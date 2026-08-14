//! Optional schema declarations for DSK/2 documents.

use super::Value;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Built-in type names.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TypeName {
    /// Null.
    Null,
    /// Bool.
    Bool,
    /// Integer.
    Integer,
    /// Float.
    Float,
    /// String.
    String,
    /// Bytes.
    Bytes,
    /// Array of a type.
    Array(Box<TypeName>),
    /// Map / object with named fields.
    Map,
    /// Timestamp.
    Timestamp,
    /// UUID.
    Uuid,
    /// URI.
    Uri,
    /// Identifier.
    Identifier,
    /// Named schema reference.
    Ref(String),
    /// Any value.
    Any,
}

/// Field in a schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldDef {
    /// Field name.
    pub name: String,
    /// Field type.
    pub ty: TypeName,
    /// Whether the field is required.
    #[serde(default = "default_true")]
    pub required: bool,
}

fn default_true() -> bool {
    true
}

/// Named schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    /// Schema name.
    pub name: String,
    /// Fields (for map-like values).
    pub fields: Vec<FieldDef>,
}

impl Schema {
    /// Validate a value against this schema (map expected).
    pub fn validate(&self, value: &Value) -> Result<(), String> {
        let Value::Map(m) = value else {
            return Err(format!("schema {} expects a map", self.name));
        };
        for f in &self.fields {
            match m.get(&f.name) {
                None if f.required => return Err(format!("missing field {}", f.name)),
                None => {}
                Some(v) => check_type(v, &f.ty)?,
            }
        }
        Ok(())
    }
}

fn check_type(v: &Value, ty: &TypeName) -> Result<(), String> {
    let ok = matches!(
        (v, ty),
        (Value::Null, TypeName::Null | TypeName::Any)
            | (Value::Bool(_), TypeName::Bool | TypeName::Any)
            | (Value::Integer(_), TypeName::Integer | TypeName::Any)
            | (Value::Float(_), TypeName::Float | TypeName::Any)
            | (Value::String(_), TypeName::String | TypeName::Any)
            | (Value::Bytes(_), TypeName::Bytes | TypeName::Any)
            | (Value::Timestamp(_), TypeName::Timestamp | TypeName::Any)
            | (Value::Uuid(_), TypeName::Uuid | TypeName::Any)
            | (Value::Uri(_), TypeName::Uri | TypeName::Any)
            | (Value::Identifier(_), TypeName::Identifier | TypeName::Any)
            | (Value::Array(_), TypeName::Array(_) | TypeName::Any)
            | (Value::Map(_), TypeName::Map | TypeName::Any)
            | (_, TypeName::Ref(_))
    );
    if ok {
        Ok(())
    } else {
        Err(format!("type mismatch: expected {ty:?}"))
    }
}

/// In-memory schema registry.
#[derive(Debug, Default, Clone)]
pub struct SchemaRegistry {
    schemas: BTreeMap<String, Schema>,
}

impl SchemaRegistry {
    /// Empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace a schema.
    pub fn register(&mut self, schema: Schema) {
        self.schemas.insert(schema.name.clone(), schema);
    }

    /// Lookup by name.
    pub fn get(&self, name: &str) -> Option<&Schema> {
        self.schemas.get(name)
    }
}
