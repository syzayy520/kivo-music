//! WASAPI context for real device open/close boundary.
//!
//! This module provides a RAII context that manages the lifecycle of
//! Windows WASAPI resources: COM apartment, device enumerator, endpoint,
//! IAudioClient, and IAudioRenderClient.
//!
//! **This module does NOT:**
//! - Start or stop IAudioClient
//! - Write PCM data
//! - Produce audible output
//! - Create output threads
//! - Use RingBuffer

use crate::playback::output_wasapi::errors::WasapiOpenError;

#[cfg(target_os = "windows")]
mod windows_impl {
    use super::WasapiOpenError;
    use std::ffi::c_void;
    use windows::Win32::Media::Audio::{
        eConsole, eRender, IAudioClient, IAudioRenderClient, IMMDeviceEnumerator,
        MMDeviceEnumerator, AUDCLNT_SHAREMODE_SHARED, WAVEFORMATEX,
    };
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, CLSCTX_ALL,
        COINIT_MULTITHREADED,
    };

    /// RAII guard for COM apartment initialization.
    struct ComGuard {
        initialized: bool,
    }

    impl ComGuard {
        fn initialize() -> Result<Self, WasapiOpenError> {
            unsafe {
                let hr = CoInitializeEx(None, COINIT_MULTITHREADED);
                if hr.is_ok() {
                    Ok(Self { initialized: true })
                } else {
                    Err(WasapiOpenError::ComInitFailed(format!(
                        "CoInitializeEx failed: {hr:?}"
                    )))
                }
            }
        }
    }

    impl Drop for ComGuard {
        fn drop(&mut self) {
            if self.initialized {
                unsafe {
                    CoUninitialize();
                }
            }
        }
    }

    /// RAII guard for WAVEFORMATEX pointer returned by GetMixFormat.
    struct MixFormatGuard {
        ptr: *mut WAVEFORMATEX,
    }

    impl Drop for MixFormatGuard {
        fn drop(&mut self) {
            if !self.ptr.is_null() {
                unsafe {
                    CoTaskMemFree(Some(self.ptr as *const c_void));
                }
            }
        }
    }

    /// WASAPI device context holding all resources needed for output.
    ///
    /// Resources are acquired during `open()` and released during `close()`.
    /// Drop is safe and idempotent.
    pub(super) struct WasapiDeviceContext {
        _com: ComGuard,
        _enumerator: IMMDeviceEnumerator,
        _endpoint: windows::Win32::Media::Audio::IMMDevice,
        audio_client: Option<IAudioClient>,
        render_client: Option<IAudioRenderClient>,
        mix_format: Option<MixFormatGuard>,
    }

    impl WasapiDeviceContext {
        /// Open a WASAPI device context.
        ///
        /// Performs the full initialization chain:
        /// 1. Initialize COM (MTA)
        /// 2. Create device enumerator
        /// 3. Get default audio render endpoint
        /// 4. Activate IAudioClient
        /// 5. Get mix format
        /// 6. Initialize IAudioClient in shared mode
        /// 7. Get IAudioRenderClient
        ///
        /// Does NOT:
        /// - Start IAudioClient
        /// - Write PCM data
        /// - Produce audible output
        pub(super) fn open() -> Result<Self, WasapiOpenError> {
            // Step 1: Initialize COM
            let com = ComGuard::initialize()?;

            // Step 2: Create device enumerator
            let enumerator: IMMDeviceEnumerator =
                unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) }.map_err(
                    |e| WasapiOpenError::DeviceNotFound(format!("CoCreateInstance failed: {e}")),
                )?;

            // Step 3: Get default audio render endpoint
            let endpoint = unsafe { enumerator.GetDefaultAudioEndpoint(eRender, eConsole) }
                .map_err(|e| {
                    WasapiOpenError::DeviceNotFound(format!("GetDefaultAudioEndpoint failed: {e}"))
                })?;

            // Step 4: Activate IAudioClient
            let audio_client: IAudioClient = unsafe { endpoint.Activate(CLSCTX_ALL, None) }
                .map_err(|e| {
                    WasapiOpenError::ClientActivateFailed(format!(
                        "Activate IAudioClient failed: {e}"
                    ))
                })?;

            // Step 5: Get mix format
            let format_ptr = unsafe { audio_client.GetMixFormat() }
                .map_err(|e| WasapiOpenError::FormatFailed(format!("GetMixFormat failed: {e}")))?;
            let mix_format = MixFormatGuard { ptr: format_ptr };

            // Step 6: Initialize IAudioClient in shared mode
            let hr = unsafe {
                audio_client.Initialize(
                    AUDCLNT_SHAREMODE_SHARED,
                    0,              // stream_flags
                    0,              // hnsBufferDuration (0 = default)
                    0,              // hnsPeriodicity (0 = default for shared mode)
                    mix_format.ptr, // pFormat
                    None,           // AudioSessionGuid
                )
            };
            if hr.is_err() {
                return Err(WasapiOpenError::InitializeFailed(format!(
                    "IAudioClient::Initialize failed: {hr:?}"
                )));
            }

            // Step 7: Get IAudioRenderClient
            let render_client: IAudioRenderClient =
                unsafe { audio_client.GetService() }.map_err(|e| {
                    WasapiOpenError::RenderClientFailed(format!(
                        "IAudioClient::GetService failed: {e}"
                    ))
                })?;

            Ok(Self {
                _com: com,
                _enumerator: enumerator,
                _endpoint: endpoint,
                audio_client: Some(audio_client),
                render_client: Some(render_client),
                mix_format: Some(mix_format),
            })
        }

        /// Check if the context has a render client.
        pub(super) fn has_render_client(&self) -> bool {
            self.render_client.is_some()
        }
    }

    impl Drop for WasapiDeviceContext {
        fn drop(&mut self) {
            // Drop render client first, then audio client
            self.render_client.take();
            self.audio_client.take();
            self.mix_format.take();
            // COM guard will clean up on drop
        }
    }
}

