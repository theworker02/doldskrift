//! Session mode: seeded dynamic mappings + manifests.

use crate::codec::{Mapping, MappingId};
use crate::{checksum_bytes, Error, Mode, Result, PROTOCOL_NAME, PROTOCOL_VERSION};
use serde::{Deserialize, Serialize};

/// Machine-readable session manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionManifest {
    /// Protocol family name (`doldskrift`).
    pub protocol: String,
    /// Protocol major version.
    pub version: u8,
    /// Always `session` for this manifest type.
    pub mode: String,
    /// Opaque session identifier.
    pub session_id: String,
    /// Mapping identifier.
    pub mapping_id: String,
    /// Alphabet description.
    pub alphabet: String,
    /// Hex-encoded CRC32C of the encode table.
    pub checksum: String,
    /// Optional hex-encoded seed (representation only — not a secret).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_hex: Option<String>,
}

impl SessionManifest {
    /// Serialize to compact JSON.
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(|e| Error::InvalidManifest(e.to_string()))
    }

    /// Serialize to pretty JSON.
    pub fn to_json_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| Error::InvalidManifest(e.to_string()))
    }

    /// Parse from JSON.
    pub fn from_json(s: &str) -> Result<Self> {
        let m: Self = serde_json::from_str(s).map_err(|e| Error::InvalidManifest(e.to_string()))?;
        m.validate()?;
        Ok(m)
    }

    /// Basic structural validation.
    pub fn validate(&self) -> Result<()> {
        if self.protocol != PROTOCOL_NAME {
            return Err(Error::InvalidManifest(format!(
                "unexpected protocol '{}'",
                self.protocol
            )));
        }
        if self.version != PROTOCOL_VERSION {
            return Err(Error::UnsupportedVersion(self.version));
        }
        if self.mode != Mode::Session.as_str() {
            return Err(Error::InvalidManifest(format!(
                "expected mode 'session', got '{}'",
                self.mode
            )));
        }
        if self.session_id.is_empty() || self.mapping_id.is_empty() {
            return Err(Error::InvalidManifest(
                "session_id and mapping_id must be non-empty".into(),
            ));
        }
        Ok(())
    }
}

/// A live session binding a seed to a mapping.
#[derive(Debug, Clone)]
pub struct Session {
    seed: Vec<u8>,
    session_id: String,
    mapping: Mapping,
    manifest: SessionManifest,
}

impl Session {
    /// Start a session builder.
    pub fn builder() -> SessionBuilder {
        SessionBuilder::default()
    }

    /// Reconstruct a session from an explicit seed and session id.
    pub fn from_seed(seed: impl AsRef<[u8]>, session_id: impl Into<String>) -> Result<Self> {
        SessionBuilder::default()
            .seed(seed.as_ref())
            .session_id(session_id)
            .build()
    }

    /// Session identifier.
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Mapping identifier.
    pub fn mapping_id(&self) -> &MappingId {
        self.mapping.id()
    }

    /// Borrow the mapping.
    pub fn mapping(&self) -> &Mapping {
        &self.mapping
    }

    /// Borrow the manifest.
    pub fn manifest(&self) -> &SessionManifest {
        &self.manifest
    }

    /// Encode with the session mapping.
    pub fn encode(&self, text: &str) -> Result<String> {
        Ok(self.mapping.encode_str(text))
    }

    /// Decode with the session mapping.
    pub fn decode(&self, encoded: &str) -> Result<String> {
        self.mapping.decode_str(encoded)
    }

    /// Seed bytes (not a cryptographic secret).
    pub fn seed(&self) -> &[u8] {
        &self.seed
    }
}

/// Builder for [`Session`].
#[derive(Debug, Default)]
pub struct SessionBuilder {
    seed: Option<Vec<u8>>,
    session_id: Option<String>,
}

impl SessionBuilder {
    /// Set the deterministic seed.
    pub fn seed(mut self, seed: impl AsRef<[u8]>) -> Self {
        self.seed = Some(seed.as_ref().to_vec());
        self
    }

    /// Set a human/agent-readable session id.
    pub fn session_id(mut self, id: impl Into<String>) -> Self {
        self.session_id = Some(id.into());
        self
    }

    /// Build the session.
    pub fn build(self) -> Result<Session> {
        let seed = self.seed.unwrap_or_default();
        let mapping = Mapping::from_seed(&seed)?;
        let session_id = self
            .session_id
            .unwrap_or_else(|| format!("sess-{:08x}", checksum_bytes(&seed)));
        let table_checksum = checksum_bytes(mapping.encode_table());
        let manifest = SessionManifest {
            protocol: PROTOCOL_NAME.into(),
            version: PROTOCOL_VERSION,
            mode: Mode::Session.as_str().into(),
            session_id: session_id.clone(),
            mapping_id: mapping.id().to_string(),
            alphabet: format!("pua-byte-{}", crate::ALPHABET_SIZE),
            checksum: format!("{table_checksum:08x}"),
            seed_hex: Some(hex::encode(&seed)),
        };
        manifest.validate()?;
        Ok(Session {
            seed,
            session_id,
            mapping,
            manifest,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_api() {
        let session = Session::builder().seed(b"demo").build().unwrap();
        let payload = session.encode("hello").unwrap();
        assert_eq!(session.decode(&payload).unwrap(), "hello");
        let json = session.manifest().to_json().unwrap();
        let parsed = SessionManifest::from_json(&json).unwrap();
        assert_eq!(parsed.session_id, session.session_id());
    }
}
