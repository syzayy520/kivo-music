use crate::playback::output::OutputSettings;

/// Configuration for the WASAPI output sink.
///
/// This is a stub configuration that does not interact with real audio devices.
/// It only stores the intended settings for future WASAPI implementation.
#[derive(Clone, Debug, Default)]
pub struct WasapiOutputConfig {
    /// Target device identifier. `None` means system default.
    pub selected_device_id: Option<String>,
    /// Whether to use WASAPI exclusive mode (AUDCLNT_SHAREMODE_EXCLUSIVE).
    pub exclusive_mode: bool,
    /// Whether to request bit-perfect output (implies exclusive mode).
    pub bit_perfect_mode: bool,
}

impl WasapiOutputConfig {
    /// Create a new configuration from pipeline output settings.
    ///
    /// This conversion does NOT validate the device or format.
    pub fn from_output_settings(settings: &OutputSettings) -> Self {
        Self {
            selected_device_id: settings.selected_device_id.clone(),
            exclusive_mode: settings.exclusive_mode,
            bit_perfect_mode: settings.bit_perfect_mode,
        }
    }
}
