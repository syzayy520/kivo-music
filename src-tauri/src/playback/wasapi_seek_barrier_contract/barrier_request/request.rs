use super::super::barrier_target::WasapiBarrierTargets;
use super::super::generation::{RenderEpoch, SeekOutputGeneration};
use super::super::ordering::WasapiSeekBarrierOrdering;
use super::super::queue_policy::{StaleCommandPolicy, WasapiQueuePolicy};
use super::super::reset_policy::WasapiResetPolicy;
use super::reason::WasapiSeekBarrierReason;

/// A single WASAPI seek barrier request.
///
/// Binds generation, render epoch, reason, targets, and all policies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasapiSeekBarrierRequest {
    /// The seek output generation this barrier targets.
    pub generation: SeekOutputGeneration,
    /// The render epoch at barrier time.
    pub render_epoch: RenderEpoch,
    /// Why the barrier was requested.
    pub reason: WasapiSeekBarrierReason,
    /// Which pipeline components must satisfy the barrier.
    pub targets: WasapiBarrierTargets,
    /// Queue drain/drop policy.
    pub queue_policy: WasapiQueuePolicy,
    /// Stale command handling policy.
    pub stale_command_policy: StaleCommandPolicy,
    /// Device reset policy.
    pub reset_policy: WasapiResetPolicy,
    /// Phase ordering.
    pub ordering: WasapiSeekBarrierOrdering,
}

impl WasapiSeekBarrierRequest {
    /// Whether this is a Playing seek barrier.
    pub fn is_playing_seek(&self) -> bool {
        self.reason == WasapiSeekBarrierReason::PlayingSeek
    }

    /// Whether this is a Paused seek barrier.
    pub fn is_paused_seek(&self) -> bool {
        self.reason == WasapiSeekBarrierReason::PausedSeek
    }
}
