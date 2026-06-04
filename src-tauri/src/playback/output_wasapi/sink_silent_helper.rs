use crate::playback::output::AudioOutputFrame;
use crate::playback::output_wasapi::ring_buffer::buffer::RingBuffer;

use super::frame_bridge::{SilentRingBufferWriter, SilentWriteError};

/// Errors from scaffold silent write operations.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaffoldSilentWriteError {
    /// Ring buffer is not prepared (None).
    NoRingBuffer,
    /// Silent write validation or buffer operation failed.
    SilentWrite(SilentWriteError),
}

impl From<SilentWriteError> for ScaffoldSilentWriteError {
    fn from(e: SilentWriteError) -> Self {
        ScaffoldSilentWriteError::SilentWrite(e)
    }
}

/// Stateless scaffold helper that writes silent frames into a prepared RingBuffer.
///
/// This is an internal Output Layer helper. It does NOT:
/// - Create or prepare RingBuffers
/// - Modify `submit_frame` behavior
/// - Modify `pending_frames` or `last_error`
/// - Open Windows audio devices
/// - Start output threads
/// - Represent real playback
#[allow(dead_code)]
pub(crate) struct ScaffoldSilentWriter;

impl ScaffoldSilentWriter {
    /// Write a silent frame into the prepared ring buffer.
    ///
    /// Returns the number of frames written, or an error if:
    /// - Ring buffer is not prepared (`NoRingBuffer`)
    /// - Frame is not silent (`SilentWrite(NonSilentFrameRejected)`)
    /// - Stream format is unsupported (`SilentWrite(...)`)
    /// - Ring buffer is full or closed (`SilentWrite(...)`)
    #[allow(dead_code)]
    pub(crate) fn write_silent_frame_to_prepared_buffer(
        frame: &AudioOutputFrame,
        ring_buffer: &mut Option<RingBuffer>,
    ) -> Result<u32, ScaffoldSilentWriteError> {
        let rb = ring_buffer
            .as_mut()
            .ok_or(ScaffoldSilentWriteError::NoRingBuffer)?;
        let written = SilentRingBufferWriter::write_silent_frame(frame, rb)?;
        Ok(written)
    }
}
