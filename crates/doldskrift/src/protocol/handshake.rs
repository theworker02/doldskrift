//! Optional agent handshake / capability negotiation.

use crate::{Capability, Error, Mode, Result, HANDSHAKE_QUERY, HANDSHAKE_REPLY, PROTOCOL_VERSION};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Capabilities advertised by the reference implementation.
pub const DEFAULT_CAPABILITIES: &[Capability] = &[
    Capability::Visual,
    Capability::Encoded,
    Capability::Session,
    Capability::Stream,
    Capability::Container,
];

/// Set of capabilities.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CapabilitySet {
    caps: BTreeSet<String>,
}

impl CapabilitySet {
    /// Empty set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reference implementation defaults.
    pub fn reference() -> Self {
        let mut set = Self::new();
        for c in DEFAULT_CAPABILITIES {
            set.insert(*c);
        }
        set
    }

    /// Insert a capability.
    pub fn insert(&mut self, cap: Capability) {
        self.caps.insert(cap.as_str().to_owned());
    }

    /// Returns true if present.
    pub fn contains(&self, cap: Capability) -> bool {
        self.caps.contains(cap.as_str())
    }

    /// Intersection.
    pub fn intersect(&self, other: &Self) -> Self {
        Self {
            caps: self.caps.intersection(&other.caps).cloned().collect(),
        }
    }

    /// Parse from comma-separated tokens.
    pub fn parse_list(s: &str) -> Self {
        let mut set = Self::new();
        for part in s.split(',') {
            if let Some(c) = Capability::parse(part) {
                set.insert(c);
            }
        }
        set
    }

    /// Render as comma-separated tokens.
    pub fn to_list(&self) -> String {
        self.caps.iter().cloned().collect::<Vec<_>>().join(",")
    }

    /// Modes supported by this set, highest preference first.
    pub fn preferred_modes(&self) -> Vec<Mode> {
        // Prefer session > encoded > visual when mutually available.
        let order = [Mode::Session, Mode::Encoded, Mode::Visual];
        order
            .into_iter()
            .filter(|m| self.contains(m.capability()))
            .collect()
    }
}

/// `DSK?` offer from Agent A.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandshakeOffer {
    /// Supported protocol versions.
    pub versions: Vec<u8>,
    /// Advertised capabilities / modes.
    pub capabilities: CapabilitySet,
}

impl HandshakeOffer {
    /// Reference offer.
    pub fn reference() -> Self {
        Self {
            versions: vec![PROTOCOL_VERSION],
            capabilities: CapabilitySet::reference(),
        }
    }

    /// Encode wire form:
    /// `DSK?\nversions=1\nmodes=visual,encoded,session\n`
    pub fn to_wire(&self) -> String {
        let modes = self
            .capabilities
            .preferred_modes()
            .into_iter()
            .map(|m| m.as_str().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        let versions = self
            .versions
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{HANDSHAKE_QUERY}\nversions={versions}\nmodes={modes}\ncapabilities={}\n",
            self.capabilities.to_list()
        )
    }

    /// Parse wire form.
    pub fn from_wire(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let first = lines.next().unwrap_or("").trim();
        if first != HANDSHAKE_QUERY {
            return Err(Error::InvalidManifest(format!(
                "expected {HANDSHAKE_QUERY}, got {first}"
            )));
        }
        let mut versions = Vec::new();
        let mut capabilities = CapabilitySet::new();
        for line in lines {
            if let Some(v) = line.strip_prefix("versions=") {
                for part in v.split(',') {
                    if let Ok(n) = part.trim().parse() {
                        versions.push(n);
                    }
                }
            } else if let Some(m) = line.strip_prefix("modes=") {
                for part in m.split(',') {
                    if let Ok(mode) = part.trim().parse::<Mode>() {
                        capabilities.insert(mode.capability());
                    }
                }
            } else if let Some(c) = line.strip_prefix("capabilities=") {
                for part in c.split(',') {
                    if let Some(cap) = Capability::parse(part) {
                        capabilities.insert(cap);
                    }
                }
            }
        }
        if versions.is_empty() {
            versions.push(PROTOCOL_VERSION);
        }
        Ok(Self {
            versions,
            capabilities,
        })
    }
}

/// `DSK!` reply from Agent B.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandshakeReply {
    /// Selected protocol version.
    pub version: u8,
    /// Selected mode.
    pub mode: Mode,
}

impl HandshakeReply {
    /// Wire form.
    pub fn to_wire(&self) -> String {
        format!(
            "{HANDSHAKE_REPLY}\nversion={}\nmode={}\n",
            self.version,
            self.mode.as_str()
        )
    }

    /// Parse wire form.
    pub fn from_wire(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let first = lines.next().unwrap_or("").trim();
        if first != HANDSHAKE_REPLY {
            return Err(Error::InvalidManifest(format!(
                "expected {HANDSHAKE_REPLY}, got {first}"
            )));
        }
        let mut version = None;
        let mut mode = None;
        for line in lines {
            if let Some(v) = line.strip_prefix("version=") {
                version = v.trim().parse().ok();
            } else if let Some(m) = line.strip_prefix("mode=") {
                mode = m.trim().parse().ok();
            }
        }
        Ok(Self {
            version: version
                .ok_or_else(|| Error::InvalidManifest("handshake reply missing version".into()))?,
            mode: mode
                .ok_or_else(|| Error::InvalidManifest("handshake reply missing mode".into()))?,
        })
    }
}

/// Select the highest mutually compatible mode and version.
pub fn negotiate(offer_a: &HandshakeOffer, offer_b: &CapabilitySet) -> Result<HandshakeReply> {
    let shared_version = offer_a
        .versions
        .iter()
        .copied()
        .find(|v| *v == PROTOCOL_VERSION)
        .ok_or_else(|| Error::NegotiationFailed("no shared protocol version".into()))?;

    let shared = offer_a.capabilities.intersect(offer_b);
    let mode = shared
        .preferred_modes()
        .into_iter()
        .next()
        .ok_or_else(|| Error::NegotiationFailed("no shared mode".into()))?;

    Ok(HandshakeReply {
        version: shared_version,
        mode,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negotiate_session() {
        let a = HandshakeOffer::reference();
        let b = CapabilitySet::reference();
        let reply = negotiate(&a, &b).unwrap();
        assert_eq!(reply.mode, Mode::Session);
        let wire = reply.to_wire();
        assert_eq!(HandshakeReply::from_wire(&wire).unwrap(), reply);
    }
}
