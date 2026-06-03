use super::errors::{PlaybackError, PlaybackResult};
use super::native_pipeline::NativePipeline;
use super::output::OutputSink;

impl NativePipeline {
    /// Drain next frame from buffer to output sink.
    ///
    /// Pops one frame from the pipeline buffer and submits it to the output sink.
    /// Returns error if buffer is empty (no frame to drain).
    /// This method does NOT:
    /// - decode frames
    /// - open real audio devices
    /// - know about engine/manager/UI
    pub(in crate::playback) fn drain_next_frame_to_output(&mut self) -> PlaybackResult<()> {
        let frame = self.buffer.drain_next_frame().ok_or_else(|| {
            PlaybackError::Backend("pipeline buffer is empty, no frame to drain".to_string())
        })?;

        let drained_position_ms = frame.position_ms;

        match self.output.submit_frame(frame) {
            Ok(status) => {
                self.state.output_status = status;
                if self.clock.is_started() {
                    self.clock.set_position(drained_position_ms);
                } else {
                    self.clock.start_at(drained_position_ms);
                }
                Ok(())
            }
            Err(error) => {
                self.state.output_status = self.output.status();
                Err(error)
            }
        }
    }

    /// Enqueue a decoded frame into the pipeline buffer.
    pub(in crate::playback) fn enqueue_decoded_frame(
        &mut self,
        frame: super::output::AudioOutputFrame,
    ) {
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
