#[cfg(windows)]
pub(crate) mod platform {
    use super::super::errors::PlaybackResult;
    use super::super::windows_audio_com::platform::windows_audio_error;
    use windows::Win32::Media::Audio::{
        eConsole, eRender, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator,
    };
    use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};

    pub(crate) fn create_device_enumerator() -> PlaybackResult<IMMDeviceEnumerator> {
        unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) }
            .map_err(|error| windows_audio_error("create device enumerator", error))
    }

    pub(crate) fn default_render_endpoint(
        enumerator: &IMMDeviceEnumerator,
    ) -> PlaybackResult<IMMDevice> {
        unsafe { enumerator.GetDefaultAudioEndpoint(eRender, eConsole) }
            .map_err(|error| windows_audio_error("read default render endpoint", error))
    }
}
