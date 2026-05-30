use super::errors::{PlaybackError, PlaybackResult};
use super::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings, OutputSink};

#[derive(Clone, Debug, Default)]
pub struct KivoNativeOutputSink {
    status: OutputRuntimeStatus,
}

impl KivoNativeOutputSink {
    pub fn new() -> Self {
        Self::default()
    }

    fn unsupported(&mut self, operation: &str) -> PlaybackResult<OutputRuntimeStatus> {
        let message = format!("kivo native output {operation} is not implemented yet");
        self.status.last_error = Some(message.clone());
        Err(PlaybackError::UnsupportedOperation(message))
    }
}

impl OutputSink for KivoNativeOutputSink {
    fn open(&mut self, settings: &OutputSettings) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.active_device_id = settings.selected_device_id.clone();
        self.unsupported("open")
    }

    fn submit_frame(&mut self, _frame: AudioOutputFrame) -> PlaybackResult<OutputRuntimeStatus> {
        self.unsupported("submit frame")
    }

    fn pause(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.unsupported("pause")
    }

    fn resume(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.unsupported("resume")
    }

    fn flush(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.unsupported("flush")
    }

    fn stop(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.is_active = false;
        self.unsupported("stop")
    }

    fn status(&self) -> OutputRuntimeStatus {
        self.status.clone()
    }

    fn close(&mut self) -> PlaybackResult<()> {
        self.status = OutputRuntimeStatus::default();
        Ok(())
    }
}
