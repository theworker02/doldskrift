//! Discovery, handshake, and DSK/2 capability negotiation.

mod discovery;
mod handshake;

pub use discovery::{
    detect_from_attrs, detect_from_html_meta, detect_pua_heuristic, Detection, DetectionSource,
};
pub use handshake::{
    negotiate, CapabilitySet, HandshakeOffer, HandshakeReply, DEFAULT_CAPABILITIES,
};

/// Capability profile names (Phase III).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityProfile {
    /// Core text codec only.
    Minimal,
    /// Semantic values + containers.
    Standard,
    /// MGE + scanner + frames.
    Visual,
    /// All stable capabilities.
    Full,
}

impl CapabilityProfile {
    /// Wire name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minimal => "DSK_MINIMAL",
            Self::Standard => "DSK_STANDARD",
            Self::Visual => "DSK_VISUAL",
            Self::Full => "DSK_FULL",
        }
    }
}
