use super::errors::PlaybackResult;
use super::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings, OutputSink};

/// Null output sink for native audio pipeline.
///
/// Consumes decoded frames without producing sound.
/// Used as the output boundary for pipeline testing and as a placeholder
/// until a real output implementation (e.g. WASAPI) is wired in.
///
/// This sink does NOT:
/// - open real audio devices
/// - produce audible output
/// - claim that Kivo can genuinely play audio
#[derive(Clone, Debug, Default)]
pub struct KivoNullOutputSink {
    status: OutputRuntimeStatus,
}

impl KivoNullOutputSink {
    pub fn new() -> Self {
        Self::default()
    }
}

impl OutputSink for KivoNullOutputSink {
    fn open(&mut self, settings: &OutputSettings) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.active_device_id = settings.selected_device_id.clone();
        self.status.is_open = true;
        self.status.is_active = true;
        self.status.last_error = None;
        Ok(self.status.clone())
    }

    fn submit_frame(&mut self, _frame: AudioOutputFrame) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.pending_frames = self.status.pending_frames.saturating_add(1);
        Ok(self.status.clone())
    }

    fn pause(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(self.status.clone())
    }

    fn resume(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(self.status.clone())
    }

    fn flush(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.pending_frames = 0;
        Ok(self.status.clone())
    }

    fn stop(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.is_active = false;
        self.status.pending_frames = 0;
        Ok(self.status.clone())
    }

    fn status(&self) -> OutputRuntimeStatus {
        self.status.clone()
    }

    fn close(&mut self) -> PlaybackResult<()> {
        self.status = OutputRuntimeStatus::default();
        Ok(())
    }
}
