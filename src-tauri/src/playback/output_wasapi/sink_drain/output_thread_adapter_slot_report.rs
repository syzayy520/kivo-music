//! Config and report types for the output-thread adapter slot.

use super::render_step_adapter_report::WasapiRenderStepAdapterReport;

/// Configuration for a single output-thread adapter slot invocation.
///
/// This is the thin entry point for a future output thread.
/// It does NOT introduce loop config, sleep duration,
/// thread config, or channel config.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct WasapiOutputThreadAdapterSlotConfig {
    /// Maximum frames to request from the adapter.
    pub(crate) requested_frames: u32,
    /// Whether the slot is allowed to execute the adapter.
    pub(crate) enabled: bool,
}

/// Report from a single output-thread adapter slot invocation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WasapiOutputThreadAdapterSlotReport {
    /// The config that was used for this slot call.
    pub(crate) config: WasapiOutputThreadAdapterSlotConfig,
    /// Report from the underlying render step adapter.
    pub(crate) adapter_report: WasapiRenderStepAdapterReport,
    /// pending_frames before slot call.
    pub(crate) pending_before: usize,
    /// pending_frames after slot call.
    pub(crate) pending_after: usize,
}
