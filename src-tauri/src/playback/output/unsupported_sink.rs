use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings, OutputSink};

#[derive(Clone, Debug, Default)]
pub struct UnsupportedOutputSink {
    status: OutputRuntimeStatus,
}

impl UnsupportedOutputSink {
    pub fn new() -> Self {
        Self::default()
    }

    fn unsupported(&mut self, operation: &str) -> PlaybackResult<OutputRuntimeStatus> {
        let message = format!("output sink operation is not implemented: {operation}");
        self.status.last_error = Some(message.clone());
        Err(PlaybackError::UnsupportedOperation(message))
    }
}

impl OutputSink for UnsupportedOutputSink {
    fn open(&mut self, settings: &OutputSettings) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.active_device_id = settings.selected_device_id.clone();
        self.unsupported("open")
    }

    fn submit_frame(&mut self, _frame: AudioOutputFrame) -> PlaybackResult<OutputRuntimeStatus> {
        self.unsupported("submit_frame")
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

    fn set_volume(&mut self, level: f32) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.controls.volume_level = level.clamp(0.0, 1.0);
        self.unsupported("set_volume")
    }

    fn set_muted(&mut self, muted: bool) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.controls.muted = muted;
        self.unsupported("set_muted")
    }

    fn status(&self) -> OutputRuntimeStatus {
        self.status.clone()
    }

    fn close(&mut self) -> PlaybackResult<()> {
        self.status = OutputRuntimeStatus::default();
        Ok(())
    }
}
