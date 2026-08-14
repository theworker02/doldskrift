//! Ordered visual / motion frame sequences.

use serde::{Deserialize, Serialize};

/// Header for a single stream frame.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamFrameHeader {
    /// Stream identifier.
    pub stream_id: u32,
    /// Monotonic sequence number.
    pub sequence: u32,
    /// Timestamp milliseconds (optional semantics).
    pub timestamp_ms: u64,
}

/// Ordered collection of frame payloads.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FrameSequence {
    /// Frames in presentation order.
    pub frames: Vec<StreamFrameHeader>,
}

impl FrameSequence {
    /// Create an empty sequence.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a frame header.
    pub fn push(&mut self, header: StreamFrameHeader) {
        self.frames.push(header);
    }

    /// Number of frames.
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    /// True when empty.
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}

/// Motion / temporal stream of frames (logical).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MotionStream {
    /// Underlying sequence.
    pub sequence: FrameSequence,
    /// Frames per second hint (0 = unknown).
    pub fps_hint: u16,
}

impl MotionStream {
    /// Create with an fps hint.
    pub fn with_fps(fps_hint: u16) -> Self {
        Self {
            sequence: FrameSequence::new(),
            fps_hint,
        }
    }
}
