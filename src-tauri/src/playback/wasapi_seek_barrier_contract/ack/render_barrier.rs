use super::super::generation::{AckGeneration, SeekOutputGeneration};
use super::status::WasapiSeekBarrierAckStatus;

/// Ack for render barrier completion.
///
/// Binds the observed generation. Barrier satisfaction requires
/// Completed status AND generation match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderBarrierAck {
    status: WasapiSeekBarrierAckStatus,
    generation: AckGeneration,
}

impl RenderBarrierAck {
    /// Create a completed ack.
    pub fn completed(generation: AckGeneration) -> Self {
        Self {
            status: WasapiSeekBarrierAckStatus::Completed,
            generation,
        }
    }

    /// Create a rejected ack.
    pub fn rejected(generation: AckGeneration) -> Self {
        Self {
            status: WasapiSeekBarrierAckStatus::Rejected,
            generation,
        }
    }

    /// Create a failed ack.
    pub fn failed(generation: AckGeneration) -> Self {
        Self {
            status: WasapiSeekBarrierAckStatus::Failed,
            generation,
        }
    }

    /// Create an unsupported ack.
    pub fn unsupported(generation: AckGeneration) -> Self {
        Self {
            status: WasapiSeekBarrierAckStatus::Unsupported,
            generation,
        }
    }

    /// Create a stale-generation ack.
    pub fn stale_generation(generation: AckGeneration) -> Self {
        Self {
            status: WasapiSeekBarrierAckStatus::StaleGeneration,
            generation,
        }
    }

    /// The observed ack generation.
    pub fn generation(&self) -> AckGeneration {
        self.generation
    }

    /// Whether the status is Completed.
    pub fn is_completed(&self) -> bool {
        self.status == WasapiSeekBarrierAckStatus::Completed
    }

    /// Whether the status is Rejected.
    pub fn is_rejected(&self) -> bool {
        self.status == WasapiSeekBarrierAckStatus::Rejected
    }

    /// Whether the status is Failed.
    pub fn is_failed(&self) -> bool {
        self.status == WasapiSeekBarrierAckStatus::Failed
    }

    /// Whether the status is Unsupported.
    pub fn is_unsupported(&self) -> bool {
        self.status == WasapiSeekBarrierAckStatus::Unsupported
    }

    /// Whether the status is StaleGeneration.
    pub fn is_stale_generation(&self) -> bool {
        self.status == WasapiSeekBarrierAckStatus::StaleGeneration
    }

    /// Whether this ack satisfies the render barrier for the expected generation.
    ///
    /// Satisfied only when status is Completed AND generation matches.
    /// Rejected / Failed / Unsupported / StaleGeneration never satisfy.
    pub fn satisfies_render_barrier(&self, expected: SeekOutputGeneration) -> bool {
        self.status == WasapiSeekBarrierAckStatus::Completed
            && self.generation.value() == expected.value()
    }
}
