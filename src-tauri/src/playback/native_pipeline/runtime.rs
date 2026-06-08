use crate::playback::errors::PlaybackResult;
use crate::playback::native_pipeline::NativePipeline;
use crate::playback::output::OutputSink;

impl NativePipeline {
    pub fn start(&mut self) -> PlaybackResult<()> {
        match self.output.open(&self.state.output_settings) {
            Ok(status) => {
                self.state.output_status = status;
                self.clock.start_at(self.clock.position_ms());
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
        self.drain_next_frame_to_output()
    }

    pub fn pause_output(&mut self) -> PlaybackResult<()> {
        match self.output.pause() {
            Ok(status) => {
                self.state.output_status = status;
                self.clock.pause();
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
                self.clock.resume();
                Ok(())
            }
            Err(error) => {
                self.state.output_status = self.output.status();
                Err(error)
            }
        }
    }

    pub fn stop_output(&mut self) -> PlaybackResult<()> {
        self.clear_buffer();
        match self.output.stop() {
            Ok(status) => {
                self.state.output_status = status;
                self.clock.reset();
                Ok(())
            }
            Err(error) => {
                self.state.output_status = self.output.status();
                Err(error)
            }
        }
    }

    pub fn flush_output(&mut self) -> PlaybackResult<()> {
        self.clear_buffer();
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

    pub fn set_output_volume(&mut self, level: f32) -> PlaybackResult<()> {
        match self.output.set_volume(level) {
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

    pub fn set_output_muted(&mut self, muted: bool) -> PlaybackResult<()> {
        match self.output.set_muted(muted) {
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
        self.clear_buffer();
        self.close_decoder()?;
        self.output.close()?;
        self.state.output_status = self.output.status();
        self.clock.reset();
        Ok(())
    }
}
