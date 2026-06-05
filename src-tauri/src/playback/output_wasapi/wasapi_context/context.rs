//! Cross-platform WASAPI context wrapper.
//!
//! On Windows: holds real WASAPI resources.
//! On non-Windows: zero-sized type returning UnsupportedPlatform.

use crate::playback::output_wasapi::errors::WasapiOpenError;

use super::format_cache::WasapiFormatCache;
use super::render_error::{WasapiRenderWriteError, WasapiRenderWriteReport};

/// WASAPI device context wrapper.
///
/// Manages the lifecycle of Windows WASAPI resources.
/// On non-Windows platforms, all operations return UnsupportedPlatform.
pub(crate) struct WasapiContext {
    #[cfg(target_os = "windows")]
    inner: Option<super::windows::WasapiDeviceContext>,
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
    ///
    /// If the context was previously open, it is closed first.
    pub(crate) fn open(&mut self) -> Result<(), WasapiOpenError> {
        // Close any existing context before opening new one
        self.close();

        #[cfg(target_os = "windows")]
        {
            let ctx = super::windows::WasapiDeviceContext::open()?;
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

    /// Start the opened audio client and return a guard that owns Stop.
    pub(crate) fn start_audio_client(
        &self,
    ) -> Result<crate::playback::output_wasapi::start_stop::guards::StartedClientGuard, String>
    {
        #[cfg(target_os = "windows")]
        {
            let inner = self.inner.as_ref().ok_or_else(|| {
                "WasapiContext::start_audio_client requires an open context".to_string()
            })?;
            let audio_client = inner
                .audio_client
                .as_ref()
                .ok_or_else(|| "WasapiContext audio client missing".to_string())?
                .clone();

            crate::playback::output_wasapi::start_stop::start_stop_steps::start_audio_client(
                &audio_client,
            )?;

            Ok(
                crate::playback::output_wasapi::start_stop::guards::StartedClientGuard::new(
                    audio_client,
                ),
            )
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err("UnsupportedPlatform".to_string())
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

    /// Get cached format fields (only available on Windows when open).
    #[allow(dead_code)]
    pub(crate) fn format_cache(&self) -> Option<&WasapiFormatCache> {
        #[cfg(target_os = "windows")]
        {
            self.inner.as_ref().and_then(|ctx| ctx.format_cache())
        }

        #[cfg(not(target_os = "windows"))]
        {
            None
        }
    }

    /// Write silence to the render buffer for the given number of frames.
    ///
    /// Does NOT call IAudioClient::Start.
    /// On non-Windows, returns NotOpen.
    #[allow(dead_code)]
    pub(crate) fn write_render_buffer_silence(
        &self,
        frames: u32,
    ) -> Result<WasapiRenderWriteReport, WasapiRenderWriteError> {
        #[cfg(target_os = "windows")]
        {
            self.inner
                .as_ref()
                .ok_or(WasapiRenderWriteError::NotOpen)?
                .write_render_buffer_silence(frames)
        }

        #[cfg(not(target_os = "windows"))]
        {
            let _ = frames;
            Err(WasapiRenderWriteError::NotOpen)
        }
    }

    /// Write byte data to the render buffer.
    ///
    /// Does NOT call IAudioClient::Start.
    /// On non-Windows, returns NotOpen.
    #[allow(dead_code)]
    pub(crate) fn write_render_buffer_bytes(
        &self,
        frames: u32,
        data: &[u8],
    ) -> Result<WasapiRenderWriteReport, WasapiRenderWriteError> {
        #[cfg(target_os = "windows")]
        {
            self.inner
                .as_ref()
                .ok_or(WasapiRenderWriteError::NotOpen)?
                .write_render_buffer_bytes(frames, data)
        }

        #[cfg(not(target_os = "windows"))]
        {
            let _ = (frames, data);
            Err(WasapiRenderWriteError::NotOpen)
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
