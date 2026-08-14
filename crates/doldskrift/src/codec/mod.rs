//! Encoding, decoding, and mappings for DOLDSKRIFT/1 (and helpers for DSK/2).

mod encoder;
mod mapping;

pub use encoder::{decode, encode, Decoder, Encoder};
pub use mapping::{identity_symbol, Mapping, MappingId, StaticMapping};

use crate::{is_symbol, Error, Mode, Result};

/// Encode `text` using the given mode.
pub fn encode_with_mode(text: &str, mode: Mode) -> Result<String> {
    match mode {
        Mode::Visual => Ok(text.to_owned()),
        Mode::Encoded => encode(text),
        Mode::Session => Err(Error::InvalidMapping(
            "session mode requires Session::encode (seeded mapping)".into(),
        )),
        Mode::Protected => Err(Error::AccessDenied(
            "Protected mode has no Open plaintext encode path; use DskDocument::encode_protected_stub"
                .into(),
        )),
    }
}

/// Decode `text`, auto-detecting encoded PUA content when possible.
pub fn decode_auto(text: &str) -> Result<String> {
    if text.chars().all(|c| c.is_ascii() || !is_symbol(c)) && !text.chars().any(is_symbol) {
        return Ok(text.to_owned());
    }
    decode(text)
}
