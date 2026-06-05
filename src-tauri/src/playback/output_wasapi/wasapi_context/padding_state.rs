//! WASAPI padding state facade.
//!
//! This module exposes read-only padding/capacity state without starting
//! playback, requesting render buffers, or writing PCM.

use super::context::WasapiContext;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WasapiPaddingStateError {
    NotOpen,
    MissingAudioClient,
    MissingBufferCapacity,
    GetCurrentPaddingFailed(String),
    PaddingExceedsCapacity { padding: u32, capacity: u32 },
    UnsupportedPlatform,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WasapiPaddingStateSnapshot {
    pub buffer_frame_capacity: u32,
    pub current_padding_frames: u32,
    pub available_frames: u32,
}

#[allow(dead_code)]
pub(crate) fn calculate_available_frames(
    capacity: u32,
    padding: u32,
) -> Result<u32, WasapiPaddingStateError> {
    if padding > capacity {
        return Err(WasapiPaddingStateError::PaddingExceedsCapacity { padding, capacity });
    }

    Ok(capacity - padding)
}

impl WasapiContext {
    #[allow(dead_code)]
    pub(crate) fn buffer_frame_capacity(&self) -> Result<u32, WasapiPaddingStateError> {
        #[cfg(target_os = "windows")]
        {
            self.inner
                .as_ref()
                .ok_or(WasapiPaddingStateError::NotOpen)?
                .buffer_frame_capacity()
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err(WasapiPaddingStateError::NotOpen)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn current_padding_frames(&self) -> Result<u32, WasapiPaddingStateError> {
        #[cfg(target_os = "windows")]
        {
            self.inner
                .as_ref()
                .ok_or(WasapiPaddingStateError::NotOpen)?
                .current_padding_frames()
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err(WasapiPaddingStateError::NotOpen)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn available_frames(&self) -> Result<u32, WasapiPaddingStateError> {
        let capacity = self.buffer_frame_capacity()?;
        let padding = self.current_padding_frames()?;

        calculate_available_frames(capacity, padding)
    }

    #[allow(dead_code)]
    pub(crate) fn padding_state_snapshot(
        &self,
    ) -> Result<WasapiPaddingStateSnapshot, WasapiPaddingStateError> {
        let capacity = self.buffer_frame_capacity()?;
        let padding = self.current_padding_frames()?;
        let available = calculate_available_frames(capacity, padding)?;

        Ok(WasapiPaddingStateSnapshot {
            buffer_frame_capacity: capacity,
            current_padding_frames: padding,
            available_frames: available,
        })
    }
}
