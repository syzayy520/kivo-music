//! Report types for render loop single-step operations.

use super::tick_report::WasapiDrainTickReport;

/// Plan for a single render loop step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct WasapiRenderLoopStepPlan {
    /// Maximum frames to drain this step.
    pub(crate) requested_frames: u32,
    /// Whether this step is allowed to execute the drain tick.
    pub(crate) enabled: bool,
}

/// Reason a render loop step was skipped (no tick attempted).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WasapiRenderLoopStepSkipReason {
    /// Tick was attempted or step succeeded.
    None,
    /// Step was disabled via plan.enabled == false.
    Disabled,
}

/// Report from a single render loop step.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WasapiRenderLoopStepReport {
    /// The plan that was used for this step.
    pub(crate) plan: WasapiRenderLoopStepPlan,
    /// Whether manual_drain_tick was called.
    pub(crate) attempted_tick: bool,
    /// Why the step was skipped (None if tick was attempted).
    pub(crate) skipped_reason: WasapiRenderLoopStepSkipReason,
    /// Result from tick if attempted and successful.
    pub(crate) tick_report: Option<WasapiDrainTickReport>,
    /// pending_frames before step.
    pub(crate) pending_before: usize,
    /// pending_frames after step.
    pub(crate) pending_after: usize,
}
