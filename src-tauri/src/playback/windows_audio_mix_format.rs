use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowsAudioMixFormat {
    pub format_tag: u16,
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub block_align: u16,
    pub avg_bytes_per_sec: u32,
}

pub fn query_default_windows_mix_format(
) -> super::errors::PlaybackResult<Option<WindowsAudioMixFormat>> {
    platform::query_default_windows_mix_format()
}

#[cfg(not(windows))]
mod platform {
    use super::WindowsAudioMixFormat;

    pub fn query_default_windows_mix_format(
    ) -> super::super::errors::PlaybackResult<Option<WindowsAudioMixFormat>> {
        Ok(None)
    }
}

#[cfg(windows)]
mod platform {
    use std::ffi::c_void;

    use super::super::errors::{PlaybackError, PlaybackResult};
    use super::WindowsAudioMixFormat;
    use windows::Win32::Media::Audio::{
        eConsole, eRender, IAudioClient, IMMDeviceEnumerator, MMDeviceEnumerator, WAVEFORMATEX,
    };
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, CLSCTX_ALL,
        COINIT_MULTITHREADED,
    };

    struct ComScope;

    impl ComScope {
        fn initialize() -> PlaybackResult<Self> {
            unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }
                .ok()
                .map_err(|error| windows_audio_error("initialize COM", error))?;
            Ok(Self)
        }
    }

    impl Drop for ComScope {
        fn drop(&mut self) {
            unsafe { CoUninitialize() };
        }
    }

    pub fn query_default_windows_mix_format() -> PlaybackResult<Option<WindowsAudioMixFormat>> {
        let _com = ComScope::initialize()?;
        let enumerator: IMMDeviceEnumerator =
            unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) }
                .map_err(|error| windows_audio_error("create device enumerator", error))?;
        let device = unsafe { enumerator.GetDefaultAudioEndpoint(eRender, eConsole) }
            .map_err(|error| windows_audio_error("read default render endpoint", error))?;
        let audio_client: IAudioClient = unsafe { device.Activate(CLSCTX_ALL, None) }
            .map_err(|error| windows_audio_error("activate audio client", error))?;
        let raw_format = unsafe { audio_client.GetMixFormat() }
            .map_err(|error| windows_audio_error("read mix format", error))?;
        let mix_format = unsafe { *raw_format };
        unsafe { CoTaskMemFree(Some(raw_format.cast::<c_void>())) };

        Ok(Some(format_from_wave_format(&mix_format)))
    }

    fn format_from_wave_format(format: &WAVEFORMATEX) -> WindowsAudioMixFormat {
        WindowsAudioMixFormat {
            format_tag: format.wFormatTag.0,
            channels: format.nChannels,
            sample_rate: format.nSamplesPerSec,
            bits_per_sample: format.wBitsPerSample,
            block_align: format.nBlockAlign,
            avg_bytes_per_sec: format.nAvgBytesPerSec,
        }
    }

    fn windows_audio_error(operation: &str, error: windows::core::Error) -> PlaybackError {
        PlaybackError::Output(format!("windows audio {operation} failed: {error}"))
    }
}
