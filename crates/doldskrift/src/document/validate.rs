//! Structural validation for `.dsk` documents (`dold validate`).
//!
//! Checks parseability, checksum integrity, mode/version consistency, and
//! common metadata expectations. Does **not** claim cryptographic authenticity.

use super::DskDocument;
use crate::{Mode, MAGIC, PROTOCOL_VERSION, PROTOCOL_VERSION_PROTECTED};
use serde::Serialize;

/// One validation check result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidateCheck {
    /// Stable machine id (e.g. `magic`, `checksum`).
    pub id: String,
    /// Human-readable description.
    pub detail: String,
    /// Whether this check passed.
    pub ok: bool,
    /// Severity when `ok` is false (`error` or `warn`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

/// Machine-readable validation report (`doldskrift.validate/1`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidateReport {
    /// Report schema id.
    pub schema: String,
    /// Overall pass (no error-severity failures).
    pub ok: bool,
    /// Open / neural / protected profile label.
    pub profile: String,
    /// Protocol label (`DSK/1` …).
    pub protocol: String,
    /// Mode name.
    pub mode: String,
    /// Individual checks.
    pub checks: Vec<ValidateCheck>,
    /// Non-fatal notes for agents.
    pub notes: Vec<String>,
}

impl ValidateReport {
    /// Pretty JSON for CLI / agents.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Human summary lines.
    pub fn display(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "dold validate — {} ({})\n",
            if self.ok { "OK" } else { "FAILED" },
            self.profile
        ));
        out.push_str(&format!("  protocol: {}\n", self.protocol));
        out.push_str(&format!("  mode:     {}\n", self.mode));
        for c in &self.checks {
            let mark = if c.ok { "✓" } else { "✗" };
            out.push_str(&format!("  {mark} {}: {}\n", c.id, c.detail));
        }
        for n in &self.notes {
            out.push_str(&format!("  note: {n}\n"));
        }
        out
    }
}

