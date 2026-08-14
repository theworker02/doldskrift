//! `.dsk` container format for DOLDSKRIFT/1.
//!
//! Use [`DskDocument::inspect`] for agent metadata (`doldskrift.inspect/1`),
//! [`DskDocument::validate`] for CI gates (`doldskrift.validate/1`), and
//! [`DskDocument::format_canonical`] for `dold fmt`.
//!
//! Binary layout (big-endian integers):
//! ```text
//! Offset  Size  Field
//! 0       4     Magic "DSK1"
//! 4       1     Version
//! 5       1     Mode
//! 6       1     Flags
//! 7       1     Reserved (0)
//! 8       4     Header length (N)
//! 12      4     Payload length (M)
//! 16      4     CRC32C(header || payload)
//! 20      N     Header JSON (UTF-8)
//! 20+N    M     Payload bytes
//! ```
//!
//! Open and Neural modes are representation — not encryption. Protected mode
//! refuses plaintext decode until real AEAD ships.

mod header;
mod inspect;
mod validate;

pub use header::{DocumentFlags, DocumentHeader};
pub use inspect::InspectReport;
pub use validate::{ValidateCheck, ValidateReport};

use crate::codec::{decode, encode, Mapping};
use crate::session::Session;
use crate::{
    checksum_bytes, verify_checksum, Error, Mode, ProtocolVersion, Result, GLYPH_ENGINE_VERSION,
    MAGIC, PROTOCOL_VERSION, PROTOCOL_VERSION_PROTECTED,
};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const FIXED_HEADER_LEN: usize = 20;

/// An in-memory Doldskrift document (`.dsk`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DskDocument {
    /// Document header metadata.
    pub header: DocumentHeader,
    /// Raw payload bytes (encoded representation for encoded/session modes,
    /// or UTF-8 semantic text for visual mode).
    pub payload: Vec<u8>,
}

impl DskDocument {
    /// Build a document by encoding `text` in `mode`.
    pub fn encode_text(text: &str, mode: Mode) -> Result<Self> {
        match mode {
            Mode::Visual => {
                let mut header = DocumentHeader::new(mode);
                stamp_meta(&mut header, "UTF-8", None);
                Self::from_parts(header, text.as_bytes().to_vec())
            }
            Mode::Encoded => {
                let encoded = encode(text)?;
                let mut header = DocumentHeader::new(mode);
                header.char_count = Some(text.chars().count() as u64);
                header.byte_count = Some(text.len() as u64);
                stamp_meta(&mut header, "UTF-8", Some("normal"));
                Self::from_parts(header, encoded.into_bytes())
            }
            Mode::Session => Err(Error::InvalidMapping(
                "use DskDocument::encode_session for session mode".into(),
            )),
            Mode::Protected => Err(Error::InvalidMapping(
                "use DskDocument::encode_protected_stub for Protected mode (no plaintext encode via Open path)"
                    .into(),
            )),
        }
    }

    /// Build a **stub** Protected container (ADR-0006 milestone 1).
    ///
    /// Payload is an opaque placeholder — **not** real AEAD ciphertext.
    /// Semantic decode and Gate open refuse until crypto ships.
    pub fn encode_protected_stub(label: &str) -> Result<Self> {
        let mut header = DocumentHeader::new(Mode::Protected);
        crate::protected::stamp_protected_meta(&mut header);
        header.byte_count = Some(label.len() as u64);
        // Opaque placeholder bytes (not ciphertext). Prefix keeps inspect honest.
        let payload = format!("PROTECTED-STUB\0{label}").into_bytes();
        Self::from_parts(header, payload)
    }

    /// True when this document is a Protected-mode object.
    pub fn is_protected(&self) -> bool {
        self.header.mode == Mode::Protected
    }

    /// Build a session-mode document.
    pub fn encode_session(text: &str, session: &Session) -> Result<Self> {
        let encoded = session.encode(text)?;
        let mut header = DocumentHeader::new(Mode::Session);
        header.char_count = Some(text.chars().count() as u64);
        header.byte_count = Some(text.len() as u64);
        header.session = Some(session.manifest().clone());
        header.mapping_id = Some(session.mapping_id().to_string());
        stamp_meta(&mut header, "UTF-8", Some("normal"));
        Self::from_parts(header, encoded.into_bytes())
    }

    /// Pack arbitrary binary bytes into a `.dsk` container (visual mode, BINARY payload).
    ///
    /// Binary payloads are not glyph-rendered as text unless explicitly requested.
    pub fn pack_binary(bytes: &[u8]) -> Result<Self> {
        let mut header = DocumentHeader::new(Mode::Visual);
        header.byte_count = Some(bytes.len() as u64);
        stamp_meta(&mut header, "BINARY", None);
        Self::from_parts(header, bytes.to_vec())
    }

