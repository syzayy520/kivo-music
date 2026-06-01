use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowsAudioRenderClientSnapshot {
    pub render_client_available: bool,
    pub buffer_frame_count: Option<u32>,
    pub note: Option<String>,
}

pub fn query_default_windows_render_client_boundary(
) -> super::errors::PlaybackResult<WindowsAudioRenderClientSnapshot> {
    platform::query_default_windows_render_client_boundary()
}

#[cfg(not(windows))]
mod platform {
    use super::WindowsAudioRenderClientSnapshot;

    pub fn query_default_windows_render_client_boundary(
    ) -> super::super::errors::PlaybackResult<WindowsAudioRenderClientSnapshot> {
        Ok(WindowsAudioRenderClientSnapshot {
            render_client_available: false,
            buffer_frame_count: None,
            note: Some("windows audio render client is only available on Windows".to_string()),
        })
    }
}

#[cfg(windows)]
mod platform {
    use std::ffi::c_void;

    use super::super::errors::PlaybackResult;
    use super::super::windows_audio_client::platform::activate_audio_client;
    use super::super::windows_audio_com::platform::{windows_audio_error, WindowsComScope};
    use super::super::windows_audio_endpoint::platform::{
        create_device_enumerator, default_render_endpoint,
    };
    use super::WindowsAudioRenderClientSnapshot;
    use windows::Win32::Media::Audio::{IAudioRenderClient, AUDCLNT_SHAREMODE_SHARED};
    use windows::Win32::System::Com::CoTaskMemFree;

    const DEFAULT_SHARED_BUFFER_DURATION_100NS: i64 = 10_000_000;

    pub fn query_default_windows_render_client_boundary(
    ) -> PlaybackResult<WindowsAudioRenderClientSnapshot> {
        let _com = WindowsComScope::initialize()?;
        let enumerator = create_device_enumerator()?;
        let device = default_render_endpoint(&enumerator)?;
        let audio_client = activate_audio_client(&device)?;
        let raw_format = unsafe { audio_client.GetMixFormat() }
            .map_err(|error| windows_audio_error("read mix format", error))?;

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
        let _render_client: IAudioRenderClient = unsafe { audio_client.GetService() }
            .map_err(|error| windows_audio_error("get render client service", error))?;
        unsafe { CoTaskMemFree(Some(raw_format.cast::<c_void>())) };

        Ok(WindowsAudioRenderClientSnapshot {
            render_client_available: true,
            buffer_frame_count: Some(buffer_frame_count),
            note: None,
        })
    }
}
