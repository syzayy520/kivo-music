// platform_windows.rs
//
// Windows-only compile boundary for WASAPI types.
//
// This module references real Windows WASAPI types from the `windows` crate
// to establish a compile boundary. It confirms that:
//   - The `windows` crate compiles with the required features
//   - WASAPI types are accessible at compile time
//
// **IMPORTANT**: This module does NOT:
//   - Call any real Windows audio APIs
//   - Open or interact with audio devices
//   - Create COM objects or threads
//   - Produce audible output
//
// The types are held as fields solely to prove they compile.

use windows::Win32::Media::Audio::{AUDCLNT_SHAREMODE, WAVEFORMATEX};

/// Compile boundary descriptor for Windows platforms.
///
/// Holds references to Windows WASAPI types to confirm they compile
/// and are accessible from the project. Does not perform any real
/// audio operations.
#[derive(Clone)]
pub struct WasapiCompileBoundary {
    /// Whether this is a Windows target (always `true` in this module).
    is_windows_target: bool,
    /// WASAPI waveform format type — held to confirm compile boundary.
    format: WAVEFORMATEX,
    /// WASAPI share mode type — held to confirm compile boundary.
    share_mode: AUDCLNT_SHAREMODE,
}

impl std::fmt::Debug for WasapiCompileBoundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Copy packed struct fields to local variables before referencing
        // (packed struct fields cannot be directly referenced due to alignment).
        let format_tag = self.format.wFormatTag;
        let channels = self.format.nChannels;
        let sample_rate = self.format.nSamplesPerSec;
        let share_mode_val = self.share_mode.0;
        f.debug_struct("WasapiCompileBoundary")
            .field("is_windows_target", &self.is_windows_target)
            .field("format_tag", &format_tag)
            .field("channels", &channels)
            .field("sample_rate", &sample_rate)
            .field("share_mode", &share_mode_val)
            .finish()
    }
}

impl Default for WasapiCompileBoundary {
    fn default() -> Self {
        Self {
            is_windows_target: true,
            format: WAVEFORMATEX::default(),
            share_mode: AUDCLNT_SHAREMODE::default(),
        }
    }
}

impl WasapiCompileBoundary {
    /// Create a new compile boundary for Windows.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` — real Windows WASAPI types are linked.
    pub fn is_windows_target(&self) -> bool {
        self.is_windows_target
    }

    /// Returns a human-readable description of the boundary state.
    pub fn describe(&self) -> String {
        "WASAPI compile boundary: Windows target (types linked, no operations)".to_string()
    }

    /// Get the held WAVEFORMATEX (for compile boundary verification).
    ///
    /// This is a default-constructed instance; it does not represent
    /// any real audio format.
    pub fn wave_format(&self) -> &WAVEFORMATEX {
        &self.format
    }

    /// Get the held AUDCLNT_SHAREMODE (for compile boundary verification).
    ///
    /// This is a default-constructed instance; it does not represent
    /// any real share mode selection.
    pub fn share_mode(&self) -> &AUDCLNT_SHAREMODE {
        &self.share_mode
    }
}

/// Probe function for Windows platforms.
///
/// Returns a compile boundary that confirms WASAPI types are linked.
pub fn wasapi_compile_boundary() -> WasapiCompileBoundary {
    WasapiCompileBoundary::new()
}
