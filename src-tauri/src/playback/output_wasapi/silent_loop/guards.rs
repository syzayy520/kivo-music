// silent_loop/guards.rs
//
// RAII guards for COM, format pointer, buffer, and started client lifetime management.
//
// This file contains:
// - ComApartment: ensures CoUninitialize is called on drop
// - MixFormatGuard: ensures CoTaskMemFree is called on drop
// - BufferGuard: ensures ReleaseBuffer is called on drop
// - StartedClientGuard: ensures Stop is called on drop after Start
//
// **PROHIBITED** (not implemented here):
//   - IAudioClient::Reset
//   - Audio thread management
//   - Render loop
//   - Non-silent audio data

use std::ffi::c_void;

use windows::Win32::Media::Audio::{
    IAudioClient, IAudioRenderClient, AUDCLNT_BUFFERFLAGS_SILENT, WAVEFORMATEX,
};
use windows::Win32::System::Com::{
    CoInitializeEx, CoTaskMemFree, CoUninitialize, COINIT_MULTITHREADED,
};

/// RAII guard for COM apartment initialization.
pub struct ComApartment {
    initialized: bool,
}

impl ComApartment {
    pub fn initialize() -> Result<Self, String> {
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
            unsafe {
                CoUninitialize();
            }
        }
    }
}

/// RAII guard for WAVEFORMATEX pointer returned by GetMixFormat.
pub struct MixFormatGuard {
    pub ptr: *mut WAVEFORMATEX,
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

/// RAII guard for IAudioRenderClient buffer.
pub struct BufferGuard {
    render_client: IAudioRenderClient,
    frames: u32,
    released: bool,
}

impl BufferGuard {
    pub fn new(render_client: IAudioRenderClient, frames: u32) -> Self {
        Self {
            render_client,
            frames,
            released: false,
        }
    }

    pub fn release_silent(&mut self) -> Result<(), String> {
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
            Err(format!("IAudioRenderClient::ReleaseBuffer failed: {hr:?}"))
        }
    }
}

impl Drop for BufferGuard {
    fn drop(&mut self) {
        if !self.released {
            let _ = unsafe {
                self.render_client
                    .ReleaseBuffer(self.frames, AUDCLNT_BUFFERFLAGS_SILENT.0 as u32)
            };
        }
    }
}

/// RAII guard ensuring IAudioClient::Stop is called after Start.
pub struct StartedClientGuard {
    audio_client: IAudioClient,
    stopped: bool,
}

impl StartedClientGuard {
    pub fn new(audio_client: IAudioClient) -> Self {
        Self {
            audio_client,
            stopped: false,
        }
    }

    pub fn stop(&mut self) -> Result<(), String> {
        if self.stopped {
            return Ok(());
        }
        let hr = unsafe { self.audio_client.Stop() };
        if hr.is_ok() {
            self.stopped = true;
            Ok(())
        } else {
            Err(format!("IAudioClient::Stop failed: {hr:?}"))
        }
    }
}

impl Drop for StartedClientGuard {
    fn drop(&mut self) {
        if !self.stopped {
            let _ = unsafe { self.audio_client.Stop() };
        }
    }
}
