//! Cached mix format fields from WAVEFORMATEX.
//!
//! Avoids repeated raw pointer access to the mix format pointer.
//! Saved once during `WasapiDeviceContext::open()` and exposed via accessor.

use windows::Win32::Media::Audio::WAVEFORMATEX;

/// WAVE_FORMAT_IEEE_FLOAT constant (format_tag = 3).
#[allow(dead_code)]
const WAVE_FORMAT_IEEE_FLOAT: u16 = 3;

/// Cached mix format fields extracted from WAVEFORMATEX during open().
///
/// Stores typed copies of the raw WAVEFORMATEX fields so callers
/// do not need to access the raw pointer again.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WasapiFormatCache {
    pub sample_rate_hz: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub block_align: u16,
    pub avg_bytes_per_sec: u32,
    pub format_tag: u16,
    pub cb_size: u16,
}

impl WasapiFormatCache {
    /// Extract and cache format fields from a WAVEFORMATEX pointer.
    ///
    /// # Safety
    /// The pointer must be valid, non-null, and properly aligned.
    /// Fields are copied to local variables before being returned
    /// (WAVEFORMATEX is a packed struct).
    pub(crate) unsafe fn from_ptr(ptr: *mut WAVEFORMATEX) -> Self {
        let fmt = unsafe { &*ptr };
        Self {
            sample_rate_hz: fmt.nSamplesPerSec,
            channels: fmt.nChannels,
            bits_per_sample: fmt.wBitsPerSample,
            block_align: fmt.nBlockAlign,
            avg_bytes_per_sec: fmt.nAvgBytesPerSec,
            format_tag: fmt.wFormatTag,
            cb_size: fmt.cbSize,
        }
    }

    /// Check if the format is IEEE Float 32 (format_tag == 3).
    #[allow(dead_code)]
    pub(crate) fn is_float32(&self) -> bool {
        self.format_tag == WAVE_FORMAT_IEEE_FLOAT
    }

    /// Calculate the byte size for a given number of frames.
    #[allow(dead_code)]
    pub(crate) fn frames_to_bytes(&self, frames: u32) -> u32 {
        frames * self.block_align as u32
    }
}
