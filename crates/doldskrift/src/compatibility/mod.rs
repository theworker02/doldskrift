//! Protocol / font version compatibility checks.

use crate::{Error, ProtocolVersion, Result, FONT_VERSION, PROTOCOL_VERSION};

/// Check that a peer protocol version is supported.
pub fn check_protocol(version: u8) -> Result<()> {
    ProtocolVersion(version).check_supported()
}

/// Check that a font / glyph grammar version is acceptable.
pub fn check_font_version(version: u32) -> Result<()> {
    if version == FONT_VERSION || version == 1 {
        Ok(())
    } else if version > FONT_VERSION {
        Err(Error::UnsupportedVersion(version as u8))
    } else {
        // Older versions: accept with forward generation.
        Ok(())
    }
}

/// Local implementation versions.
pub fn local_versions() -> (u8, u32) {
    (PROTOCOL_VERSION, FONT_VERSION)
}
