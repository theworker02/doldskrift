//! Textual armor for binary Doldskrift payloads (base64 lines).

use crate::{Error, Result};
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;

const BEGIN: &str = "-----BEGIN DOLDSKRIFT-----";
const END: &str = "-----END DOLDSKRIFT-----";

/// Wrap raw bytes in armored text.
pub fn armor(bytes: &[u8]) -> String {
    let encoded = B64.encode(bytes);
    let mut out = String::from(BEGIN);
    out.push('\n');
    for chunk in encoded.as_bytes().chunks(64) {
        out.push_str(std::str::from_utf8(chunk).unwrap());
        out.push('\n');
    }
    out.push_str(END);
    out.push('\n');
    out
}

/// Unwrap armored text to raw bytes.
pub fn dearmor(text: &str) -> Result<Vec<u8>> {
    let mut collecting = false;
    let mut body = String::new();
    for line in text.lines() {
        let line = line.trim();
        if line == BEGIN {
            collecting = true;
            continue;
        }
        if line == END {
            break;
        }
        if collecting {
            body.push_str(line);
        }
    }
    if body.is_empty() {
        return Err(Error::Parse("missing armored body".into()));
    }
    B64.decode(body.as_bytes())
        .map_err(|e| Error::Parse(format!("base64: {e}")))
}
