//! Render loop single-step scaffold for future output thread wiring.

use super::render_loop_error::WasapiRenderLoopStepError;
use super::render_loop_report::{
    WasapiRenderLoopStepPlan, WasapiRenderLoopStepReport, WasapiRenderLoopStepSkipReason,
};
use super::tick::manual_drain_tick;
use crate::playback::output_wasapi::sink::WasapiOutputSink;

/// Execute one render loop step on a WasapiOutputSink.
///
/// This is the boundary that future output thread / render loop
/// will call each iteration. It does NOT:
/// - Create threads
/// - Call IAudioClient::Start
/// - Call GetBuffer / ReleaseBuffer directly
/// - Auto-run from submit_frame / open / play / flush / stop / close
/// - Modify NativePipeline
/// - Run a loop (single step only)
///
/// # Behavior
/// - If `plan.enabled == false`: returns no-op report with `Disabled` skip reason.
/// - If `plan.enabled == true`: delegates to `manual_drain_tick`.
///
/// # Safety
/// - Does not modify pending_frames on error (tick + drain helper guarantees this).
/// - Does not consume RingBuffer on error (tick + drain helper guarantees this).
#[allow(dead_code)] // temporary until P0-074E3 output-thread wiring
pub(crate) fn run_manual_drain_render_loop_step(
    sink: &mut WasapiOutputSink,
    plan: WasapiRenderLoopStepPlan,
) -> Result<WasapiRenderLoopStepReport, WasapiRenderLoopStepError> {
    let pending_before = sink.runtime.pending_frames;

    // Skip: step disabled
    if !plan.enabled {
        return Ok(WasapiRenderLoopStepReport {
            plan,
            attempted_tick: false,
            skipped_reason: WasapiRenderLoopStepSkipReason::Disabled,
            tick_report: None,
            pending_before,
            pending_after: pending_before,
        });
    }

    // Execute tick
    let tick_report = manual_drain_tick(sink, plan.requested_frames)?;
    let pending_after = sink.runtime.pending_frames;

    Ok(WasapiRenderLoopStepReport {
        plan,
        attempted_tick: true,
        skipped_reason: WasapiRenderLoopStepSkipReason::None,
        tick_report: Some(tick_report),
        pending_before,
        pending_after,
    })
}
