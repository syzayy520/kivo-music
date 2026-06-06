use crate::playback::decoder::AudioStreamInfo;
use crate::playback::output_wasapi::frame_bridge::ring_buffer_format_from_stream;
use crate::playback::output_wasapi::ring_buffer::RingBufferFormat;

use super::error::AudioRouteError;

pub fn derive_route_ring_buffer_format(
    stream: &AudioStreamInfo,
) -> Result<RingBufferFormat, AudioRouteError> {
    ring_buffer_format_from_stream(stream).map_err(|_| AudioRouteError::InvalidFormat)
}

pub(super) fn ensure_route_format(
    stream: &AudioStreamInfo,
    format: RingBufferFormat,
) -> Result<(), AudioRouteError> {
    if derive_route_ring_buffer_format(stream)? == format {
        Ok(())
    } else {
        Err(AudioRouteError::FormatMismatch)
    }
}
