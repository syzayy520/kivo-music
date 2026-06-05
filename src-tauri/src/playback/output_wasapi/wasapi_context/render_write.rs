//! Context-internal render buffer write primitives.
//!
//! Provides `write_render_buffer_silence()` and `write_render_buffer_bytes()`
//! methods on `WasapiDeviceContext`, encapsulating GetBuffer/ReleaseBuffer pairing.
//!
//! **Does NOT:**
//! - Call IAudioClient::Start
//! - Produce audible output
//! - Use RingBuffer
//! - Modify sink.rs or NativePipeline

#![allow(dead_code)]

use windows::Win32::Media::Audio::{IAudioRenderClient, AUDCLNT_BUFFERFLAGS_SILENT};

use super::render_error::{WasapiRenderWriteError, WasapiRenderWriteReport};

/// RAII guard for IAudioRenderClient::GetBuffer.
///
/// Ensures ReleaseBuffer is called on drop if not already released.
/// On drop, releases with AUDCLNT_BUFFERFLAGS_SILENT as safety net.
struct RenderBufferGuard {
    render_client: IAudioRenderClient,
    frames: u32,
    released: bool,
}

impl RenderBufferGuard {
    fn new(render_client: IAudioRenderClient, frames: u32) -> Self {
        Self {
            render_client,
            frames,
            released: false,
        }
    }

    /// Explicitly release the buffer with data (no silent flag).
    fn release_data(&mut self) -> Result<(), WasapiRenderWriteError> {
        if self.released {
            return Ok(());
        }
        let hr = unsafe { self.render_client.ReleaseBuffer(self.frames, 0) };
        if hr.is_ok() {
            self.released = true;
            Ok(())
        } else {
            Err(WasapiRenderWriteError::ReleaseBufferFailed(format!(
                "{hr:?}"
            )))
        }
    }

    /// Explicitly release the buffer with AUDCLNT_BUFFERFLAGS_SILENT.
    fn release_silent(&mut self) -> Result<(), WasapiRenderWriteError> {
        if self.released {
            return Ok(());
        }
        let hr = unsafe {
            self.render_client
                .ReleaseBuffer(self.frames, AUDCLNT_BUFFERFLAGS_SILENT.0 as u32)
        };
        if hr.is_ok() {
            self.released = true;
            Ok(())
        } else {
            Err(WasapiRenderWriteError::ReleaseBufferFailed(format!(
                "{hr:?}"
            )))
        }
    }
}

impl Drop for RenderBufferGuard {
    fn drop(&mut self) {
        // Safety net: release buffer with silent flag if not already released.
        // Do not panic in drop.
        if !self.released {
            let _ = unsafe {
                self.render_client
                    .ReleaseBuffer(self.frames, AUDCLNT_BUFFERFLAGS_SILENT.0 as u32)
            };
        }
    }
}

impl super::windows::WasapiDeviceContext {
    /// Write silence to the render buffer for the given number of frames.
    ///
    /// Uses GetBuffer + ReleaseBuffer(AUDCLNT_BUFFERFLAGS_SILENT).
    /// Does NOT call IAudioClient::Start.
    pub(super) fn write_render_buffer_silence(
        &self,
        frames: u32,
    ) -> Result<WasapiRenderWriteReport, WasapiRenderWriteError> {
        if frames == 0 {
            return Err(WasapiRenderWriteError::InvalidFrameCount);
        }
        let render_client = self
            .render_client
            .as_ref()
            .ok_or(WasapiRenderWriteError::MissingRenderClient)?;
        let _audio_client = self
            .audio_client
            .as_ref()
            .ok_or(WasapiRenderWriteError::MissingAudioClient)?;

        let mut guard = RenderBufferGuard::new(render_client.clone(), frames);
        // GetBuffer: we don't use the returned pointer for silence
        let _ptr = unsafe { render_client.GetBuffer(frames) }
            .map_err(|e| WasapiRenderWriteError::GetBufferFailed(format!("{e:?}")))?;

        guard.release_silent()?;

        let format = self.format_cache.as_ref();
        Ok(WasapiRenderWriteReport {
            frames_written: frames,
            bytes_written: 0,
            used_silent_flag: true,
            sample_rate_hz: format.map_or(0, |f| f.sample_rate_hz),
            channels: format.map_or(0, |f| f.channels),
        })
    }

    /// Write byte data to the render buffer.
    ///
    /// Uses GetBuffer + memcpy + ReleaseBuffer (no silent flag).
    /// Does NOT call IAudioClient::Start.
    ///
    /// The byte slice length must equal `frames * block_align` from the cached format.
    pub(super) fn write_render_buffer_bytes(
        &self,
        frames: u32,
        data: &[u8],
    ) -> Result<WasapiRenderWriteReport, WasapiRenderWriteError> {
        if frames == 0 {
            return Err(WasapiRenderWriteError::InvalidFrameCount);
        }
        let format = self
            .format_cache
            .as_ref()
            .ok_or(WasapiRenderWriteError::MissingAudioClient)?;
        if !format.is_float32() {
            return Err(WasapiRenderWriteError::UnsupportedFormat);
        }
        let expected_bytes = format.frames_to_bytes(frames) as usize;
        if data.len() != expected_bytes {
            return Err(WasapiRenderWriteError::ByteLengthMismatch {
                expected: expected_bytes,
                actual: data.len(),
            });
        }

        let render_client = self
            .render_client
            .as_ref()
            .ok_or(WasapiRenderWriteError::MissingRenderClient)?;
        let _audio_client = self
            .audio_client
            .as_ref()
            .ok_or(WasapiRenderWriteError::MissingAudioClient)?;

        let mut guard = RenderBufferGuard::new(render_client.clone(), frames);
        let ptr = unsafe { render_client.GetBuffer(frames) }
            .map_err(|e| WasapiRenderWriteError::GetBufferFailed(format!("{e:?}")))?;

        // Copy data into the WASAPI buffer
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, expected_bytes);
        }

        guard.release_data()?;

        Ok(WasapiRenderWriteReport {
            frames_written: frames,
            bytes_written: expected_bytes as u32,
            used_silent_flag: false,
            sample_rate_hz: format.sample_rate_hz,
            channels: format.channels,
        })
    }
}
