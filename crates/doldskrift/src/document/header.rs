//! Document header JSON metadata.

use crate::session::SessionManifest;
use crate::{Error, Mode, Result, FONT_VERSION, PROTOCOL_VERSION};
use serde::{Deserialize, Serialize};

/// Bitflags stored in the binary flags byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DocumentFlags(u8);

impl DocumentFlags {
    /// No flags.
    pub const NONE: Self = Self(0);
    /// Payload is compressed (reserved for future use).
    pub const COMPRESSED: Self = Self(0b0000_0001);
    /// Experimental ECC present (reserved).
    pub const ECC: Self = Self(0b0000_0010);

    /// Construct from raw bits.
    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    /// Raw bits.
    pub const fn bits(self) -> u8 {
        self.0
    }
}

/// JSON header embedded in a `.dsk` container.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentHeader {
    /// Protocol major version.
    pub version: u8,
    /// Operating mode.
    pub mode: Mode,
    /// Flags (also mirrored in the binary preamble).
    #[serde(default)]
    pub flags: DocumentFlags,
    /// CRC32C over header JSON bytes || payload (filled on write/parse).
    #[serde(default)]
    pub checksum: u32,
    /// Optional mapping id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping_id: Option<String>,
    /// Optional session manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<SessionManifest>,
    /// Optional semantic character count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_count: Option<u64>,
    /// Optional semantic byte count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_count: Option<u64>,
    /// Font / glyph grammar version.
    #[serde(default = "default_font_version")]
    pub font_version: u32,
    /// Free-form metadata.
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub meta: serde_json::Map<String, serde_json::Value>,
}

fn default_font_version() -> u32 {
    FONT_VERSION
}

impl DocumentHeader {
    /// Fresh header for `mode`.
    pub fn new(mode: Mode) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            mode,
            flags: DocumentFlags::NONE,
            checksum: 0,
            mapping_id: None,
            session: None,
            char_count: None,
            byte_count: None,
            font_version: FONT_VERSION,
            meta: serde_json::Map::new(),
        }
    }

    /// Serialize header JSON bytes (checksum field zeroed for coverage calc).
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut for_ser = self.clone();
        // Checksum lives in the binary preamble; keep JSON free of it for stability.
        for_ser.checksum = 0;
        serde_json::to_vec(&for_ser).map_err(|e| Error::InvalidManifest(e.to_string()))
    }

    /// Parse header JSON bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(|e| Error::InvalidManifest(e.to_string()))
    }
}

impl Serialize for DocumentFlags {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_u8(self.bits())
    }
}

impl<'de> Deserialize<'de> for DocumentFlags {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        let bits = u8::deserialize(deserializer)?;
        Ok(DocumentFlags::from_bits(bits))
    }
}
