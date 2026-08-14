//! Gate capability policy stubs (shared vocabulary with Protected track).
//!
//! Honest limits: these are policy *types* only. No crypto, no TPM, no remote auth.

use serde::{Deserialize, Serialize};

/// Named capability token for Gate-scoped operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GateCapability {
    /// Capability id, e.g. `dsk.inspect`, `dsk.reconstruct`.
    pub id: String,
}

impl GateCapability {
    /// Inspect metadata only.
    pub fn inspect() -> Self {
        Self {
            id: "dsk.inspect".into(),
        }
    }

    /// Reconstruct semantics (Neural reader or Protected decrypt path).
    pub fn reconstruct() -> Self {
        Self {
            id: "dsk.reconstruct".into(),
        }
    }

    /// Export plaintext (high privilege; Protected).
    pub fn export_plaintext() -> Self {
        Self {
            id: "dsk.capability.export.plaintext".into(),
        }
    }
}

/// Simple allow-list policy stub.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatePolicy {
    /// Allowed capability ids.
    pub allowed: Vec<String>,
}

impl GatePolicy {
    /// Whether `cap` is allowed.
    pub fn allows(&self, cap: &GateCapability) -> bool {
        self.allowed.iter().any(|a| a == &cap.id)
    }

    /// Permissive local-dev policy (not a security boundary).
    pub fn permissive_dev() -> Self {
        Self {
            allowed: vec![
                "dsk.inspect".into(),
                "dsk.reconstruct".into(),
                "dsk.capability.export.plaintext".into(),
            ],
        }
    }
}
