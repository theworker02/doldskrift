//! Capture helpers for acquiring visual / textual Doldskrift inputs.

use crate::{Error, Result};
use std::path::Path;

/// Captured input kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureKind {
    /// UTF-8 text / armored payload.
    Text,
    /// Raw `.dsk` bytes.
    Binary,
    /// Image path (vision pipeline).
    Image,
}

/// A captured input reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capture {
    /// Kind of capture.
    pub kind: CaptureKind,
    /// Optional filesystem path.
    pub path: Option<String>,
    /// In-memory bytes when available.
    pub bytes: Vec<u8>,
}

impl Capture {
    /// Capture from an in-memory byte buffer.
    pub fn from_bytes(kind: CaptureKind, bytes: Vec<u8>) -> Self {
        Self {
            kind,
            path: None,
            bytes,
        }
    }

    /// Capture by reading a file (binary).
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let bytes = std::fs::read(path).map_err(|e| Error::IoError(e.to_string()))?;
        let kind = match path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase()
            .as_str()
        {
            "dsk" => CaptureKind::Binary,
            "png" | "jpg" | "jpeg" | "webp" => CaptureKind::Image,
            _ => CaptureKind::Text,
        };
        Ok(Self {
            kind,
            path: Some(path.display().to_string()),
            bytes,
        })
    }
}
