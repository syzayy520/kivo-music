//! Source-to-sink mapper.
//!
//! Pure function mapping RenderSourceResult to SinkRequest.
//! No behavior, no IO, no WASAPI.

use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::RenderSourceResult;
use crate::playback::output_wasapi::output_thread::sink_boundary::SinkRequest;

/// Map a RenderSourceResult to a SinkRequest.
///
/// When the source provides frames, the sink should render them.
/// When the source is exhausted, the sink should flush.
/// This is a pure memory mapping — no side effects, no IO.
pub fn map_source_result_to_sink_request(
    result: &RenderSourceResult,
    sample_rate: u32,
    channel_count: u16,
) -> SinkRequest {
    match result {
        RenderSourceResult::Packet {
            frames_provided, ..
        } => SinkRequest::Render {
            frame_count: *frames_provided,
            sample_rate,
            channel_count,
        },
        RenderSourceResult::Exhausted => SinkRequest::Flush,
        RenderSourceResult::Skipped => SinkRequest::Noop,
        RenderSourceResult::Noop => SinkRequest::Noop,
    }
}
