//! Sink-to-source mapper.
//!
//! Pure function mapping SinkRequest to RenderSourceRequest.
//! No behavior, no IO, no WASAPI.

use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::RenderSourceRequest;
use crate::playback::output_wasapi::output_thread::sink_boundary::SinkRequest;

/// Map a SinkRequest to a RenderSourceRequest.
///
/// When the sink needs frames, the source should read packets.
/// When the sink needs silence, the source has no work (Noop).
/// This is a pure memory mapping — no side effects, no IO.
pub fn map_sink_request_to_source_request(request: &SinkRequest) -> RenderSourceRequest {
    match request {
        SinkRequest::Render {
            frame_count,
            sample_rate,
            channel_count,
        } => RenderSourceRequest::ReadPacket {
            frame_count: *frame_count,
            sample_rate: *sample_rate,
            channel_count: *channel_count,
        },
        SinkRequest::WriteSilence { .. } => RenderSourceRequest::Noop,
        SinkRequest::Flush => RenderSourceRequest::Flush,
        SinkRequest::Noop => RenderSourceRequest::Noop,
    }
}
