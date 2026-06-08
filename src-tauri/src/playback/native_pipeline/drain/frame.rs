use crate::playback::errors::PlaybackResult;
use crate::playback::native_pipeline::NativePipeline;
use crate::playback::output::AudioOutputFrame;

use super::error;

impl NativePipeline {
    pub(in crate::playback) fn drain_output_frame(&mut self) -> PlaybackResult<AudioOutputFrame> {
        self.buffer
            .drain_next_frame()
            .ok_or_else(error::empty_buffer_error)
    }

    /// Enqueue a decoded frame into the pipeline buffer.
    pub(in crate::playback) fn enqueue_decoded_frame(&mut self, frame: AudioOutputFrame) {
        self.buffer.enqueue_frame(frame);
    }

    /// Clear the pipeline buffer.
    pub(in crate::playback) fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    /// Get the number of buffered frames.
    #[allow(dead_code)]
    pub(in crate::playback) fn buffered_frame_count(&self) -> usize {
        self.buffer.len()
    }
}
