//! Config and report types for the render step adapter.

use super::render_loop_report::WasapiRenderLoopStepReport;

/// Configuration for a single render step adapter invocation.
///
/// This is the thin entry point for a future output thread.
/// It does NOT introduce loop config, sleep duration,
/// thread config, or channel config.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct WasapiRenderStepAdapterConfig {
    /// Maximum frames to request from the render loop step.
    pub(crate) requested_frames: u32,
    /// Whether the adapter is allowed to execute the step.
    pub(crate) enabled: bool,
}

/// Report from a single render step adapter invocation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WasapiRenderStepAdapterReport {
    /// The config that was used for this adapter call.
    pub(crate) config: WasapiRenderStepAdapterConfig,
    /// Report from the underlying render loop step.
    pub(crate) step_report: WasapiRenderLoopStepReport,
    /// pending_frames before adapter call.
    pub(crate) pending_before: usize,
    /// pending_frames after adapter call.
    pub(crate) pending_after: usize,
}
