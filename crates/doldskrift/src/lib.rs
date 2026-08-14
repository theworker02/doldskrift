//! # Doldskrift
//!
//! Machine-native typography and representation for software agents.
//!
//! Doldskrift is **not encryption**. It is a deterministic encoding and
//! typography system that can make text visually opaque to humans while
//! remaining exactly recoverable by Doldskrift-aware software.
//!
//! ```rust
//! use doldskrift::{Decoder, Encoder, Mode};
//!
//! let encoder = Encoder::new(Mode::Encoded);
//! let payload = encoder.encode("Hello, agent!").unwrap();
//! let decoder = Decoder::new();
//! let message = decoder.decode(&payload).unwrap();
//! assert_eq!(message, "Hello, agent!");
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod address;
pub mod alphabet;
pub mod armor;
pub mod canonical;
pub mod capture;
pub mod codec;
pub mod compatibility;
pub mod document;
pub mod ecc;
pub mod errors;
pub mod frame;
pub mod glyph;
pub mod limits;
pub mod neural;
pub mod protected;
pub mod protocol;
pub mod query;
pub mod registry;
pub mod runtime;
pub mod schema;
pub mod semantic;
pub mod session;
pub mod stream;
pub mod textform;
pub mod transport;

#[cfg(feature = "ffi")]
#[allow(unsafe_code)]
pub mod ffi;

pub use errors::{
    byte_to_symbol, checksum_bytes, is_control_symbol, is_project_mark, is_symbol, symbol_id_of,
    symbol_to_byte, verify_checksum, Capability, ControlSymbol, Error, Mode, ProtocolVersion,
    Result, ALPHABET_SIZE, CHECKSUM_ALGORITHM, CONTROL_BASE, DSK_PROJECT_MARK,
    DSK_PROJECT_MARK_NAME, FONT_VERSION, FONT_VERSION_MGE2, GLYPH_ENGINE_VERSION, HANDSHAKE_QUERY,
    HANDSHAKE_REPLY, MAGIC, MAGIC_DSK2, MIME_BINARY, MIME_TEXT, PROTOCOL_NAME, PROTOCOL_VERSION,
    PROTOCOL_VERSION_PROTECTED, PUA_BASE,
};

pub use protected::{gate_open, ProtectedInspect};

pub use codec::{
    decode, decode_auto, encode, encode_with_mode, identity_symbol, Decoder, Encoder, Mapping,
    MappingId, StaticMapping,
};
pub use document::{
    DocumentFlags, DocumentHeader, DskDocument, InspectReport, ValidateCheck, ValidateReport,
};
pub use protocol::{
    detect_from_attrs, detect_from_html_meta, detect_pua_heuristic, negotiate, CapabilityProfile,
    CapabilitySet, Detection, DetectionSource, HandshakeOffer, HandshakeReply,
    DEFAULT_CAPABILITIES,
};
pub use session::{Session, SessionBuilder, SessionManifest};
pub use stream::{StreamingDecoder, StreamingEncoder};
