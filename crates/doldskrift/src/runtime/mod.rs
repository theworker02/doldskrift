//! Runtime feature detection and build metadata.

use crate::{
    FONT_VERSION, GLYPH_ENGINE_VERSION, PROTOCOL_VERSION, PROTOCOL_VERSION_PROTECTED,
};

/// Structured crate / protocol metadata (shared by CLI, WASM, agents).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VersionInfo {
    /// `CARGO_PKG_VERSION` of the `doldskrift` crate.
    pub crate_version: &'static str,
    /// Open protocol major (`DSK/1`).
    pub protocol: u8,
    /// Protected container version marker (AEAD not shipping).
    pub protocol_protected: u8,
    /// Font / alphabet revision.
    pub font: u32,
    /// Glyph engine major (MGE).
    pub glyph_engine: u32,
    /// Whether the `ffi` Cargo feature is enabled.
    pub ffi: bool,
}

/// Whether the `ffi` feature is enabled in this build.
pub const fn ffi_enabled() -> bool {
    cfg!(feature = "ffi")
}

/// Crate version string.
pub fn crate_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Snapshot of build / protocol versions for tooling.
pub fn version_info() -> VersionInfo {
    VersionInfo {
        crate_version: crate_version(),
        protocol: PROTOCOL_VERSION,
        protocol_protected: PROTOCOL_VERSION_PROTECTED,
        font: FONT_VERSION,
        glyph_engine: GLYPH_ENGINE_VERSION,
        ffi: ffi_enabled(),
    }
}

/// Compact human-readable runtime summary.
pub fn summary() -> String {
    let v = version_info();
    format!(
        "doldskrift {} (protocol {}, font {}, mge {}, ffi={})",
        v.crate_version, v.protocol, v.font, v.glyph_engine, v.ffi
    )
}

/// Agent-friendly JSON for [`version_info`] (no serde dependency in this module).
pub fn version_info_json() -> String {
    let v = version_info();
    format!(
        "{{\n  \"crate_version\": \"{}\",\n  \"protocol\": {},\n  \"protocol_protected\": {},\n  \"font\": {},\n  \"glyph_engine\": {},\n  \"ffi\": {},\n  \"honesty\": \"Open/Neural ≠ encryption; Protected AEAD not shipping\"\n}}",
        v.crate_version,
        v.protocol,
        v.protocol_protected,
        v.font,
        v.glyph_engine,
        v.ffi
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_info_matches_crate() {
        let v = version_info();
        assert_eq!(v.crate_version, env!("CARGO_PKG_VERSION"));
        assert_eq!(v.protocol, PROTOCOL_VERSION);
        assert_eq!(v.font, FONT_VERSION);
        assert!(version_info_json().contains("crate_version"));
        assert!(summary().contains("doldskrift"));
    }
}
