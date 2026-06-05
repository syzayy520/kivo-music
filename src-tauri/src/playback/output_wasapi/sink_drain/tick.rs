//! Manual drain tick boundary for future render loop wiring.

use super::drain_once::drain_wasapi_output_sink_once;
use super::tick_error::WasapiDrainTickError;
use super::tick_report::{WasapiDrainTickReport, WasapiDrainTickSkipReason};
use crate::playback::output_wasapi::sink::WasapiOutputSink;

/// Execute one manual drain tick on a WasapiOutputSink.
///
/// This is the boundary that future output thread / render loop
/// will call. It does NOT:
/// - Create threads
/// - Call IAudioClient::Start
/// - Call GetBuffer / ReleaseBuffer directly
/// - Auto-run from submit_frame / open / play / flush / stop / close
/// - Modify NativePipeline
///
/// # Skip conditions
/// - `requested_frames == 0`: returns no-op report with `RequestedZero`.
/// - `pending_frames == 0`: returns no-op report with `NoPendingFrames`.
///
/// # Drain condition
/// If both `requested_frames > 0` and `pending_frames > 0`:
/// - Calls `drain_wasapi_output_sink_once`.
/// - On success: returns report with `attempted=true`, `skipped_reason=None`.
/// - On error: returns `WasapiDrainTickError::Drain(error)`.
///
/// # Safety
/// - Does not modify pending_frames on error (drain helper guarantees this).
/// - Does not consume RingBuffer on error (drain helper guarantees this).
#[allow(dead_code)] // temporary until P0-074E2 render loop wiring
pub(crate) fn manual_drain_tick(
    sink: &mut WasapiOutputSink,
    requested_frames: u32,
) -> Result<WasapiDrainTickReport, WasapiDrainTickError> {
    let pending_before = sink.runtime.pending_frames;

    // Skip: requested_frames == 0
    if requested_frames == 0 {
        return Ok(WasapiDrainTickReport {
            requested_frames,
            attempted: false,
            skipped_reason: WasapiDrainTickSkipReason::RequestedZero,
            drain_report: None,
            pending_before,
            pending_after: pending_before,
        });
    }

    // Skip: pending_frames == 0
    if pending_before == 0 {
        return Ok(WasapiDrainTickReport {
            requested_frames,
            attempted: false,
            skipped_reason: WasapiDrainTickSkipReason::NoPendingFrames,
            drain_report: None,
            pending_before,
            pending_after: pending_before,
        });
    }

    // Attempt drain
    let drain_report = drain_wasapi_output_sink_once(sink, requested_frames)?;
    let pending_after = sink.runtime.pending_frames;

    Ok(WasapiDrainTickReport {
        requested_frames,
        attempted: true,
        skipped_reason: WasapiDrainTickSkipReason::None,
        drain_report: Some(drain_report),
        pending_before,
        pending_after,
    })
}
