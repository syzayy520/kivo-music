use serde::{Deserialize, Serialize};

use super::windows_audio_mix_format::WindowsAudioMixFormat;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowsAudioClientInitSnapshot {
    pub initialized: bool,
    pub buffer_frame_count: Option<u32>,
    pub mix_format: Option<WindowsAudioMixFormat>,
    pub note: Option<String>,
}

pub fn initialize_default_windows_audio_client(
) -> super::errors::PlaybackResult<WindowsAudioClientInitSnapshot> {
    platform::initialize_default_windows_audio_client()
}

#[cfg(not(windows))]
mod platform {
    use super::WindowsAudioClientInitSnapshot;

    pub fn initialize_default_windows_audio_client(
    ) -> super::super::errors::PlaybackResult<WindowsAudioClientInitSnapshot> {
        Ok(WindowsAudioClientInitSnapshot {
            initialized: false,
            buffer_frame_count: None,
            mix_format: None,
            note: Some("windows audio client initialization is only available on Windows".to_string()),
        })
    }
}

#[cfg(windows)]
mod platform {
    use std::ffi::c_void;

    use super::super::errors::{PlaybackError, PlaybackResult};
    use super::super::windows_audio_mix_format::WindowsAudioMixFormat;
    use super::WindowsAudioClientInitSnapshot;
    use windows::Win32::Media::Audio::{
        eConsole, eRender, AUDCLNT_SHAREMODE_SHARED, IAudioClient, IMMDeviceEnumerator,
        MMDeviceEnumerator, WAVEFORMATEX,
    };
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, CLSCTX_ALL,
        COINIT_MULTITHREADED,
    };

    const DEFAULT_SHARED_BUFFER_DURATION_100NS: i64 = 10_000_000;

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

    pub fn initialize_default_windows_audio_client(
    ) -> PlaybackResult<WindowsAudioClientInitSnapshot> {
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

        unsafe {
            audio_client.Initialize(
                AUDCLNT_SHAREMODE_SHARED,
                0,
                DEFAULT_SHARED_BUFFER_DURATION_100NS,
                0,
                raw_format,
                None,
            )
        }
        .map_err(|error| windows_audio_error("initialize shared audio client", error))?;

        let buffer_frame_count = unsafe { audio_client.GetBufferSize() }
            .map_err(|error| windows_audio_error("read buffer size", error))?;
        unsafe { CoTaskMemFree(Some(raw_format.cast::<c_void>())) };

        Ok(WindowsAudioClientInitSnapshot {
            initialized: true,
            buffer_frame_count: Some(buffer_frame_count),
            mix_format: Some(format_from_wave_format(&mix_format)),
            note: None,
        })
    }

    fn format_from_wave_format(format: &WAVEFORMATEX) -> WindowsAudioMixFormat {
        WindowsAudioMixFormat {
            format_tag: format.wFormatTag,
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
