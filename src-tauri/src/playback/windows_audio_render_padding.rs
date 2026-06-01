#[cfg(windows)]
pub(crate) mod platform {
    use super::super::errors::PlaybackResult;
    use super::super::windows_audio_com::platform::windows_audio_error;
    use windows::Win32::Media::Audio::IAudioClient;

    pub(crate) fn current_padding_frames(audio_client: &IAudioClient) -> PlaybackResult<u32> {
        unsafe { audio_client.GetCurrentPadding() }
            .map_err(|error| windows_audio_error("read current padding", error))
    }
}
