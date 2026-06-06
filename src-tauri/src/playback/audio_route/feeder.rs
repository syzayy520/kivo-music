use crate::playback::audio_bridge::{
    from_decoded_frame, from_output_frame, write_pcm_source_chunk_to_ring_buffer, PcmSourceChunk,
    SourceToRingBufferBridgeError,
};
use crate::playback::decoder::DecodedAudioFrame;
use crate::playback::output::AudioOutputFrame;
use crate::playback::output_wasapi::ring_buffer::RingBufferError;

use super::error::AudioRouteError;
use super::format::ensure_route_format;
use super::owner::AudioRouteOwner;
use super::report::AudioRouteReport;

pub(super) fn feed_pcm_source_chunk(
    owner: &mut AudioRouteOwner,
    chunk: PcmSourceChunk<'_>,
) -> Result<AudioRouteReport, AudioRouteError> {
    if owner.is_closed() {
        return Err(AudioRouteError::Closed);
    }
    ensure_route_format(&chunk.stream, owner.ring_buffer.format())?;

    let bridge_report = write_pcm_source_chunk_to_ring_buffer(chunk, &mut owner.ring_buffer)
        .map_err(map_bridge_error)?;
    owner.report.record_bridge_report(&bridge_report);
    Ok(owner.report)
}

pub(super) fn feed_decoded_frame(
    owner: &mut AudioRouteOwner,
    frame: &DecodedAudioFrame,
) -> Result<AudioRouteReport, AudioRouteError> {
    feed_pcm_source_chunk(owner, from_decoded_frame(frame))
}

pub(super) fn feed_output_frame(
    owner: &mut AudioRouteOwner,
    frame: &AudioOutputFrame,
) -> Result<AudioRouteReport, AudioRouteError> {
    feed_pcm_source_chunk(owner, from_output_frame(frame))
}

fn map_bridge_error(error: SourceToRingBufferBridgeError) -> AudioRouteError {
    match error {
        SourceToRingBufferBridgeError::FormatMismatch => AudioRouteError::FormatMismatch,
        SourceToRingBufferBridgeError::RingBufferWriteFailed(RingBufferError::Closed) => {
            AudioRouteError::Closed
        }
        other => AudioRouteError::Bridge(other),
    }
}
