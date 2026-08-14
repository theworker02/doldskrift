//! Human-readable inspection reports (never include decoded payload by default).

use super::DskDocument;
use crate::{Mode, GLYPH_ENGINE_VERSION, MAGIC, PROTOCOL_NAME};
use serde::Serialize;

/// Diagnostic summary for `dold inspect`.
///
/// JSON shape is stable for agents under `schema = "doldskrift.inspect/1"`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InspectReport {
    /// Report schema id for machine consumers.
    pub schema: String,
    /// Container magic ASCII (`DSK1`).
    pub magic: String,
    /// Protocol label, e.g. `DSK/1`.
    pub protocol: String,
    /// Wire protocol version byte.
    pub protocol_version: u8,
    /// Glyph engine version.
    pub glyph_engine: u32,
    /// Mode name.
    pub mode: String,
    /// Profile bucket: `open`, `neural`, or `protected`.
    pub profile: String,
    /// Payload size in bytes.
    pub payload_bytes: usize,
    /// Optional character / symbol count from header.
    pub characters: Option<u64>,
    /// Optional semantic byte count from header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_count: Option<u64>,
    /// Payload kind description.
    pub payload_kind: String,
    /// True when payload is marked BINARY.
    pub binary_payload: bool,
    /// Mapping description.
    pub mapping: String,
    /// Optional mapping id from header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping_id: Option<String>,
    /// Density if recorded in metadata.
    pub density: Option<String>,
    /// Whether the stored checksum matched on parse.
    pub checksum_valid: bool,
    /// Checksum value.
    pub checksum: u32,
    /// Checksum algorithm label (integrity, not a MAC).
    pub checksum_algorithm: String,
    /// Font version.
    pub font_version: u32,
    /// Raw flags byte.
    pub flags: u8,
    /// Session id if present.
    pub session_id: Option<String>,
    /// Required capabilities hint.
    pub capabilities: Vec<String>,
    /// Sorted meta keys present in the header (no values — avoid surprise leaks).
    pub meta_keys: Vec<String>,
    /// Visual engine label (Protected / Neural).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual: Option<String>,
    /// Cipher label when Protected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher: Option<String>,
    /// Session requirement (`Required` for Protected).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_requirement: Option<String>,
    /// Agent policy requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_policy: Option<String>,
    /// Readable flag (`Yes` / `No`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readable: Option<String>,
    /// True when this is the honest Protected stub (not real AEAD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_stub: Option<bool>,
    /// Short agent-oriented hints (never includes plaintext).
    pub agent_hints: Vec<String>,
}

impl InspectReport {
    /// Build from a parsed document (checksum already verified by parse).
    pub fn from_document(doc: &DskDocument) -> Self {
        let mapping = match doc.header.mode {
            Mode::Visual => "none (visual)".into(),
            Mode::Encoded => doc
                .header
                .mapping_id
                .clone()
                .unwrap_or_else(|| "static-v1".into()),
            Mode::Session => "dynamic".into(),
            Mode::Protected => "ciphertext-symbols (Gate)".into(),
        };
        let payload_kind = doc
            .header
            .meta
            .get("payload_type")
            .and_then(|v| v.as_str())
            .unwrap_or(match doc.header.mode {
                Mode::Visual => "UTF-8",
                Mode::Protected => "Protected",
                _ => "UTF-8-symbols",
            })
            .to_string();
        let density = doc
            .header
            .meta
            .get("density")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let mut capabilities = vec!["DSK_CONTAINER".into()];
        capabilities.push(match doc.header.mode {
            Mode::Visual => "DSK_VISUAL".into(),
            Mode::Encoded => "DSK_ENCODED".into(),
            Mode::Session => "DSK_SESSION".into(),
            Mode::Protected => "DSK_PROTECTED".into(),
        });

        let profile = match doc.header.mode {
            Mode::Protected => "protected",
            _ => "open",
        }
        .to_string();

        let (visual, cipher, session_requirement, agent_policy, readable, protected_stub) =
            if doc.header.mode == Mode::Protected {
                let cipher_label = doc
                    .header
                    .meta
                    .get("cipher")
                    .and_then(|v| v.as_str())
                    .unwrap_or("AEAD-stub")
                    .to_string();
                let stub =
                    cipher_label == "AEAD-stub" || doc.payload.starts_with(b"PROTECTED-STUB");
                (
                    Some(
                        doc.header
                            .meta
                            .get("visual")
                            .and_then(|v| v.as_str())
                            .unwrap_or("MGE/5")
                            .to_string(),
                    ),
                    Some(cipher_label),
                    Some("Required".into()),
                    Some("Required".into()),
                    Some("No".into()),
                    Some(stub),
                )
            } else {
                (None, None, None, None, Some("Yes".into()), None)
            };

        let mut meta_keys: Vec<String> = doc.header.meta.keys().cloned().collect();
        meta_keys.sort();

        let mut agent_hints = vec![
            "Payload plaintext is omitted unless explicitly requested with --reveal (Open only)."
                .into(),
            "CRC32C is corruption detection, not authenticity.".into(),
            format!("Use `dold validate` for structured checks ({PROTOCOL_NAME})."),
        ];
        match doc.header.mode {
            Mode::Protected => {
                agent_hints.push(
                    "Protected: decode refuses; `dold open` → AccessDenied until AEAD ships."
                        .into(),
                );
                if protected_stub == Some(true) {
                    agent_hints
                        .push("This object is a Protected stub — not real ciphertext.".into());
                }
            }
            Mode::Session => {
                agent_hints.push(
                    "Session seed in the manifest is a representation parameter, not a crypto key."
                        .into(),
                );
            }
            _ => {}
        }

        Self {
            schema: "doldskrift.inspect/1".into(),
            magic: String::from_utf8_lossy(MAGIC).into_owned(),
            protocol: format!("DSK/{}", doc.header.version),
            protocol_version: doc.header.version,
            glyph_engine: doc
                .header
                .meta
                .get("glyph_engine")
                .and_then(|v| v.as_u64())
                .unwrap_or(GLYPH_ENGINE_VERSION as u64) as u32,
            mode: doc.header.mode.to_string(),
            profile,
            payload_bytes: doc.payload.len(),
            characters: doc.header.char_count,
            byte_count: doc.header.byte_count,
            payload_kind,
            binary_payload: doc.is_binary_payload(),
            mapping,
            mapping_id: doc.header.mapping_id.clone(),
            density,
            checksum_valid: true,
            checksum: doc.header.checksum,
            checksum_algorithm: "CRC32C".into(),
            font_version: doc.header.font_version,
            flags: doc.header.flags.bits(),
            session_id: doc.header.session.as_ref().map(|s| s.session_id.clone()),
            capabilities,
            meta_keys,
            visual,
            cipher,
            session_requirement,
            agent_policy,
            readable,
            protected_stub,
            agent_hints,
        }
    }

