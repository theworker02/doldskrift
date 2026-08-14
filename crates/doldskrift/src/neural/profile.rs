//! Open / Neural / Protected profile identifiers.

use serde::{Deserialize, Serialize};

/// Top-level Doldskrift product profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileKind {
    /// DSK/1–2 open codec (shipping).
    Open,
    /// DSK/3 Neural — learned latent grammar (foundation).
    Neural,
    /// DSK/3 Protected — AEAD + Gate (design; not shipping crypto).
    Protected,
}

impl ProfileKind {
    /// Wire / inspect label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Neural => "neural",
            Self::Protected => "protected",
        }
    }
}

/// DSK/3 sub-profile metadata carried in manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dsk3Profile {
    /// Neural or Protected (Open uses DSK/1–2, not this struct).
    pub kind: ProfileKind,
    /// Grammar epoch string when Neural (e.g. `LSG/1-E0001`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grammar_epoch: Option<String>,
    /// Reader family expected for reconstruction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reader_family: Option<String>,
    /// Human-readable notes (never a security claim).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl Dsk3Profile {
    /// Neural profile with a grammar epoch.
    pub fn neural(epoch: impl Into<String>) -> Self {
        Self {
            kind: ProfileKind::Neural,
            grammar_epoch: Some(epoch.into()),
            reader_family: Some("deterministic-baseline".into()),
            notes: Some("Neural representation — not encryption".into()),
        }
    }

    /// Protected profile stub (no crypto yet).
    pub fn protected_stub() -> Self {
        Self {
            kind: ProfileKind::Protected,
            grammar_epoch: None,
            reader_family: None,
            notes: Some("Protected/AEAD + Gate — design only; not shipping".into()),
        }
    }
}
