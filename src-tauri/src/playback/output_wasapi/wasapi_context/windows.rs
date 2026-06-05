//! Windows-only WASAPI device context implementation.
//!
//! Contains the real WASAPI initialization chain:
//! COM → device enumerator → endpoint → IAudioClient → IAudioRenderClient.

use std::ffi::c_void;

use windows::Win32::Media::Audio::{
    eConsole, eRender, IAudioClient, IAudioRenderClient, IMMDeviceEnumerator, MMDeviceEnumerator,
    AUDCLNT_SHAREMODE_SHARED, WAVEFORMATEX,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, CLSCTX_ALL,
    COINIT_MULTITHREADED,
};

use crate::playback::output_wasapi::errors::WasapiOpenError;

use super::format_cache::WasapiFormatCache;

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
/// Resources are acquired during `open()` and released during `close()` or `drop`.
/// Drop is safe and idempotent.
pub(super) struct WasapiDeviceContext {
    _com: ComGuard,
    _enumerator: IMMDeviceEnumerator,
    _endpoint: windows::Win32::Media::Audio::IMMDevice,
    pub(super) audio_client: Option<IAudioClient>,
    pub(super) render_client: Option<IAudioRenderClient>,
    _mix_format: MixFormatGuard,
    #[allow(dead_code)]
    pub(super) format_cache: Option<WasapiFormatCache>,
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
        let enumerator: IMMDeviceEnumerator = unsafe {
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
        }
        .map_err(|e| WasapiOpenError::DeviceNotFound(format!("CoCreateInstance failed: {e}")))?;

        // Step 3: Get default audio render endpoint
        let endpoint =
            unsafe { enumerator.GetDefaultAudioEndpoint(eRender, eConsole) }.map_err(|e| {
                WasapiOpenError::DeviceNotFound(format!("GetDefaultAudioEndpoint failed: {e}"))
            })?;

        // Step 4: Activate IAudioClient
        let audio_client: IAudioClient =
            unsafe { endpoint.Activate(CLSCTX_ALL, None) }.map_err(|e| {
                WasapiOpenError::ClientActivateFailed(format!("Activate IAudioClient failed: {e}"))
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

        // Step 6.5: Extract and cache format fields from mix format pointer
        let format_cache = unsafe { WasapiFormatCache::from_ptr(mix_format.ptr) };

        // Step 7: Get IAudioRenderClient
        let render_client: IAudioRenderClient =
            unsafe { audio_client.GetService() }.map_err(|e| {
                WasapiOpenError::RenderClientFailed(format!("IAudioClient::GetService failed: {e}"))
            })?;

        Ok(Self {
            _com: com,
            _enumerator: enumerator,
            _endpoint: endpoint,
            audio_client: Some(audio_client),
            render_client: Some(render_client),
            _mix_format: mix_format,
            format_cache: Some(format_cache),
        })
    }

    /// Check if the context has a render client.
    pub(super) fn has_render_client(&self) -> bool {
        self.render_client.is_some()
    }

    /// Get the cached format fields, if available.
    #[allow(dead_code)]
    pub(super) fn format_cache(&self) -> Option<&WasapiFormatCache> {
        self.format_cache.as_ref()
    }
}

impl Drop for WasapiDeviceContext {
    fn drop(&mut self) {
        // Drop render client first, then audio client
        self.render_client.take();
        self.audio_client.take();
        // COM guard and mix format guard will clean up on drop
    }
}
