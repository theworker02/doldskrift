//! Resource limits for parsers and streams.

/// Default maximum `.dsk` payload size (64 MiB).
pub const DEFAULT_MAX_PAYLOAD: usize = 64 * 1024 * 1024;

/// Default maximum header JSON size (1 MiB).
pub const DEFAULT_MAX_HEADER: usize = 1024 * 1024;

/// Default maximum streaming buffer (4 MiB).
pub const DEFAULT_MAX_STREAM_BUF: usize = 4 * 1024 * 1024;

/// Configurable limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    /// Max payload bytes.
    pub max_payload: usize,
    /// Max header bytes.
    pub max_header: usize,
    /// Max stream buffer.
    pub max_stream_buf: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_payload: DEFAULT_MAX_PAYLOAD,
            max_header: DEFAULT_MAX_HEADER,
            max_stream_buf: DEFAULT_MAX_STREAM_BUF,
        }
    }
}

impl Limits {
    /// Strict small-document limits (for tests / constrained agents).
    pub fn strict() -> Self {
        Self {
            max_payload: 256 * 1024,
            max_header: 64 * 1024,
            max_stream_buf: 64 * 1024,
        }
    }

    /// Check payload length against limit.
    pub fn check_payload(self, len: usize) -> Result<(), String> {
        if len > self.max_payload {
            Err(format!("payload {len} exceeds limit {}", self.max_payload))
        } else {
            Ok(())
        }
    }
}
