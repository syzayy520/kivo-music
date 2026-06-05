//! Report types for manual drain tick operations.

use super::report::WasapiRingBufferDrainReport;

/// Reason a drain tick was skipped (no drain attempted).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WasapiDrainTickSkipReason {
    /// Drain was attempted.
    None,
    /// requested_frames == 0, no-op.
    RequestedZero,
    /// pending_frames == 0, no-op.
    NoPendingFrames,
}

/// Report from a manual drain tick.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WasapiDrainTickReport {
    /// Maximum frames requested to drain this tick.
    pub(crate) requested_frames: u32,
    /// Whether drain_wasapi_output_sink_once was called.
    pub(crate) attempted: bool,
    /// Why the tick was skipped (None if drain was attempted).
    pub(crate) skipped_reason: WasapiDrainTickSkipReason,
    /// Result from drain helper if attempted.
    pub(crate) drain_report: Option<WasapiRingBufferDrainReport>,
    /// pending_frames before tick.
    pub(crate) pending_before: usize,
    /// pending_frames after tick.
    pub(crate) pending_after: usize,
}
