//! Protected Visual Objects — ADR-0006 milestone 1–2 stubs.
//!
//! Honest status: metadata + refuse paths only. No AEAD, no real Gate crypto.
//! Open encode/decode is unchanged. Neural ≠ encryption; Protected *is* the
//! named encryption track when it ships.

use crate::document::DskDocument;
use crate::neural::GateCapability;
use crate::{Error, Mode, Result, PROTOCOL_VERSION_PROTECTED};
use serde::Serialize;

/// Inspect fields for Protected objects (ADR-0006 example shape).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProtectedInspect {
    /// Protocol label, e.g. `DSK/3`.
    pub protocol: String,
    /// Visual engine label, e.g. `MGE/5`.
    pub visual: String,
    /// Payload kind (`Protected`).
    pub payload: String,
    /// Cipher label (`AEAD` placeholder until real crypto).
    pub cipher: String,
    /// Session required for semantic access.
    pub session: String,
    /// Agent policy required.
    pub agent_policy: String,
    /// Integrity of container checksum (`Valid` / `Invalid`).
    pub integrity: String,
    /// Always `No` for Protected — glyphs are not readable as plaintext.
    pub readable: String,
    /// Explicit honesty note for operators.
    pub notes: String,
}

impl ProtectedInspect {
    /// Build from a Protected document (caller must check mode).
    pub fn from_document(doc: &DskDocument) -> Self {
        let integrity = if doc.inspect().checksum_valid {
            "Valid"
        } else {
            "Invalid"
        };
        Self {
            protocol: format!("DSK/{}", doc.header.version),
            visual: doc
                .header
                .meta
                .get("visual")
                .and_then(|v| v.as_str())
                .unwrap_or("MGE/5")
                .into(),
            payload: "Protected".into(),
            cipher: doc
                .header
                .meta
                .get("cipher")
                .and_then(|v| v.as_str())
                .unwrap_or("AEAD-stub")
                .into(),
            session: "Required".into(),
            agent_policy: "Required".into(),
            integrity: integrity.into(),
            readable: "No".into(),
            notes: "Stub container — no AEAD ciphertext yet. Open decode refuses; Gate open stubs AccessDenied.".into(),
        }
    }

    /// CLI-friendly block matching ADR-0006 example fields.
    pub fn display(&self) -> String {
        format!(
            "Protocol: {}\n\
             Visual: {}\n\
             Payload: {}\n\
             Cipher: {}\n\
             Session: {}\n\
             Agent Policy: {}\n\
             Integrity: {}\n\
             Readable: {}\n\
             Note: {}",
            self.protocol,
            self.visual,
            self.payload,
            self.cipher,
            self.session,
            self.agent_policy,
            self.integrity,
            self.readable,
            self.notes
        )
    }
}

/// Gate-mediated open (stub). Always `AccessDenied` for Protected until AEAD ships.
///
/// Open-mode documents fall through to normal `decode_text`.
pub fn gate_open(doc: &DskDocument, capability: Option<&GateCapability>) -> Result<String> {
    if doc.header.mode != Mode::Protected {
        return doc.decode_text();
    }
    match capability {
        None => Err(Error::AccessDenied(
            "Protected object: no capability provided. Semantic recovery requires Gate + capability (AEAD not implemented). Use `dold inspect` for metadata."
                .into(),
        )),
        Some(cap) => Err(Error::AccessDenied(format!(
            "Protected object: capability '{}' presented, but Gate AEAD decrypt is not implemented (Phase V stub). No plaintext path.",
            cap.id
        ))),
    }
}

/// Stamp Protected metadata on a header (shared by stub constructors).
pub(crate) fn stamp_protected_meta(header: &mut crate::document::DocumentHeader) {
    header.version = PROTOCOL_VERSION_PROTECTED;
    header
        .meta
        .insert("glyph_engine".into(), serde_json::json!(5));
    header
        .meta
        .insert("payload_type".into(), serde_json::json!("Protected"));
    header
        .meta
        .insert("payload".into(), serde_json::json!("Protected"));
    header
        .meta
        .insert("cipher".into(), serde_json::json!("AEAD-stub"));
    header
        .meta
        .insert("visual".into(), serde_json::json!("MGE/5"));
    header
        .meta
        .insert("session_required".into(), serde_json::json!(true));
    header
        .meta
        .insert("agent_policy_required".into(), serde_json::json!(true));
    header
        .meta
        .insert("readable".into(), serde_json::json!(false));
    header.meta.insert("dsk".into(), serde_json::json!(3));
    header.meta.insert(
        "notes".into(),
        serde_json::json!("Protected stub — placeholder bytes, not real AEAD"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DskDocument;

    #[test]
    fn protected_stub_refuses_decode() {
        let doc = DskDocument::encode_protected_stub("demo").unwrap();
        assert_eq!(doc.header.mode, Mode::Protected);
        assert_eq!(doc.header.version, PROTOCOL_VERSION_PROTECTED);
        let err = doc.decode_text().unwrap_err();
        assert!(matches!(err, Error::AccessDenied(_)));
    }

    #[test]
    fn gate_open_without_capability() {
        let doc = DskDocument::encode_protected_stub("x").unwrap();
        let err = gate_open(&doc, None).unwrap_err();
        assert!(matches!(err, Error::AccessDenied(_)));
    }

    #[test]
    fn gate_open_with_capability_still_denied() {
        let doc = DskDocument::encode_protected_stub("x").unwrap();
        let cap = GateCapability::export_plaintext();
        let err = gate_open(&doc, Some(&cap)).unwrap_err();
        assert!(matches!(err, Error::AccessDenied(_)));
    }

    #[test]
    fn inspect_readable_no() {
        let doc = DskDocument::encode_protected_stub("meta").unwrap();
        let p = ProtectedInspect::from_document(&doc);
        assert_eq!(p.readable, "No");
        assert_eq!(p.payload, "Protected");
        assert!(p.display().contains("Readable: No"));
    }

    #[test]
    fn open_mode_still_decodes() {
        let doc = DskDocument::encode_text("hello", Mode::Encoded).unwrap();
        assert_eq!(gate_open(&doc, None).unwrap(), "hello");
    }
}
