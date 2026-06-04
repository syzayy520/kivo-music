use crate::playback::output::AudioOutputFrame;
use crate::playback::output_wasapi::ring_buffer::buffer::RingBuffer;
use crate::playback::output_wasapi::ring_buffer::errors::RingBufferError;

use super::format_mapper::{ring_buffer_format_from_stream, FrameBridgeError};
use super::silent_guard::ensure_silent_frame;

/// Errors from silent ring buffer write operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SilentWriteError {
    /// Frame bridge validation failed.
    Bridge(FrameBridgeError),
    /// Ring buffer operation failed.
    Buffer(RingBufferError),
}

impl From<FrameBridgeError> for SilentWriteError {
    fn from(e: FrameBridgeError) -> Self {
        SilentWriteError::Bridge(e)
    }
}

impl From<RingBufferError> for SilentWriteError {
    fn from(e: RingBufferError) -> Self {
        SilentWriteError::Buffer(e)
    }
}

/// Stateless helper that validates and converts silent Float32 frames
/// into RingBuffer byte writes.
///
/// Only accepts silent Float32 frames. Byte conversion uses native
/// endianness (`f32::to_ne_bytes`) matching WASAPI expectations.
pub struct SilentRingBufferWriter;

impl SilentRingBufferWriter {
    /// Write a silent frame into the ring buffer.
    ///
    /// Returns the number of frames written, or an error if:
    /// - The frame is not silent (`NonSilentFrameRejected`)
    /// - The stream format is unsupported or invalid
    /// - The ring buffer is full (`WouldBlock`) or closed (`Closed`)
    pub fn write_silent_frame(
        frame: &AudioOutputFrame,
        ring_buffer: &mut RingBuffer,
    ) -> Result<u32, SilentWriteError> {
        // 1. Validate frame is silent
        ensure_silent_frame(frame)?;

        // 2. Validate stream format maps to RingBufferFormat
        let _format = ring_buffer_format_from_stream(&frame.stream)?;

        // 3. Convert f32 samples to bytes (native endianness)
        let mut bytes = Vec::with_capacity(frame.samples.len() * 4);
        for &sample in &frame.samples {
            bytes.extend_from_slice(&sample.to_ne_bytes());
        }

        // 4. Write to ring buffer
        let frames_written = ring_buffer.write_frames(&bytes)?;

        Ok(frames_written)
    }
}
