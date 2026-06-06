use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::output_wasapi::frame_bridge::{
    ring_buffer_format_from_stream, FrameBridgeError,
};
use crate::playback::output_wasapi::ring_buffer::{RingBuffer, RingBufferError, RingBufferFormat};

use super::error::SourceToRingBufferBridgeError;
use super::report::SourceToRingBufferBridgeReport;
use super::types::PcmSourceChunk;

pub fn write_pcm_source_chunk_to_ring_buffer(
    chunk: PcmSourceChunk<'_>,
    ring_buffer: &mut RingBuffer,
) -> Result<SourceToRingBufferBridgeReport, SourceToRingBufferBridgeError> {
    let requested_frames = validate_source_chunk(&chunk)?;
    let expected_format = expected_ring_buffer_format(&chunk.stream)?;
    if ring_buffer.format() != expected_format {
        return Err(SourceToRingBufferBridgeError::FormatMismatch);
    }

    if requested_frames == 0 {
        return Ok(report_for_write(&chunk, 0, 0, ring_buffer));
    }
    if ring_buffer.is_full() {
        return Err(SourceToRingBufferBridgeError::BufferFull);
    }

    let bytes = f32_samples_to_le_bytes(chunk.samples)?;
    let frames_written = ring_buffer
        .write_frames(&bytes)
        .map_err(map_ring_buffer_error)?;

    Ok(report_for_write(
        &chunk,
        requested_frames,
        frames_written,
        ring_buffer,
    ))
}

fn validate_source_chunk(chunk: &PcmSourceChunk<'_>) -> Result<u32, SourceToRingBufferBridgeError> {
    if chunk.stream.channels == 0 {
        return Err(SourceToRingBufferBridgeError::InvalidChannels);
    }
    if chunk.stream.sample_rate_hz == 0 {
        return Err(SourceToRingBufferBridgeError::InvalidSampleRate);
    }
    match &chunk.stream.sample_format {
        AudioSampleFormat::Float32 => {}
        _ => return Err(SourceToRingBufferBridgeError::UnsupportedSampleFormat),
    }

    let channels = usize::from(chunk.stream.channels);
    if !chunk.samples.len().is_multiple_of(channels) {
        return Err(SourceToRingBufferBridgeError::InvalidSampleCount);
    }
    u32::try_from(chunk.samples.len() / channels)
        .map_err(|_| SourceToRingBufferBridgeError::ByteLengthOverflow)
}

fn expected_ring_buffer_format(
    stream: &AudioStreamInfo,
) -> Result<RingBufferFormat, SourceToRingBufferBridgeError> {
    ring_buffer_format_from_stream(stream).map_err(|error| match error {
        FrameBridgeError::ZeroChannels => SourceToRingBufferBridgeError::InvalidChannels,
        FrameBridgeError::ZeroSampleRate => SourceToRingBufferBridgeError::InvalidSampleRate,
        FrameBridgeError::UnsupportedSampleFormat => {
            SourceToRingBufferBridgeError::UnsupportedSampleFormat
        }
        FrameBridgeError::ZeroBlockAlign | FrameBridgeError::NonSilentFrameRejected => {
            SourceToRingBufferBridgeError::ByteLengthOverflow
        }
    })
}

fn f32_samples_to_le_bytes(samples: &[f32]) -> Result<Vec<u8>, SourceToRingBufferBridgeError> {
    let byte_len = samples
        .len()
        .checked_mul(4)
        .ok_or(SourceToRingBufferBridgeError::ByteLengthOverflow)?;
    let mut bytes = Vec::with_capacity(byte_len);
    for sample in samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    Ok(bytes)
}

fn report_for_write(
    chunk: &PcmSourceChunk<'_>,
    requested_frames: u32,
    frames_written: u32,
    ring_buffer: &RingBuffer,
) -> SourceToRingBufferBridgeReport {
    let bytes_written = usize::try_from(frames_written)
        .ok()
        .and_then(|frames| frames.checked_mul(usize::from(ring_buffer.format().block_align)))
        .unwrap_or(0);
    let rejected_frames = requested_frames.saturating_sub(frames_written);

    SourceToRingBufferBridgeReport {
        requested_frames,
        accepted_frames: frames_written,
        rejected_frames,
        bytes_written,
        frames_written,
        pending_frames_after_write: ring_buffer.available_frames(),
        source_closed: chunk.source_closed,
        ring_buffer_full: ring_buffer.is_full(),
        format_validated: true,
        partial_write: frames_written < requested_frames,
    }
}

fn map_ring_buffer_error(error: RingBufferError) -> SourceToRingBufferBridgeError {
    match error {
        RingBufferError::WouldBlock => SourceToRingBufferBridgeError::BufferFull,
        other => SourceToRingBufferBridgeError::RingBufferWriteFailed(other),
    }
}
