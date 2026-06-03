use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings, OutputSink};

use super::config::WasapiOutputConfig;
use super::errors::wasapi_unsupported;
use super::status::WasapiOutputStatus;

/// WASAPI output sink stub for the native audio pipeline.
///
/// This is a placeholder implementation that does NOT:
/// - Open real audio devices
/// - Produce audible output
/// - Create output threads
/// - Use Windows audio APIs
/// - Claim that Kivo can genuinely play audio via WASAPI
///
/// All methods return `PlaybackError::UnsupportedOperation` except `close()` and `status()`.
#[derive(Clone, Debug, Default)]
pub struct WasapiOutputSink {
    /// Current stub configuration.
    config: WasapiOutputConfig,
    /// Current stub status.
    status: WasapiOutputStatus,
}

impl WasapiOutputSink {
    /// Create a new WASAPI output sink stub.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get current stub configuration.
    pub fn config(&self) -> &WasapiOutputConfig {
        &self.config
    }

    /// Get current stub status.
    pub fn wasapi_status(&self) -> &WasapiOutputStatus {
        &self.status
    }
}

impl OutputSink for WasapiOutputSink {
    fn open(&mut self, settings: &OutputSettings) -> PlaybackResult<OutputRuntimeStatus> {
        self.config = WasapiOutputConfig::from_output_settings(settings);
        self.status.is_open_attempted = true;
        self.status.last_error = Some(wasapi_unsupported("open"));

        Err(PlaybackError::UnsupportedOperation(wasapi_unsupported(
            "open",
        )))
    }

    fn submit_frame(&mut self, _frame: AudioOutputFrame) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.last_error = Some(wasapi_unsupported("submit_frame"));

        Err(PlaybackError::UnsupportedOperation(wasapi_unsupported(
            "submit_frame",
        )))
    }

    fn pause(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.last_error = Some(wasapi_unsupported("pause"));

        Err(PlaybackError::UnsupportedOperation(wasapi_unsupported(
            "pause",
        )))
    }

    fn resume(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.last_error = Some(wasapi_unsupported("resume"));

        Err(PlaybackError::UnsupportedOperation(wasapi_unsupported(
            "resume",
        )))
    }

    fn flush(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.last_error = Some(wasapi_unsupported("flush"));

        Err(PlaybackError::UnsupportedOperation(wasapi_unsupported(
            "flush",
        )))
    }

    fn stop(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.last_error = Some(wasapi_unsupported("stop"));

        Err(PlaybackError::UnsupportedOperation(wasapi_unsupported(
            "stop",
        )))
    }

    fn set_volume(&mut self, _level: f32) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.last_error = Some(wasapi_unsupported("set_volume"));

        Err(PlaybackError::UnsupportedOperation(wasapi_unsupported(
            "set_volume",
        )))
    }

    fn set_muted(&mut self, _muted: bool) -> PlaybackResult<OutputRuntimeStatus> {
        self.status.last_error = Some(wasapi_unsupported("set_muted"));

        Err(PlaybackError::UnsupportedOperation(wasapi_unsupported(
            "set_muted",
        )))
    }

    fn status(&self) -> OutputRuntimeStatus {
        self.status
            .to_output_runtime_status(&self.config.selected_device_id)
    }

    fn close(&mut self) -> PlaybackResult<()> {
        // Close is idempotent and always succeeds for the stub.
        // Does not interact with real devices.
        self.status = WasapiOutputStatus::default();
        self.config = WasapiOutputConfig::default();
        Ok(())
    }
}
