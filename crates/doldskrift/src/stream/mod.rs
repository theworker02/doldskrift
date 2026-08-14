//! Incremental / streaming encode and decode.

use crate::codec::Mapping;
use crate::{Error, Result};

/// Streaming encoder: push UTF-8 chunks, pull encoded PUA text.
#[derive(Debug, Clone)]
pub struct StreamingEncoder {
    mapping: Mapping,
    /// Incomplete UTF-8 sequence carried across `push` calls.
    pending: Vec<u8>,
}

impl StreamingEncoder {
    /// Create a streaming encoder with the identity mapping.
    pub fn new() -> Self {
        Self::with_mapping(Mapping::identity())
    }

    /// Create a streaming encoder with a custom mapping.
    pub fn with_mapping(mapping: Mapping) -> Self {
        Self {
            mapping,
            pending: Vec::new(),
        }
    }

    /// Push a chunk of UTF-8 bytes (may be mid-character).
    ///
    /// Returns the encoded symbols for all complete bytes consumed.
    pub fn push(&mut self, chunk: &[u8]) -> String {
        self.pending.extend_from_slice(chunk);
        let mut out = String::new();
        // Encode all bytes we have; UTF-8 validity is checked at finish /
        // by the application. Streaming encode is byte-oriented by design.
        for &b in &self.pending {
            out.push(self.mapping.encode_byte(b));
        }
        self.pending.clear();
        out
    }

    /// Push a UTF-8 string chunk.
    pub fn push_str(&mut self, chunk: &str) -> String {
        self.push(chunk.as_bytes())
    }

    /// Finish the stream. Returns an error if pending incomplete data remains
    /// (should not happen when only `push_str` is used).
    pub fn finish(self) -> Result<String> {
        if self.pending.is_empty() {
            Ok(String::new())
        } else {
            // Encode remaining raw bytes (caller supplied raw bytes).
            let mut out = String::new();
            for &b in &self.pending {
                out.push(self.mapping.encode_byte(b));
            }
            Ok(out)
        }
    }
}

impl Default for StreamingEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Streaming decoder: push encoded PUA text, pull decoded UTF-8.
#[derive(Debug, Clone)]
pub struct StreamingDecoder {
    mapping: Mapping,
    /// Accumulated decoded bytes awaiting valid UTF-8 boundary flush.
    byte_buf: Vec<u8>,
}

impl StreamingDecoder {
    /// Identity-mapping streaming decoder.
    pub fn new() -> Self {
        Self::with_mapping(Mapping::identity())
    }

    /// Custom-mapping streaming decoder.
    pub fn with_mapping(mapping: Mapping) -> Self {
        Self {
            mapping,
            byte_buf: Vec::new(),
        }
    }

    /// Push encoded text; returns any complete UTF-8 prefix decoded so far.
    pub fn push(&mut self, encoded_chunk: &str) -> Result<String> {
        for ch in encoded_chunk.chars() {
            let b = self.mapping.decode_char(ch)?;
            self.byte_buf.push(b);
        }
        flush_utf8_prefix(&mut self.byte_buf)
    }

    /// Finish decoding; remaining bytes must form valid UTF-8.
    pub fn finish(self) -> Result<String> {
        if self.byte_buf.is_empty() {
            return Ok(String::new());
        }
        String::from_utf8(self.byte_buf).map_err(|e| Error::InvalidUtf8 {
            offset: e.utf8_error().valid_up_to(),
        })
    }
}

impl Default for StreamingDecoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Split off the longest valid UTF-8 prefix from `buf`.
fn flush_utf8_prefix(buf: &mut Vec<u8>) -> Result<String> {
    match std::str::from_utf8(buf) {
        Ok(s) => {
            let out = s.to_owned();
            buf.clear();
            Ok(out)
        }
        Err(e) => {
            let valid_up_to = e.valid_up_to();
            if valid_up_to == 0 {
                // Need more bytes — check error_len for hard invalid sequences.
                if let Some(error_len) = e.error_len() {
                    return Err(Error::InvalidUtf8 {
                        offset: error_len, // relative; absolute unknown in stream
                    });
                }
                return Ok(String::new());
            }
            let out = std::str::from_utf8(&buf[..valid_up_to])
                .map_err(|err| Error::InvalidUtf8 {
                    offset: err.valid_up_to(),
                })?
                .to_owned();
            let rest = buf[valid_up_to..].to_vec();
            *buf = rest;
            Ok(out)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_roundtrip() {
        let text = "Hello from Doldskrift 🚀";
        let mut enc = StreamingEncoder::new();
        let mut encoded = String::new();
        for chunk in text.as_bytes().chunks(3) {
            encoded.push_str(&enc.push(chunk));
        }
        encoded.push_str(&enc.finish().unwrap());

        let mut dec = StreamingDecoder::new();
        let mut decoded = String::new();
        // Push one char at a time
        for ch in encoded.chars() {
            let mut s = String::new();
            s.push(ch);
            decoded.push_str(&dec.push(&s).unwrap());
        }
        decoded.push_str(&dec.finish().unwrap());
        assert_eq!(decoded, text);
    }
}
