use crate::playback::output::OutputRuntimeStatus;

/// Status of the WASAPI output sink stub.
///
/// This is a stub status that does not represent real audio device state.
/// All real device capabilities are marked as unavailable.
#[derive(Clone, Debug, Default)]
pub struct WasapiOutputStatus {
    /// Whether `open()` has been called (even if it failed).
    pub is_open_attempted: bool,
    /// Whether a real audio device is open and ready.
    ///
    /// **This field is ALWAYS `false` in the stub implementation.**
    /// It exists only as a placeholder for future WASAPI implementation.
    pub is_real_device_open: bool,
    /// Number of frames submitted (stub always stays 0).
    pub submitted_frames: u64,
    /// Last error message from stub operations.
    pub last_error: Option<String>,
}

impl WasapiOutputStatus {
    /// Convert to pipeline-compatible `OutputRuntimeStatus`.
    ///
    /// The stub always reports `is_open: false` and `is_active: false`.
    pub fn to_output_runtime_status(
        &self,
        selected_device_id: &Option<String>,
    ) -> OutputRuntimeStatus {
        OutputRuntimeStatus {
            is_open: false,   // stub never opens real device
            is_active: false, // stub never activates real device
            active_device_id: selected_device_id.clone(),
            pending_frames: 0,
            latency: Default::default(),
            controls: Default::default(),
            gap_count: 0,
            last_error: self.last_error.clone(),
        }
    }
}
