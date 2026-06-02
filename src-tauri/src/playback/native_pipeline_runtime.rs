use super::errors::{PlaybackError, PlaybackResult};
use super::native_pipeline::NativePipeline;
use super::output::OutputSink;

impl NativePipeline {
    pub fn start(&mut self) -> PlaybackResult<()> {
        Err(PlaybackError::UnsupportedOperation(
            "native pipeline start is not implemented yet".to_string(),
        ))
    }

    pub fn submit(&mut self) -> PlaybackResult<()> {
        Err(PlaybackError::UnsupportedOperation(
            "native pipeline submit is not implemented yet".to_string(),
        ))
    }

    pub fn schedule_output_submit_step(&mut self) -> PlaybackResult<()> {
        let frame = self.state.last_decoded_frame.clone().ok_or_else(|| {
            PlaybackError::Backend("native pipeline output frame is not ready".to_string())
        })?;

        match self.output.submit_frame(frame) {
            Ok(status) => {
                self.state.output_status = status;
                Ok(())
            }
            Err(error) => {
                self.state.output_status = self.output.status();
                Err(error)
            }
        }
    }

    pub fn shutdown(&mut self) -> PlaybackResult<()> {
        self.close_decoder()?;
        self.output.close()?;
        self.state.output_status = self.output.status();
        Ok(())
    }
}
