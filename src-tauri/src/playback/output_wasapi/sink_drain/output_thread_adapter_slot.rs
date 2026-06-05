//! Thin output-thread adapter slot for future output thread wiring.
//!
//! Delegates to `run_wasapi_render_step_adapter` without
//! directly calling `run_manual_drain_render_loop_step`,
//! `manual_drain_tick`, or `drain_wasapi_output_sink_once`.
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

use super::output_thread_adapter_slot_error::WasapiOutputThreadAdapterSlotError;
use super::output_thread_adapter_slot_report::{
    WasapiOutputThreadAdapterSlotConfig, WasapiOutputThreadAdapterSlotReport,
};
use super::render_step_adapter::run_wasapi_render_step_adapter;
use super::render_step_adapter_report::WasapiRenderStepAdapterConfig;
use crate::playback::output_wasapi::sink::WasapiOutputSink;

/// Execute one output-thread adapter slot on a WasapiOutputSink.
///
/// This is the thin boundary that a future output thread will call.
/// It delegates to `run_wasapi_render_step_adapter` which in turn
/// delegates to `run_manual_drain_render_loop_step` → `manual_drain_tick`
/// → safe drain helper.
///
/// # Behavior
/// - If `config.enabled == false`: delegates to adapter which returns Disabled.
/// - If `config.enabled == true`: delegates to adapter which checks pending/requested.
///
/// # Safety
/// - Does not modify pending_frames on error (adapter/step/tick/drain guarantees this).
/// - Does not consume RingBuffer on error (adapter/step/tick/drain guarantees this).
#[allow(dead_code)] // temporary until P0-074E5 real output-thread wiring
pub(crate) fn run_wasapi_output_thread_adapter_slot(
    sink: &mut WasapiOutputSink,
    config: WasapiOutputThreadAdapterSlotConfig,
) -> Result<WasapiOutputThreadAdapterSlotReport, WasapiOutputThreadAdapterSlotError> {
    let pending_before = sink.runtime.pending_frames;

    // Build the config for the render step adapter
    let adapter_config = WasapiRenderStepAdapterConfig {
        requested_frames: config.requested_frames,
        enabled: config.enabled,
    };

    // Delegate to render step adapter
    let adapter_report = run_wasapi_render_step_adapter(sink, adapter_config)?;

    let pending_after = sink.runtime.pending_frames;

    Ok(WasapiOutputThreadAdapterSlotReport {
        config,
        adapter_report,
        pending_before,
        pending_after,
    })
}