/// WASAPI device context wrapper.
///
/// On Windows, holds the real WASAPI resources.
/// On non-Windows, is a zero-sized type.
pub(crate) struct WasapiContext {
    #[cfg(target_os = "windows")]
    inner: Option<windows_impl::WasapiDeviceContext>,
}

impl WasapiContext {
    /// Create a new empty context.
    pub(crate) fn new() -> Self {
        Self {
            #[cfg(target_os = "windows")]
            inner: None,
        }
    }

    /// Open the WASAPI device context.
    ///
    /// On Windows: attempts full WASAPI initialization chain.
    /// On non-Windows: returns UnsupportedPlatform error.
    pub(crate) fn open(&mut self) -> Result<(), WasapiOpenError> {
        #[cfg(target_os = "windows")]
        {
            let ctx = windows_impl::WasapiDeviceContext::open()?;
            self.inner = Some(ctx);
            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err(WasapiOpenError::UnsupportedPlatform)
        }
    }

    /// Check if the context is open (has real device resources).
    pub(crate) fn is_open(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            self.inner.is_some()
        }

        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }

    /// Check if the context has a render client acquired.
    pub(crate) fn has_render_client(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            self.inner
                .as_ref()
                .is_some_and(|ctx| ctx.has_render_client())
        }

        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }

    /// Close the context, releasing all resources.
    ///
    /// Idempotent: safe to call multiple times.
    pub(crate) fn close(&mut self) {
        #[cfg(target_os = "windows")]
        {
            self.inner.take();
        }
    }
}

impl Default for WasapiContext {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for WasapiContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasapiContext")
            .field("is_open", &self.is_open())
            .field("has_render_client", &self.has_render_client())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_context_is_not_open() {
        let ctx = WasapiContext::new();
        assert!(!ctx.is_open());
        assert!(!ctx.has_render_client());
    }

    #[test]
    fn close_before_open_is_noop() {
        let mut ctx = WasapiContext::new();
        ctx.close();
        assert!(!ctx.is_open());
    }

    #[test]
    fn default_is_not_open() {
        let ctx = WasapiContext::default();
        assert!(!ctx.is_open());
        assert!(!ctx.has_render_client());
    }

    #[test]
    fn debug_format_shows_state() {
        let ctx = WasapiContext::new();
        let debug = format!("{ctx:?}");
        assert!(debug.contains("is_open: false"));
        assert!(debug.contains("has_render_client: false"));
    }
}
