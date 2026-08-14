//! Named schema / capability registries.

use crate::protocol::CapabilityProfile;
use crate::semantic::{Schema, SchemaRegistry};

/// Built-in schema registry with no entries (callers register as needed).
pub fn empty_schema_registry() -> SchemaRegistry {
    SchemaRegistry::new()
}

/// Register a schema, returning the updated registry.
pub fn with_schema(mut reg: SchemaRegistry, schema: Schema) -> SchemaRegistry {
    reg.register(schema);
    reg
}

/// Default capability profile for the reference stack.
pub fn default_profile() -> CapabilityProfile {
    CapabilityProfile::Standard
}
