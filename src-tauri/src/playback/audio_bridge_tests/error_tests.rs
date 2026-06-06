use crate::playback::audio_bridge::SourceToRingBufferBridgeError;
use crate::playback::audio_bridge::{from_parts, write_pcm_source_chunk_to_ring_buffer};
use crate::playback::decoder::AudioSampleFormat;
use crate::playback::output_wasapi::ring_buffer::RingBufferError;

use super::fixtures::{chunk, ring_buffer, stream_with};

#[test]
fn byte_length_overflow_is_guarded() -> Result<(), String> {
    let samples = [];
    let mut buffer = ring_buffer(4)?;
    let stream = stream_with(44_100, 20_000, AudioSampleFormat::Float32);

    assert_eq!(
        write_pcm_source_chunk_to_ring_buffer(from_parts(stream, 0, &samples, false), &mut buffer),
        Err(SourceToRingBufferBridgeError::ByteLengthOverflow)
    );
    Ok(())
}

#[test]
fn closed_ring_buffer_error_is_explicit() -> Result<(), String> {
    let samples = [0.0, 0.0];
    let mut buffer = ring_buffer(4)?;
    buffer.close();

    assert_eq!(
        write_pcm_source_chunk_to_ring_buffer(chunk(&samples), &mut buffer),
        Err(SourceToRingBufferBridgeError::RingBufferWriteFailed(
            RingBufferError::Closed
        ))
    );
    Ok(())
}
