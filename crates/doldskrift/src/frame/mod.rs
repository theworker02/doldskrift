//! DSK Visual Frames — rectangular machine-readable regions.

mod beacon;
mod channel;
mod sequence;

pub use beacon::{detect_beacon, Beacon, BEACON_PATTERN};
pub use channel::{ChannelHeader, ChannelId, MultiplexedFrame};
pub use sequence::{FrameSequence, MotionStream, StreamFrameHeader};

use crate::{checksum_bytes, Error, Result, MAGIC_DSK2};
use serde::{Deserialize, Serialize};

/// Visual frame orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum Orientation {
    /// Upright.
    #[default]
    Deg0 = 0,
    /// Rotated 90° CW.
    Deg90 = 1,
    /// Rotated 180°.
    Deg180 = 2,
    /// Rotated 270° CW.
    Deg270 = 3,
}

/// Corner marker pattern IDs (locator).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CornerMarkers {
    /// Top-left pattern id.
    pub tl: u8,
    /// Top-right.
    pub tr: u8,
    /// Bottom-left.
    pub bl: u8,
    /// Bottom-right (encodes orientation hint).
    pub br: u8,
}

impl CornerMarkers {
    /// Standard DSK frame corners.
    pub fn standard(orientation: Orientation) -> Self {
        Self {
            tl: 0xA1,
            tr: 0xA2,
            bl: 0xA3,
            br: 0xB0 | (orientation as u8),
        }
    }

    /// Infer orientation from BR marker.
    pub fn orientation(self) -> Orientation {
        match self.br & 0x0f {
            1 => Orientation::Deg90,
            2 => Orientation::Deg180,
            3 => Orientation::Deg270,
            _ => Orientation::Deg0,
        }
    }
}

/// DSK Visual Frame header (logical).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFrame {
    /// Protocol major (2 for DSK/2 frames).
    pub version: u8,
    /// Frame id within a stream.
    pub frame_id: u32,
    /// Sequence number.
    pub sequence: u32,
    /// Orientation.
    pub orientation: Orientation,
    /// Channel map / multiplexed payloads.
    pub channels: Vec<ChannelHeader>,
    /// Raw payload (may already include ECC).
    pub payload: Vec<u8>,
    /// ECC level used.
    pub ecc_level: crate::ecc::EccLevel,
    /// CRC32C over header fields + payload.
    pub checksum: u32,
}

impl VisualFrame {
    /// Build a single-channel semantic frame.
    pub fn from_payload(
        frame_id: u32,
        sequence: u32,
        payload: Vec<u8>,
        ecc: crate::ecc::EccLevel,
    ) -> Self {
        let protected = crate::ecc::encode(&payload, ecc).unwrap_or_else(|_| payload.clone());
        let mut frame = Self {
            version: 2,
            frame_id,
            sequence,
            orientation: Orientation::Deg0,
            channels: vec![ChannelHeader {
                id: ChannelId(0),
                sequence,
                payload_type: 1, // semantic
                length: protected.len() as u32,
                integrity: checksum_bytes(&protected),
            }],
            payload: protected,
            ecc_level: ecc,
            checksum: 0,
        };
        frame.checksum = frame.compute_checksum();
        frame
    }

    fn compute_checksum(&self) -> u32 {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.version.to_be_bytes());
        buf.extend_from_slice(&self.frame_id.to_be_bytes());
        buf.extend_from_slice(&self.sequence.to_be_bytes());
        buf.push(self.orientation as u8);
        buf.extend_from_slice(&self.payload);
        checksum_bytes(&buf)
    }

    /// Serialize to a compact binary envelope (not the glyph raster).
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let meta = serde_json::to_vec(self).map_err(|e| Error::Parse(e.to_string()))?;
        let mut out = Vec::with_capacity(8 + meta.len());
        out.extend_from_slice(MAGIC_DSK2);
        out.extend_from_slice(&(meta.len() as u32).to_be_bytes());
        out.extend_from_slice(&meta);
        Ok(out)
    }

    /// Parse binary envelope.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < 8 || &data[0..4] != MAGIC_DSK2 {
            return Err(Error::Frame("expected DSK2 frame magic".into()));
        }
        let len = u32::from_be_bytes(data[4..8].try_into().unwrap()) as usize;
        if data.len() < 8 + len {
            return Err(Error::TruncatedPayload {
                needed: 8 + len - data.len(),
            });
        }
        let frame: Self = serde_json::from_slice(&data[8..8 + len])?;
        if frame.checksum != frame.compute_checksum() {
            return Err(Error::ChecksumMismatch {
                expected: frame.checksum,
                actual: frame.compute_checksum(),
            });
        }
        Ok(frame)
    }

    /// Recover payload through ECC and return bytes + recovery report.
    pub fn decode_payload(&self) -> Result<(Vec<u8>, crate::ecc::RecoveryReport)> {
        crate::ecc::decode(&self.payload, self.ecc_level)
    }
}

/// Detected frame in an image (vision output).
#[derive(Debug, Clone, PartialEq)]
pub struct DetectedFrame {
    /// Axis-aligned bounds (x, y, w, h) in image pixels.
    pub bounds: (u32, u32, u32, u32),
    /// Estimated orientation.
    pub orientation: Orientation,
    /// Detection confidence 0.0–1.0.
    pub confidence: f32,
    /// Optional frame id if decoded from header.
    pub frame_id: Option<u32>,
}
