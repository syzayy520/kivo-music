// guards.rs
//
// RAII guards for COM, format pointer, and buffer lifetime management.
//
// This file contains:
// - ComApartment: ensures CoUninitialize is called on drop
// - MixFormatGuard: ensures CoTaskMemFree is called on drop
// - BufferGuard: ensures ReleaseBuffer is called on drop

use std::ffi::c_void;

use windows::Win32::Media::Audio::{IAudioRenderClient, AUDCLNT_BUFFERFLAGS_SILENT, WAVEFORMATEX};
use windows::Win32::System::Com::{
    CoInitializeEx, CoTaskMemFree, CoUninitialize, COINIT_MULTITHREADED,
};

/// RAII guard for COM apartment initialization.
///
/// Calls `CoInitializeEx` on creation and `CoUninitialize` on drop.
/// This ensures COM is properly cleaned up even if the smoke probe
/// encounters an error.
pub struct ComApartment {
    initialized: bool,
}

impl ComApartment {
    /// Initialize COM apartment with MTA (multi-threaded apartment).
    ///
    /// Returns `Ok(ComApartment)` if initialization succeeded,
    /// or `Err(String)` with the error description.
    pub fn initialize() -> Result<Self, String> {
        // Windows COM FFI boundary
        // no audio client initialization
        // no render client
        // no playback
        unsafe {
            let hr = CoInitializeEx(None, COINIT_MULTITHREADED);
            if hr.is_ok() {
                Ok(Self { initialized: true })
            } else {
                Err(format!("CoInitializeEx failed: {hr:?}"))
            }
        }
    }
}

impl Drop for ComApartment {
    fn drop(&mut self) {
        if self.initialized {
            // Windows COM FFI boundary
            unsafe {
                CoUninitialize();
            }
        }
    }
}

/// RAII guard for WAVEFORMATEX pointer returned by GetMixFormat.
///
/// Calls `CoTaskMemFree` on drop to release the COM-allocated memory.
/// This ensures the format pointer is always properly released even if
/// the smoke probe encounters an error after GetMixFormat succeeds.
pub struct MixFormatGuard {
    pub ptr: *mut WAVEFORMATEX,
}

impl Drop for MixFormatGuard {
    fn drop(&mut self) {
        // mix format pointer lifetime
        // release COM-allocated memory
        if !self.ptr.is_null() {
            unsafe {
                CoTaskMemFree(Some(self.ptr as *const c_void));
            }
        }
    }
}

/// RAII guard for IAudioRenderClient buffer.
///
/// Calls `ReleaseBuffer` on drop to ensure the buffer is always released.
/// This ensures the buffer is properly released even if the smoke probe
/// encounters an error after GetBuffer succeeds.
pub struct BufferGuard {
    render_client: IAudioRenderClient,
    frames: u32,
    released: bool,
}

impl BufferGuard {
    /// Create a new BufferGuard after a successful GetBuffer call.
    pub fn new(render_client: IAudioRenderClient, frames: u32) -> Self {
        Self {
            render_client,
            frames,
            released: false,
        }
    }

    /// Explicitly release the buffer with AUDCLNT_BUFFERFLAGS_SILENT.
    ///
    /// Returns `Ok(())` if ReleaseBuffer succeeded, or `Err(String)` with the error description.
    /// After successful release, the guard will not release again on drop.
    pub fn release_silent(&mut self) -> Result<(), String> {
        if self.released {
            return Ok(());
        }
        // Windows COM FFI boundary
        // release buffer with silent flag
        // no audio produced
        let hr = unsafe {
            self.render_client
                .ReleaseBuffer(self.frames, AUDCLNT_BUFFERFLAGS_SILENT.0 as u32)
        };
        if hr.is_ok() {
            self.released = true;
            Ok(())
        } else {
            Err(format!("IAudioRenderClient::ReleaseBuffer failed: {hr:?}"))
        }
    }
}

impl Drop for BufferGuard {
    fn drop(&mut self) {
        // Safety net: release buffer if not already released
        // Do not panic in drop
        if !self.released {
            let _ = unsafe {
                self.render_client
                    .ReleaseBuffer(self.frames, AUDCLNT_BUFFERFLAGS_SILENT.0 as u32)
            };
        }
    }
}