    /// Unpack raw payload bytes (no decode).
    pub fn unpack_bytes(&self) -> &[u8] {
        &self.payload
    }

    /// True when header marks payload as binary.
    pub fn is_binary_payload(&self) -> bool {
        self.header
            .meta
            .get("payload_type")
            .and_then(|v| v.as_str())
            == Some("BINARY")
    }

    /// Construct from header + payload (computes checksum).
    pub fn from_parts(mut header: DocumentHeader, payload: Vec<u8>) -> Result<Self> {
        if header.mode == Mode::Protected {
            header.version = PROTOCOL_VERSION_PROTECTED;
        } else {
            header.version = PROTOCOL_VERSION;
        }
        let header_bytes = header.to_bytes()?;
        let mut covered = Vec::with_capacity(header_bytes.len() + payload.len());
        covered.extend_from_slice(&header_bytes);
        covered.extend_from_slice(&payload);
        header.checksum = checksum_bytes(&covered);
        Ok(Self { header, payload })
    }

    /// Open and parse a `.dsk` file from disk.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Self::parse(&bytes)
    }

    /// Parse a `.dsk` blob.
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < FIXED_HEADER_LEN {
            return Err(Error::TruncatedPayload {
                needed: FIXED_HEADER_LEN - bytes.len(),
            });
        }
        if &bytes[0..4] != MAGIC.as_slice() {
            return Err(Error::InvalidHeader);
        }
        let version = bytes[4];
        ProtocolVersion(version).check_supported()?;
        let mode = Mode::from_u8(bytes[5]).ok_or(Error::UnsupportedMode(bytes[5]))?;
        let flags = DocumentFlags::from_bits(bytes[6]);
        let _reserved = bytes[7];
        let header_len = u32::from_be_bytes(bytes[8..12].try_into().unwrap()) as usize;
        let payload_len = u32::from_be_bytes(bytes[12..16].try_into().unwrap()) as usize;
        let expected_checksum = u32::from_be_bytes(bytes[16..20].try_into().unwrap());

        let body_start = FIXED_HEADER_LEN;
        let header_end = body_start
            .checked_add(header_len)
            .ok_or_else(|| Error::InvalidLength("header length overflow".into()))?;
        let payload_end = header_end
            .checked_add(payload_len)
            .ok_or_else(|| Error::InvalidLength("payload length overflow".into()))?;
        if bytes.len() < payload_end {
            return Err(Error::TruncatedPayload {
                needed: payload_end - bytes.len(),
            });
        }
        if bytes.len() > payload_end {
            // Trailing junk is rejected for strictness.
            return Err(Error::InvalidLength(format!(
                "trailing {} bytes after payload",
                bytes.len() - payload_end
            )));
        }

        let header_bytes = &bytes[body_start..header_end];
        let payload = bytes[header_end..payload_end].to_vec();
        let mut covered = Vec::with_capacity(header_bytes.len() + payload.len());
        covered.extend_from_slice(header_bytes);
        covered.extend_from_slice(&payload);
        verify_checksum(&covered, expected_checksum)?;

        let mut header = if header_bytes.is_empty() {
            DocumentHeader::new(mode)
        } else {
            DocumentHeader::from_bytes(header_bytes)?
        };
        header.version = version;
        header.mode = mode;
        header.flags = flags;
        header.checksum = expected_checksum;

        Ok(Self { header, payload })
    }

    /// Serialize to `.dsk` bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let header_bytes = self.header.to_bytes()?;
        let mut covered = Vec::with_capacity(header_bytes.len() + self.payload.len());
        covered.extend_from_slice(&header_bytes);
        covered.extend_from_slice(&self.payload);
        let checksum = checksum_bytes(&covered);

        let mut out = Vec::with_capacity(FIXED_HEADER_LEN + covered.len());
        out.extend_from_slice(MAGIC);
        out.push(self.header.version);
        out.push(self.header.mode.as_u8());
        out.push(self.header.flags.bits());
        out.push(0); // reserved
        out.extend_from_slice(&(header_bytes.len() as u32).to_be_bytes());
        out.extend_from_slice(&(self.payload.len() as u32).to_be_bytes());
        out.extend_from_slice(&checksum.to_be_bytes());
        out.extend_from_slice(&header_bytes);
        out.extend_from_slice(&self.payload);
        Ok(out)
    }

    /// Write to a file path.
    pub fn write(&self, path: impl AsRef<Path>) -> Result<()> {
        let bytes = self.to_bytes()?;
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;
        Ok(())
    }

    /// Decode semantic text from the document.
    pub fn decode_text(&self) -> Result<String> {
        match self.header.mode {
            Mode::Visual => String::from_utf8(self.payload.clone()).map_err(Error::from),
            Mode::Encoded => {
                let encoded =
                    std::str::from_utf8(&self.payload).map_err(|e| Error::InvalidUtf8 {
                        offset: e.valid_up_to(),
                    })?;
                decode(encoded)
            }
            Mode::Session => {
                let encoded =
                    std::str::from_utf8(&self.payload).map_err(|e| Error::InvalidUtf8 {
                        offset: e.valid_up_to(),
                    })?;
                let session = self.recover_session()?;
                session.decode(encoded)
            }
            Mode::Protected => Err(Error::AccessDenied(
                "Protected payload: semantic decode refused. Glyphs encode ciphertext (stub placeholder today), not plaintext. Use `dold inspect` for metadata or `dold open` with a Gate capability (AEAD not implemented — AccessDenied)."
                    .into(),
            )),
        }
    }

    /// Recover a [`Session`] from header metadata when possible.
    pub fn recover_session(&self) -> Result<Session> {
        let manifest = self
            .header
            .session
            .as_ref()
            .ok_or_else(|| Error::InvalidManifest("missing session manifest".into()))?;
        let seed_hex = manifest
            .seed_hex
            .as_ref()
            .ok_or_else(|| Error::InvalidManifest("missing seed_hex in manifest".into()))?;
        let seed = hex::decode(seed_hex)
            .map_err(|e| Error::InvalidManifest(format!("bad seed_hex: {e}")))?;
        Session::from_seed(seed, manifest.session_id.clone())
    }

    /// Verify checksum and structural integrity (already verified on parse).
    pub fn verify(&self) -> Result<()> {
        let bytes = self.to_bytes()?;
        let again = Self::parse(&bytes)?;
        if again.header.checksum != self.header.checksum && again.payload != self.payload {
            return Err(Error::Other("re-serialize mismatch".into()));
        }
        Ok(())
    }

    /// Produce an inspect report **without** decoding the payload.
    pub fn inspect(&self) -> InspectReport {
        InspectReport::from_document(self)
    }

    /// Payload length in bytes.
    pub fn payload_len(&self) -> usize {
        self.payload.len()
    }
}

