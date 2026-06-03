// guards.rs
//
// RAII guards for COM and format pointer lifetime management.
//
// This file contains:
// - ComApartment: ensures CoUninitialize is called on drop
// - MixFormatGuard: ensures CoTaskMemFree is called on drop

use std::ffi::c_void;

use windows::Win32::Media::Audio::WAVEFORMATEX;
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
