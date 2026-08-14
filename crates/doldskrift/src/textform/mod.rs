//! Text-form helpers (plain / encoded / armored).

use crate::armor::{armor, dearmor};
use crate::codec::{decode, encode};
use crate::{Error, Mode, Result};

/// Detect a likely text form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextForm {
    /// Ordinary UTF-8.
    Plain,
    /// PUA-encoded DOLDSKRIFT/1.
    Encoded,
    /// Base64 armored binary.
    Armored,
}

/// Sniff the form of `text`.
pub fn detect(text: &str) -> TextForm {
    if text.contains("BEGIN DOLDSKRIFT") {
        TextForm::Armored
    } else if text.chars().any(crate::is_symbol) {
        TextForm::Encoded
    } else {
        TextForm::Plain
    }
}

/// Encode text for a mode into a text form string.
pub fn encode_form(text: &str, mode: Mode) -> Result<String> {
    match mode {
        Mode::Visual => Ok(text.to_owned()),
        Mode::Encoded => encode(text),
        Mode::Session => Err(Error::InvalidMapping(
            "session mode needs Session::encode".into(),
        )),
        Mode::Protected => Err(Error::AccessDenied(
            "Protected mode has no Open plaintext text-form encode".into(),
        )),
    }
}

/// Decode a text form back to semantic UTF-8 when possible.
pub fn decode_form(text: &str) -> Result<String> {
    match detect(text) {
        TextForm::Plain => Ok(text.to_owned()),
        TextForm::Encoded => decode(text),
        TextForm::Armored => {
            let bytes = dearmor(text)?;
            String::from_utf8(bytes).map_err(Error::from)
        }
    }
}

/// Armor arbitrary bytes as text.
pub fn armor_bytes(bytes: &[u8]) -> String {
    armor(bytes)
}
