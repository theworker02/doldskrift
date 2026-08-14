//! Errors, modes, versions, checksums, and symbol constants (canonical core).

mod checksum;
mod mode;
mod symbols;
mod version;

pub use checksum::{checksum_bytes, verify_checksum, CHECKSUM_ALGORITHM};
pub use mode::{Capability, Mode};
pub use symbols::{
    is_control_symbol, is_project_mark, symbol_id_of, ControlSymbol, CONTROL_BASE,
    DSK_PROJECT_MARK, DSK_PROJECT_MARK_NAME, FONT_VERSION_MGE2, GLYPH_ENGINE_VERSION,
};
pub use version::{ProtocolVersion, PROTOCOL_NAME, PROTOCOL_VERSION, PROTOCOL_VERSION_PROTECTED};

use thiserror::Error;

/// Convenient result alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Comprehensive Doldskrift error type.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    /// Container or frame magic bytes were not recognized.
    #[error("invalid header: expected DSK1 magic")]
    InvalidHeader,
    /// Protocol version is not supported by this implementation.
    #[error("unsupported protocol version: {0}")]
    UnsupportedVersion(u8),
    /// Mode byte or mode name is not recognized.
    #[error("unsupported mode: {0}")]
    UnsupportedMode(u8),
    /// A codepoint fell outside the valid symbol alphabet.
    #[error("invalid codepoint: U+{0:04X}")]
    InvalidCodepoint(u32),
    /// Session or static mapping is inconsistent or unknown.
    #[error("invalid mapping: {0}")]
    InvalidMapping(String),
    /// Two distinct inputs produced colliding glyphs or mapping entries.
    #[error("mapping collision: {0}")]
    MappingCollision(String),
    /// CRC32C (or other) checksum did not match.
    #[error("checksum mismatch: expected {expected:#010x}, got {actual:#010x}")]
    ChecksumMismatch {
        /// Expected checksum.
        expected: u32,
        /// Computed checksum.
        actual: u32,
    },
    /// Input ended before a complete frame/document was available.
    #[error("truncated payload: needed {needed} more bytes")]
    TruncatedPayload {
        /// Additional bytes required.
        needed: usize,
    },
    /// Decoded bytes were not valid UTF-8.
    #[error("invalid UTF-8 at byte offset {offset}")]
    InvalidUtf8 {
        /// Byte offset of the failure.
        offset: usize,
    },
    /// Session or document manifest could not be parsed.
    #[error("invalid manifest: {0}")]
    InvalidManifest(String),
    /// Font / glyph generation failed.
    #[error("font generation error: {0}")]
    FontGenerationError(String),
    /// I/O failure (stringified for Clone + Eq friendliness).
    #[error("I/O error: {0}")]
    IoError(String),
    /// Frame length fields were inconsistent or exceeded limits.
    #[error("invalid length: {0}")]
    InvalidLength(String),
    /// Capability negotiation could not select a mutually supported mode.
    #[error("negotiation failed: {0}")]
    NegotiationFailed(String),
    /// Vision / OCR pipeline failure (experimental).
    #[error("vision error: {0}")]
    VisionError(String),
    /// Semantic / schema error (DSK/2).
    #[error("semantic error: {0}")]
    Semantic(String),
    /// Query path error.
    #[error("query error: {0}")]
    Query(String),
    /// Visual frame error.
    #[error("frame error: {0}")]
    Frame(String),
    /// ECC recovery failed.
    #[error("ecc error: {0}")]
    Ecc(String),
    /// Resource limit exceeded.
    #[error("resource limit exceeded: {0}")]
    ResourceLimit(String),
    /// Unsupported optional extension.
    #[error("unsupported extension: {0}")]
    UnsupportedExtension(String),
    /// JSON / structural parse failure.
    #[error("parse error: {0}")]
    Parse(String),
    /// Protected / Gate access denied (no plaintext path).
    #[error("access denied: {0}")]
    AccessDenied(String),
    /// Generic validation failure.
    #[error("{0}")]
    Other(String),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::InvalidUtf8 {
            offset: value.utf8_error().valid_up_to(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Parse(value.to_string())
    }
}

/// Magic bytes identifying a DSK/1 binary container: `DSK1`.
pub const MAGIC: &[u8; 4] = b"DSK1";
/// Magic for experimental DSK/2 containers: `DSK2`.
pub const MAGIC_DSK2: &[u8; 4] = b"DSK2";
/// ASCII discovery token requesting capability advertisement.
pub const HANDSHAKE_QUERY: &str = "DSK?";
/// ASCII discovery token answering a capability advertisement.
pub const HANDSHAKE_REPLY: &str = "DSK!";
/// Base of the DOLDSKRIFT/1 Private Use Area alphabet (256 symbols).
pub const PUA_BASE: u32 = 0xE000;
/// Number of symbols in the byte alphabet.
pub const ALPHABET_SIZE: usize = 256;
/// Font / glyph grammar version used by the procedural generator (MGE/2 baseline).
pub const FONT_VERSION: u32 = FONT_VERSION_MGE2;
/// MIME type for binary `.dsk` containers.
pub const MIME_BINARY: &str = "application/vnd.doldskrift";
/// MIME type for textual Doldskrift representations.
pub const MIME_TEXT: &str = "text/doldskrift";

/// Convert a UTF-8 byte to its encoded PUA codepoint.
#[inline]
pub fn byte_to_symbol(byte: u8) -> char {
    char::from_u32(PUA_BASE + u32::from(byte)).expect("PUA symbol must be valid")
}

/// Convert an encoded PUA codepoint back to a UTF-8 byte.
#[inline]
pub fn symbol_to_byte(ch: char) -> Option<u8> {
    let cp = ch as u32;
    if (PUA_BASE..PUA_BASE + ALPHABET_SIZE as u32).contains(&cp) {
        Some((cp - PUA_BASE) as u8)
    } else {
        None
    }
}

/// Returns true if `ch` is a DOLDSKRIFT/1 encoded alphabet symbol.
#[inline]
pub fn is_symbol(ch: char) -> bool {
    symbol_to_byte(ch).is_some()
}
