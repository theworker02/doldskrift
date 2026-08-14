//! Foreign-language surface for Doldskrift.
//!
//! Rust remains the canonical protocol implementation. This crate exposes
//! thin WASM (and optional C ABI via `doldskrift/ffi`) adapters.

#![warn(missing_docs)]

use doldskrift::{decode, encode, runtime, PROTOCOL_VERSION, PUA_BASE};

/// Non-WASM helpers for Node tests / embedding.
pub fn encode_text(text: &str) -> doldskrift::Result<String> {
    encode(text)
}

/// Decode helper.
pub fn decode_text(text: &str) -> doldskrift::Result<String> {
    decode(text)
}

/// Structured version JSON (same fields as `doldskrift::runtime::version_info_json`).
pub fn version_info_json() -> String {
    runtime::version_info_json()
}

/// Encode text with the static DSK/1 mapping (WASM/JS entry).
#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn wasm_encode(text: &str) -> Result<String, wasm_bindgen::JsError> {
    encode(text).map_err(|e| wasm_bindgen::JsError::new(&e.to_string()))
}

/// Decode PUA-encoded text (WASM/JS entry).
#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn wasm_decode(text: &str) -> Result<String, wasm_bindgen::JsError> {
    decode(text).map_err(|e| wasm_bindgen::JsError::new(&e.to_string()))
}

/// Protocol major version exposed to JS.
#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn wasm_protocol_version() -> u8 {
    PROTOCOL_VERSION
}

/// PUA base codepoint for the 256-symbol alphabet.
#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn wasm_pua_base() -> u32 {
    PUA_BASE
}

/// Package / crate version string.
#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn wasm_crate_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

/// JSON blob: crate + protocol + font + glyph engine versions (WASM/JS).
#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn wasm_version_info() -> String {
    runtime::version_info_json()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let e = encode_text("hello").unwrap();
        assert_eq!(decode_text(&e).unwrap(), "hello");
    }

    #[test]
    fn version_info_json_lists_protocol() {
        let j = version_info_json();
        assert!(j.contains("protocol"));
        assert!(j.contains("crate_version"));
    }
}
