//! Thin render step adapter for future output thread wiring.
//!
//! Delegates to `run_manual_drain_render_loop_step` without
//! directly calling `manual_drain_tick` or `drain_wasapi_output_sink_once`.
//!
//! **This module does NOT:**
//! - Create threads
//! - Create channels
//! - Write loops
//! - Call IAudioClient::Start
//! - Call GetBuffer / ReleaseBuffer directly
//! - Auto-run from submit_frame / open / play / flush / stop / close
//! - Modify NativePipeline
//! - Modify sink.rs

use super::render_loop_report::WasapiRenderLoopStepPlan;
use super::render_loop_step::run_manual_drain_render_loop_step;
use super::render_step_adapter_error::WasapiRenderStepAdapterError;
use super::render_step_adapter_report::{
    WasapiRenderStepAdapterConfig, WasapiRenderStepAdapterReport,
};
use crate::playback::output_wasapi::sink::WasapiOutputSink;

/// Execute one render step via the adapter on a WasapiOutputSink.
///
/// This is the thin boundary that a future output thread will call.
/// It delegates to `run_manual_drain_render_loop_step` which in turn
/// delegates to `manual_drain_tick` and then to the safe drain helper.
///
/// # Behavior
/// - If `config.enabled == false`: delegates to step which returns Disabled.
/// - If `config.enabled == true`: delegates to step which checks pending/requested.
///
/// # Safety
/// - Does not modify pending_frames on error (step/tick/drain guarantees this).
/// - Does not consume RingBuffer on error (step/tick/drain guarantees this).
#[allow(dead_code)] // temporary until P0-074E4 output-thread wiring
pub(crate) fn run_wasapi_render_step_adapter(
    sink: &mut WasapiOutputSink,
    config: WasapiRenderStepAdapterConfig,
) -> Result<WasapiRenderStepAdapterReport, WasapiRenderStepAdapterError> {
    let pending_before = sink.runtime.pending_frames;

    // Build the plan for the render loop step
    let plan = WasapiRenderLoopStepPlan {
        requested_frames: config.requested_frames,
        enabled: config.enabled,
    };

    // Delegate to render loop step
    let step_report = run_manual_drain_render_loop_step(sink, plan)?;

    let pending_after = sink.runtime.pending_frames;

    Ok(WasapiRenderStepAdapterReport {
        config,
        step_report,
        pending_before,
        pending_after,
    })
}
