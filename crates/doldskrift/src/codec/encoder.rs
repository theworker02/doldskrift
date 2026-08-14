//! High-level Encoder / Decoder API.

use super::mapping::Mapping;
use crate::{Mode, Result};

/// Encode `text` with the fixed DOLDSKRIFT/1 identity mapping.
pub fn encode(text: &str) -> Result<String> {
    Ok(Mapping::identity().encode_str(text))
}

/// Decode PUA-encoded `text` with the fixed DOLDSKRIFT/1 identity mapping.
pub fn decode(encoded: &str) -> Result<String> {
    Mapping::identity().decode_str(encoded)
}

/// Idiomatic encoder bound to a mode (and optional mapping).
#[derive(Debug, Clone)]
pub struct Encoder {
    mode: Mode,
    mapping: Mapping,
}

impl Encoder {
    /// Create an encoder for `mode`.
    ///
    /// For [`Mode::Session`], prefer [`crate::Session::builder`].
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            mapping: Mapping::identity(),
        }
    }

    /// Create an encoder with an explicit mapping (session or custom).
    pub fn with_mapping(mode: Mode, mapping: Mapping) -> Self {
        Self { mode, mapping }
    }

    /// Operating mode.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Encode semantic text to the representation for this mode.
    pub fn encode(&self, text: &str) -> Result<String> {
        match self.mode {
            Mode::Visual => Ok(text.to_owned()),
            Mode::Encoded | Mode::Session => Ok(self.mapping.encode_str(text)),
            Mode::Protected => Err(crate::Error::AccessDenied(
                "Protected mode: use DskDocument::encode_protected_stub (no Open Encoder path)"
                    .into(),
            )),
        }
    }
}

/// Idiomatic decoder.
#[derive(Debug, Clone)]
pub struct Decoder {
    mapping: Mapping,
}

impl Decoder {
    /// Decoder using the fixed encoded-mode mapping.
    pub fn new() -> Self {
        Self {
            mapping: Mapping::identity(),
        }
    }

    /// Decoder using a session / custom mapping.
    pub fn with_mapping(mapping: Mapping) -> Self {
        Self { mapping }
    }

    /// Decode representation text back to semantic UTF-8.
    ///
    /// If the input contains no PUA symbols, it is returned unchanged
    /// (visual-mode passthrough).
    pub fn decode(&self, payload: &str) -> Result<String> {
        if !payload.chars().any(crate::is_symbol) {
            return Ok(payload.to_owned());
        }
        self.mapping.decode_str(payload)
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_roundtrip() {
        let encoder = Encoder::new(Mode::Encoded);
        let payload = encoder.encode("Hello, agent!").unwrap();
        let decoder = Decoder::new();
        let message = decoder.decode(&payload).unwrap();
        assert_eq!(message, "Hello, agent!");
    }

    #[test]
    fn visual_passthrough() {
        let encoder = Encoder::new(Mode::Visual);
        assert_eq!(encoder.encode("DEPLOY READY").unwrap(), "DEPLOY READY");
    }
}
