use crate::playback::audio_bridge::SourceToRingBufferBridgeError;
use crate::playback::audio_bridge::{from_parts, write_pcm_source_chunk_to_ring_buffer};
use crate::playback::decoder::AudioSampleFormat;

use super::fixtures::{float_stream, ring_buffer, ring_buffer_with, ring_format, stream_with};

#[test]
fn invalid_channels_are_rejected() -> Result<(), String> {
    let samples = [];
    let mut buffer = ring_buffer(4)?;
    let chunk = from_parts(
        stream_with(44_100, 0, AudioSampleFormat::Float32),
        0,
        &samples,
        false,
    );

    assert_eq!(
        write_pcm_source_chunk_to_ring_buffer(chunk, &mut buffer),
        Err(SourceToRingBufferBridgeError::InvalidChannels)
    );
    Ok(())
}

#[test]
fn invalid_sample_rate_is_rejected() -> Result<(), String> {
    let samples = [];
    let mut buffer = ring_buffer(4)?;
    let chunk = from_parts(
        stream_with(0, 2, AudioSampleFormat::Float32),
        0,
        &samples,
        false,
    );

    assert_eq!(
        write_pcm_source_chunk_to_ring_buffer(chunk, &mut buffer),
        Err(SourceToRingBufferBridgeError::InvalidSampleRate)
    );
    Ok(())
}

#[test]
fn non_float32_is_rejected() -> Result<(), String> {
    let samples = [];
    let mut buffer = ring_buffer(4)?;
    let chunk = from_parts(
        stream_with(44_100, 2, AudioSampleFormat::Signed16),
        0,
        &samples,
        false,
    );

    assert_eq!(
        write_pcm_source_chunk_to_ring_buffer(chunk, &mut buffer),
        Err(SourceToRingBufferBridgeError::UnsupportedSampleFormat)
    );
    Ok(())
}

#[test]
fn sample_count_must_match_channels() -> Result<(), String> {
    let samples = [0.0, 1.0, 2.0];
    let mut buffer = ring_buffer(4)?;

    assert_eq!(
        write_pcm_source_chunk_to_ring_buffer(
            from_parts(float_stream(), 0, &samples, false),
            &mut buffer
        ),
        Err(SourceToRingBufferBridgeError::InvalidSampleCount)
    );
    Ok(())
}

#[test]
fn target_format_mismatch_is_rejected() -> Result<(), String> {
    let samples = [0.0, 0.0];
    let mut buffer = ring_buffer_with(ring_format(48_000, 2), 4)?;

    assert_eq!(
        write_pcm_source_chunk_to_ring_buffer(
            from_parts(float_stream(), 0, &samples, false),
            &mut buffer
        ),
        Err(SourceToRingBufferBridgeError::FormatMismatch)
    );
    Ok(())
}
