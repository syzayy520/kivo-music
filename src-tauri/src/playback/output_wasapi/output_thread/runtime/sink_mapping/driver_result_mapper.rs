//! Driver result mapper.
//!
//! Pure function mapping DriverResult to SinkRequest.
//! No behavior, no IO, no WASAPI.

use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;
use crate::playback::output_wasapi::output_thread::sink_boundary::SinkRequest;

/// Map a DriverResult to a SinkRequest.
///
/// This is a pure memory mapping — no side effects, no IO.
pub fn map_driver_result_to_sink_request(
    result: DriverResult,
    frame_count: u64,
    sample_rate: u32,
    channel_count: u16,
) -> SinkRequest {
    match result {
        DriverResult::Continue => SinkRequest::Render {
            frame_count,
            sample_rate,
            channel_count,
        },
        DriverResult::Idle => SinkRequest::Noop,
        DriverResult::Stop => SinkRequest::Flush,
        DriverResult::Error => SinkRequest::Noop,
    }
}
