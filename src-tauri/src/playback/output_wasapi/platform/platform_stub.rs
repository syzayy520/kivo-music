// platform_stub.rs
//
// Non-Windows compile boundary stub.
//
// On platforms other than Windows, this module provides a minimal
// `WasapiCompileBoundary` that confirms the crate compiles without
// referencing any real Windows audio types.

/// Compile boundary descriptor for non-Windows platforms.
///
/// On non-Windows targets, WASAPI types are not available.
/// This stub exists solely to satisfy the compile boundary contract
/// and allow the output_wasapi module to compile cross-platform.
#[derive(Clone, Debug)]
pub struct WasapiCompileBoundary {
    /// Always `false` on non-Windows platforms.
    is_windows_target: bool,
}

impl Default for WasapiCompileBoundary {
    fn default() -> Self {
        Self {
            is_windows_target: false,
        }
    }
}

impl WasapiCompileBoundary {
    /// Create a new compile boundary for non-Windows.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `false` — no real Windows WASAPI types are linked.
    pub fn is_windows_target(&self) -> bool {
        self.is_windows_target
    }

    /// Returns a human-readable description of the boundary state.
    pub fn describe(&self) -> String {
        "WASAPI compile boundary: stub (non-Windows platform)".to_string()
    }
}

/// Probe function for non-Windows platforms.
///
/// Returns a compile boundary stub that indicates WASAPI is not available.
pub fn wasapi_compile_boundary() -> WasapiCompileBoundary {
    WasapiCompileBoundary::new()
}
