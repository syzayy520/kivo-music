#[cfg(windows)]
pub(crate) mod platform {
    use super::super::errors::PlaybackResult;
    use super::super::windows_audio_com::platform::windows_audio_error;
    use windows::Win32::Media::Audio::{IAudioClient, IMMDevice};
    use windows::Win32::System::Com::CLSCTX_ALL;

    pub(crate) fn activate_audio_client(device: &IMMDevice) -> PlaybackResult<IAudioClient> {
        unsafe { device.Activate(CLSCTX_ALL, None) }
            .map_err(|error| windows_audio_error("activate audio client", error))
    }
}
