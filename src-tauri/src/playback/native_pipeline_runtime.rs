use super::errors::{PlaybackError, PlaybackResult};
use super::native_pipeline::NativePipeline;
use super::output::OutputSink;

impl NativePipeline {
    pub fn start(&mut self) -> PlaybackResult<()> {
        match self.output.open(&self.state.output_settings) {
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

    pub fn submit(&mut self) -> PlaybackResult<()> {
        self.schedule_output_submit_step()
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

    pub fn pause_output(&mut self) -> PlaybackResult<()> {
        match self.output.pause() {
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

    pub fn resume_output(&mut self) -> PlaybackResult<()> {
        match self.output.resume() {
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

    pub fn stop_output(&mut self) -> PlaybackResult<()> {
        match self.output.stop() {
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

    pub fn flush_output(&mut self) -> PlaybackResult<()> {
        match self.output.flush() {
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
