#[cfg(not(windows))]
use crate::audio::errors::AudioError;

#[cfg(windows)]
use crate::audio::output_wasapi::RenderDeviceInfo;

#[cfg(not(windows))]
use super::types::RenderDeviceInfo;

#[tauri::command]

pub fn audio_wasapi_list_render_devices() -> Result<Vec<RenderDeviceInfo>, String> {
    #[cfg(windows)]
    {
        crate::audio::output_wasapi::try_list_render_devices().map_err(|e| e.to_public_string())
    }

    #[cfg(not(windows))]
    {
        Err(
            AudioError::UnsupportedPlatform("wasapi render device enumeration requires windows")
                .to_public_string(),
        )
    }
}