fn stamp_meta(header: &mut DocumentHeader, payload_type: &str, density: Option<&str>) {
    header.meta.insert(
        "glyph_engine".into(),
        serde_json::json!(GLYPH_ENGINE_VERSION),
    );
    header
        .meta
        .insert("payload_type".into(), serde_json::json!(payload_type));
    header.meta.insert("dsk".into(), serde_json::json!(1));
    if let Some(d) = density {
        header.meta.insert("density".into(), serde_json::json!(d));
    }
}

/// Re-export mapping helper for callers assembling custom docs.
pub fn identity_mapping() -> Mapping {
    Mapping::identity()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_file_roundtrip() {
        let doc = DskDocument::encode_text("Hello from Doldskrift", Mode::Encoded).unwrap();
        let bytes = doc.to_bytes().unwrap();
        let parsed = DskDocument::parse(&bytes).unwrap();
        assert_eq!(parsed.decode_text().unwrap(), "Hello from Doldskrift");
        assert!(parsed.inspect().checksum_valid);
    }

    #[test]
    fn session_document() {
        let session = Session::builder()
            .seed(b"alpha")
            .session_id("001")
            .build()
            .unwrap();
        let doc = DskDocument::encode_session("ping", &session).unwrap();
        let parsed = DskDocument::parse(&doc.to_bytes().unwrap()).unwrap();
        assert_eq!(parsed.decode_text().unwrap(), "ping");
    }

    #[test]
    fn rejects_bad_magic() {
        let err = DskDocument::parse(b"XXXX................").unwrap_err();
        assert_eq!(err, Error::InvalidHeader);
    }

    #[test]
    fn protected_stub_roundtrip_bytes_refuses_plaintext() {
        let doc = DskDocument::encode_protected_stub("task-42").unwrap();
        let parsed = DskDocument::parse(&doc.to_bytes().unwrap()).unwrap();
        assert!(parsed.is_protected());
        assert_eq!(parsed.header.version, PROTOCOL_VERSION_PROTECTED);
        assert!(matches!(parsed.decode_text(), Err(Error::AccessDenied(_))));
        let report = parsed.inspect();
        assert_eq!(report.readable.as_deref(), Some("No"));
        assert_eq!(report.payload_kind, "Protected");
    }
}