impl DskDocument {
    /// Validate structure and common metadata expectations.
    ///
    /// Prefer this for agent/CI gates. [`Self::verify`] remains a lighter
    /// re-serialize sanity check used by `dold verify`.
    pub fn validate(&self) -> ValidateReport {
        let mut checks = Vec::new();
        let mut notes = Vec::new();

        checks.push(ValidateCheck {
            id: "magic".into(),
            detail: format!("container magic is {}", String::from_utf8_lossy(MAGIC)),
            ok: true,
            severity: None,
        });

        let version_ok = match self.header.mode {
            Mode::Protected => self.header.version == PROTOCOL_VERSION_PROTECTED,
            _ => self.header.version == PROTOCOL_VERSION,
        };
        checks.push(ValidateCheck {
            id: "version_mode".into(),
            detail: if version_ok {
                format!(
                    "version {} matches mode {}",
                    self.header.version, self.header.mode
                )
            } else {
                format!(
                    "version {} unexpected for mode {}",
                    self.header.version, self.header.mode
                )
            },
            ok: version_ok,
            severity: if version_ok {
                None
            } else {
                Some("error".into())
            },
        });

        let checksum_ok = self.verify().is_ok();
        checks.push(ValidateCheck {
            id: "checksum".into(),
            detail: if checksum_ok {
                format!("CRC32C {:08x} consistent", self.header.checksum)
            } else {
                "checksum / re-serialize mismatch".into()
            },
            ok: checksum_ok,
            severity: if checksum_ok {
                None
            } else {
                Some("error".into())
            },
        });

        let has_payload_type = self.header.meta.contains_key("payload_type");
        checks.push(ValidateCheck {
            id: "payload_type_meta".into(),
            detail: if has_payload_type {
                format!(
                    "payload_type={}",
                    self.header
                        .meta
                        .get("payload_type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("?")
                )
            } else {
                "missing meta.payload_type (allowed but unusual)".into()
            },
            ok: true,
            severity: if has_payload_type {
                None
            } else {
                Some("warn".into())
            },
        });
        if !has_payload_type {
            notes.push("meta.payload_type absent — older or hand-built container".into());
        }

        match self.header.mode {
            Mode::Session => {
                let has_session = self.header.session.is_some();
                checks.push(ValidateCheck {
                    id: "session_manifest".into(),
                    detail: if has_session {
                        "session manifest present".into()
                    } else {
                        "session mode missing manifest".into()
                    },
                    ok: has_session,
                    severity: if has_session {
                        None
                    } else {
                        Some("error".into())
                    },
                });
            }
            Mode::Protected => {
                let stub = self
                    .header
                    .meta
                    .get("cipher")
                    .and_then(|v| v.as_str())
                    .unwrap_or("AEAD-stub")
                    == "AEAD-stub"
                    || self.payload.starts_with(b"PROTECTED-STUB");
                checks.push(ValidateCheck {
                    id: "protected_refuse".into(),
                    detail: "Protected decode refuses plaintext (expected)".into(),
                    ok: self.decode_text().is_err(),
                    severity: Some("error".into()),
                });
                if stub {
                    notes.push(
                        "Protected stub: cipher meta / payload is not real AEAD ciphertext".into(),
                    );
                }
                notes.push("Readable: No — use Gate/`dold open` when AEAD ships".into());
            }
            Mode::Encoded | Mode::Visual => {
                if !self.is_binary_payload() {
                    match self.decode_text() {
                        Ok(_) => checks.push(ValidateCheck {
                            id: "decode_roundtrip".into(),
                            detail: "semantic decode succeeded".into(),
                            ok: true,
                            severity: None,
                        }),
                        Err(e) => checks.push(ValidateCheck {
                            id: "decode_roundtrip".into(),
                            detail: format!("semantic decode failed: {e}"),
                            ok: false,
                            severity: Some("error".into()),
                        }),
                    }
                } else {
                    notes.push("BINARY payload — semantic decode skipped".into());
                }
            }
        }

        // Fix protected_refuse severity when ok
        if let Some(c) = checks.iter_mut().find(|c| c.id == "protected_refuse") {
            if c.ok {
                c.severity = None;
            }
        }

        let profile = match self.header.mode {
            Mode::Protected => "protected",
            _ => "open",
        }
        .to_string();

        let ok = checks
            .iter()
            .filter(|c| c.severity.as_deref() == Some("error"))
            .all(|c| c.ok);

        ValidateReport {
            schema: "doldskrift.validate/1".into(),
            ok,
            profile,
            protocol: format!("DSK/{}", self.header.version),
            mode: self.header.mode.to_string(),
            checks,
            notes,
        }
    }

    /// Re-serialize to canonical `.dsk` bytes (normalize header JSON + checksum).
    ///
    /// Used by `dold fmt`. Does not alter payload semantics.
    pub fn format_canonical(&self) -> crate::Result<Vec<u8>> {
        // Recompute via from_parts so checksum and version stamps stay consistent.
        let mut header = self.header.clone();
        header.checksum = 0;
        Self::from_parts(header, self.payload.clone())?.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Mode;

    #[test]
    fn validate_encoded_ok() {
        let doc = DskDocument::encode_text("validate me", Mode::Encoded).unwrap();
        let report = doc.validate();
        assert!(report.ok, "{}", report.display());
        assert_eq!(report.profile, "open");
    }

    #[test]
    fn validate_protected_stub() {
        let doc = DskDocument::encode_protected_stub("x").unwrap();
        let report = doc.validate();
        assert!(report.ok, "{}", report.display());
        assert_eq!(report.profile, "protected");
        assert!(report.notes.iter().any(|n| n.contains("stub")));
    }

    #[test]
    fn format_canonical_stable() {
        let doc = DskDocument::encode_text("fmt", Mode::Encoded).unwrap();
        let a = doc.format_canonical().unwrap();
        let again = DskDocument::parse(&a).unwrap();
        let b = again.format_canonical().unwrap();
        assert_eq!(a, b);
        assert_eq!(again.decode_text().unwrap(), "fmt");
    }
}
