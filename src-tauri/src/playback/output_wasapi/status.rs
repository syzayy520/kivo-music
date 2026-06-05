use crate::playback::output::OutputRuntimeStatus;

/// Status of the WASAPI output sink.
///
/// Tracks the state of real WASAPI device initialization.
#[derive(Clone, Debug, Default)]
pub struct WasapiOutputStatus {
    /// Whether `open()` has been called (even if it failed).
    pub is_open_attempted: bool,
    /// Whether a real audio device is open and ready.
    ///
    /// On Windows: true when IAudioClient is initialized.
    /// On non-Windows: always false.
    pub is_real_device_open: bool,
    /// Whether IAudioRenderClient has been acquired.
    ///
    /// On Windows: true when GetService(IAudioRenderClient) succeeds.
    /// On non-Windows: always false.
    pub is_render_client_acquired: bool,
    /// Number of frames submitted (stub always stays 0).
    pub submitted_frames: u64,
    /// Last error message from operations.
    pub last_error: Option<String>,
}

impl WasapiOutputStatus {
    /// Convert to pipeline-compatible `OutputRuntimeStatus`.
    pub fn to_output_runtime_status(
        &self,
        selected_device_id: &Option<String>,
    ) -> OutputRuntimeStatus {
        OutputRuntimeStatus {
            is_open: self.is_real_device_open,
            is_active: self.is_render_client_acquired,
            active_device_id: selected_device_id.clone(),
            pending_frames: 0,
            latency: Default::default(),
            controls: Default::default(),
            gap_count: 0,
            last_error: self.last_error.clone(),
        }
    }

    /// Mark status as successfully opened with render client.
    pub(crate) fn mark_opened_with_render_client(&mut self) {
        self.is_open_attempted = true;
        self.is_real_device_open = true;
        self.is_render_client_acquired = true;
        self.last_error = None;
    }

    /// Mark status as open attempted but failed.
    pub(crate) fn mark_open_failed(&mut self, error: String) {
        self.is_open_attempted = true;
        self.is_real_device_open = false;
        self.is_render_client_acquired = false;
        self.last_error = Some(error);
    }

    /// Mark status as unsupported platform.
    #[allow(dead_code)]
    pub(crate) fn mark_unsupported_platform(&mut self) {
        self.is_open_attempted = true;
        self.is_real_device_open = false;
        self.is_render_client_acquired = false;
        self.last_error = Some("WASAPI not supported on this platform".to_string());
    }

    /// Reset status to closed state.
    pub(crate) fn mark_closed(&mut self) {
        self.is_real_device_open = false;
        self.is_render_client_acquired = false;
        self.last_error = None;
    }
}
