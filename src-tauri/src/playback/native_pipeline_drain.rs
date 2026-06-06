mod clock;
mod error;
mod frame;
mod status;
mod submit;

use super::errors::PlaybackResult;
use super::native_pipeline::NativePipeline;

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
        let frame = self.drain_output_frame()?;
        let drained_position_ms = frame.position_ms;

        match self.submit_drained_output_frame(frame) {
            Ok(status) => {
                self.apply_drain_submit_success_status(status);
                self.apply_drain_success_clock_position(drained_position_ms);
                Ok(())
            }
            Err(error) => error::handle_drain_submit_error(self, error),
        }
    }
}
