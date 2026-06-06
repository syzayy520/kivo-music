use crate::playback::decoder::{AudioStreamInfo, DecodedAudioFrame};
use crate::playback::output::AudioOutputFrame;

use super::types::PcmSourceChunk;

pub fn from_decoded_frame(frame: &DecodedAudioFrame) -> PcmSourceChunk<'_> {
    from_parts(
        frame.stream.clone(),
        frame.position_ms,
        &frame.samples,
        false,
    )
}

pub fn from_output_frame(frame: &AudioOutputFrame) -> PcmSourceChunk<'_> {
    from_parts(
        frame.stream.clone(),
        frame.position_ms,
        &frame.samples,
        false,
    )
}

pub fn from_parts<'a>(
    stream: AudioStreamInfo,
    position_ms: u64,
    samples: &'a [f32],
    source_closed: bool,
) -> PcmSourceChunk<'a> {
    PcmSourceChunk {
        stream,
        position_ms,
        samples,
        source_closed,
    }
}
