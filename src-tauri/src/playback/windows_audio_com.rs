#[cfg(windows)]
pub(crate) mod platform {
    use super::super::errors::{PlaybackError, PlaybackResult};
    use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

    pub(crate) struct WindowsComScope;

    impl WindowsComScope {
        pub(crate) fn initialize() -> PlaybackResult<Self> {
            unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }
                .ok()
                .map_err(|error| windows_audio_error("initialize COM", error))?;
            Ok(Self)
        }
    }

    impl Drop for WindowsComScope {
        fn drop(&mut self) {
            unsafe { CoUninitialize() };
        }
    }

    pub(crate) fn windows_audio_error(
        operation: &str,
        error: windows::core::Error,
    ) -> PlaybackError {
        PlaybackError::Output(format!("windows audio {operation} failed: {error}"))
    }
}
