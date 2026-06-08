use super::super::generation::{AckGeneration, StaleGeneration};

/// Focused ack value for stale-generation reporting.
///
/// StaleGenerationAck is the ack-level stale-generation value.
/// Ack status variant WasapiSeekBarrierAckStatus::StaleGeneration is the status classification.
/// Both exist; tests prove they do not replace generation::StaleGeneration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaleGenerationAck {
    generation: AckGeneration,
    mismatch: StaleGeneration,
}

impl StaleGenerationAck {
    /// Create a new StaleGenerationAck.
    pub fn new(generation: AckGeneration, mismatch: StaleGeneration) -> Self {
        Self {
            generation,
            mismatch,
        }
    }

    /// The observed ack generation.
    pub fn generation(&self) -> AckGeneration {
        self.generation
    }

    /// The generation mismatch detail.
    pub fn mismatch(&self) -> &StaleGeneration {
        &self.mismatch
    }
}
