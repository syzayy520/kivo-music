use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::output_wasapi::ring_buffer::types::RingBufferFormat;

/// Errors specific to frame bridge operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameBridgeError {
    /// Sample format is not Float32.
    UnsupportedSampleFormat,
    /// Channels is zero.
    ZeroChannels,
    /// Sample rate is zero.
    ZeroSampleRate,
    /// Calculated block_align is zero (should not happen if channels > 0).
    ZeroBlockAlign,
    /// Frame contains non-silent samples.
    NonSilentFrameRejected,
}

/// Map AudioStreamInfo to RingBufferFormat for Float32 samples.
///
/// Only Float32 is supported. For other formats, returns UnsupportedSampleFormat.
pub fn ring_buffer_format_from_stream(
    stream: &AudioStreamInfo,
) -> Result<RingBufferFormat, FrameBridgeError> {
    if stream.channels == 0 {
        return Err(FrameBridgeError::ZeroChannels);
    }
    if stream.sample_rate_hz == 0 {
        return Err(FrameBridgeError::ZeroSampleRate);
    }
    match stream.sample_format {
        AudioSampleFormat::Float32 => {}
        _ => return Err(FrameBridgeError::UnsupportedSampleFormat),
    }

    let bits_per_sample = 32u16;
    let block_align = stream
        .channels
        .checked_mul(4)
        .ok_or(FrameBridgeError::ZeroBlockAlign)?;

    Ok(RingBufferFormat {
        sample_rate_hz: stream.sample_rate_hz,
        channels: stream.channels,
        bits_per_sample,
        block_align,
    })
}
