//! DSK/3 Neural — latent structure grammar, readers, and profile metadata.
//!
//! **Neural ≠ encryption.** Latent visuals and trained readers are a
//! representation stack. Confidentiality is the Protected/AEAD + Gate profile.
//!
//! Foundation pass: compiling types + Mock/DeterministicBaseline path for CI.
//! Trained Gemma/adapter inference is deferred.

mod composer;
mod epoch;
mod gate_caps;
mod lsg;
mod observation;
mod profile;
mod reader;

pub use composer::{compose_deterministic, DeterministicComposer, SemanticComposer};
pub use epoch::{GrammarEpoch, ReaderCompatibility, LSG_VERSION, LSG_VERSION_LABEL};
pub use gate_caps::{GateCapability, GatePolicy};
pub use lsg::{LatentEdge, LatentGraph, LatentMetadata, LatentNode, LatentNodeKind};
pub use observation::{ObservationEdge, ObservationGraph, ObservationNode, ObservationNodeKind};
pub use profile::{Dsk3Profile, ProfileKind};
pub use reader::{
    DeterministicBaseline, EpochLockedReader, MockReader, ReaderBackend, ReaderId, ReaderOutput,
    ReaderStatus, SessionReader,
};

#[cfg(feature = "gemma-reader")]
pub use reader::GemmaReaderStub;
