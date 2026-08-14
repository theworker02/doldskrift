//! Multiplexed logical channels inside a visual / protocol frame.

use serde::{Deserialize, Serialize};

/// Channel identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChannelId(pub u16);

impl ChannelId {
    /// Primary message channel.
    pub const MESSAGE: Self = Self(0);
    /// Metadata.
    pub const METADATA: Self = Self(1);
    /// Telemetry.
    pub const TELEMETRY: Self = Self(2);
    /// Acknowledgements.
    pub const ACK: Self = Self(3);
}

/// Per-channel header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelHeader {
    /// Channel id.
    pub id: ChannelId,
    /// Sequence within the channel.
    pub sequence: u32,
    /// Payload type tag (1=semantic, 2=raw, 3=text, …).
    pub payload_type: u8,
    /// Payload length in bytes.
    pub length: u32,
    /// Integrity (CRC32C of channel payload).
    pub integrity: u32,
}

/// A multiplexed frame body: channel headers + concatenated payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiplexedFrame {
    /// Channel descriptors (in order matching payload concatenation).
    pub channels: Vec<ChannelHeader>,
    /// Concatenated channel payloads.
    pub body: Vec<u8>,
}

impl MultiplexedFrame {
    /// Build from discrete channel payloads.
    pub fn from_channels(parts: Vec<(ChannelId, u8, Vec<u8>)>) -> Self {
        let mut channels = Vec::new();
        let mut body = Vec::new();
        for (i, (id, pty, payload)) in parts.into_iter().enumerate() {
            let integrity = crate::checksum_bytes(&payload);
            channels.push(ChannelHeader {
                id,
                sequence: i as u32,
                payload_type: pty,
                length: payload.len() as u32,
                integrity,
            });
            body.extend_from_slice(&payload);
        }
        Self { channels, body }
    }

    /// Extract payload for a channel id.
    pub fn channel_payload(&self, id: ChannelId) -> Option<&[u8]> {
        let mut offset = 0usize;
        for ch in &self.channels {
            let end = offset + ch.length as usize;
            if end > self.body.len() {
                return None;
            }
            if ch.id == id {
                return Some(&self.body[offset..end]);
            }
            offset = end;
        }
        None
    }
}
