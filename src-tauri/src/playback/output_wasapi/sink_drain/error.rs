//! Error types for RingBuffer drain operations.

use crate::playback::output_wasapi::ring_buffer::errors::RingBufferError;
use crate::playback::output_wasapi::wasapi_context::WasapiRenderWriteError;

/// Error from a RingBuffer drain operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WasapiRingBufferDrainError {
    /// RingBuffer peek or consume failed.
    RingBuffer(RingBufferError),
    /// WASAPI render write failed.
    RenderWrite(WasapiRenderWriteError),
    /// pending_frames would underflow if consumed.
    PendingFrameUnderflow { pending: usize, consumed: u32 },
    /// Writer reported different frames than peeked.
    RenderedFrameMismatch { peeked: u32, rendered: u32 },
    /// Writer reported different bytes than expected.
    RenderedByteMismatch { expected: usize, rendered: usize },
}

impl From<RingBufferError> for WasapiRingBufferDrainError {
    fn from(e: RingBufferError) -> Self {
        Self::RingBuffer(e)
    }
}

impl From<WasapiRenderWriteError> for WasapiRingBufferDrainError {
    fn from(e: WasapiRenderWriteError) -> Self {
        Self::RenderWrite(e)
    }
}