    /// Pretty boxed CLI display.
    pub fn display(&self) -> String {
        if self.mode == "protected" {
            return self.display_protected();
        }
        let payload = format_bytes(self.payload_bytes);
        let chars = self
            .characters
            .map(|c| format!("{c}"))
            .unwrap_or_else(|| "n/a".into());
        let checksum = if self.checksum_valid {
            format!("✓ Valid ({:08x})", self.checksum)
        } else {
            format!("✗ INVALID ({:08x})", self.checksum)
        };
        let density = self.density.clone().unwrap_or_else(|| "normal".into());
        let mapping = &self.mapping;
        format!(
            "╭─ Doldskrift Document ─────────────────╮\n\
             │                                      │\n\
             │ Protocol        {protocol:<20} │\n\
             │ Glyph Engine    MGE/{engine:<17} │\n\
             │ Mode            {mode:<20} │\n\
             │ Profile         {profile:<20} │\n\
             │ Symbols         {chars:<20} │\n\
             │ Payload         {kind:<20} │\n\
             │ Size            {payload:<20} │\n\
             │ Mapping         {mapping:<20} │\n\
             │ Density         {density:<20} │\n\
             │ Checksum        {checksum:<20} │\n\
             │ Font Version    {font:<20} │\n\
             │                                      │\n\
             ╰──────────────────────────────────────╯",
            protocol = self.protocol,
            engine = self.glyph_engine,
            mode = title_case(&self.mode),
            profile = title_case(&self.profile),
            chars = chars,
            kind = self.payload_kind,
            payload = payload,
            mapping = mapping,
            density = title_case(&density),
            checksum = checksum,
            font = format!("{}.0", self.font_version),
        )
    }

    fn display_protected(&self) -> String {
        let integrity = if self.checksum_valid {
            format!("Valid ({:08x})", self.checksum)
        } else {
            format!("INVALID ({:08x})", self.checksum)
        };
        format!(
            "╭─ Doldskrift Protected Object ─────────╮\n\
             │                                      │\n\
             │ Protocol        {protocol:<20} │\n\
             │ Visual          {visual:<20} │\n\
             │ Payload         {payload:<20} │\n\
             │ Cipher          {cipher:<20} │\n\
             │ Session         {session:<20} │\n\
             │ Agent Policy    {policy:<20} │\n\
             │ Integrity       {integrity:<20} │\n\
             │ Readable        {readable:<20} │\n\
             │ Size            {size:<20} │\n\
             │                                      │\n\
             │ Stub: no AEAD yet. decode refuses;   │\n\
             │ open → AccessDenied without Gate.    │\n\
             ╰──────────────────────────────────────╯",
            protocol = self.protocol,
            visual = self.visual.as_deref().unwrap_or("MGE/5"),
            payload = self.payload_kind,
            cipher = self.cipher.as_deref().unwrap_or("AEAD-stub"),
            session = self.session_requirement.as_deref().unwrap_or("Required"),
            policy = self.agent_policy.as_deref().unwrap_or("Required"),
            integrity = integrity,
            readable = self.readable.as_deref().unwrap_or("No"),
            size = format_bytes(self.payload_bytes),
        )
    }

    /// JSON for tooling (`dold inspect --json`).
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Legacy plain text (tests / simple UIs).
    pub fn display_plain(&self) -> String {
        format!(
            "Doldskrift Document\nProtocol:        {} ({PROTOCOL_NAME})\nGlyph Engine:    MGE/{}\nMode:            {}\nPayload:         {}\nChecksum:        {}",
            self.protocol,
            self.glyph_engine,
            self.mode,
            format_bytes(self.payload_bytes),
            if self.checksum_valid {
                "valid"
            } else {
                "INVALID"
            }
        )
    }
}

fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn format_bytes(n: usize) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    if n >= MB as usize {
        format!("{:.1} MB", n as f64 / MB)
    } else if n >= 1024 {
        format!("{:.1} KB", n as f64 / KB)
    } else {
        format!("{n} B")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Mode;

    #[test]
    fn inspect_json_has_agent_schema() {
        let doc = DskDocument::encode_text("agent", Mode::Encoded).unwrap();
        let report = doc.inspect();
        assert_eq!(report.schema, "doldskrift.inspect/1");
        assert_eq!(report.profile, "open");
        assert_eq!(report.magic, "DSK1");
        let json = report.to_json().unwrap();
        assert!(json.contains("agent_hints"));
        assert!(json.contains("meta_keys"));
    }
}
