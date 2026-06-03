// platform.rs
//
// Platform facade for WASAPI compile boundary.
//
// This module selects the appropriate compile boundary implementation
// based on the target platform:
//   - Windows: `platform_windows` with real WASAPI type references
//   - Non-Windows: `platform_stub` with a no-op stub

#[cfg(windows)]
pub use platform_windows::*;

#[cfg(not(windows))]
pub use platform_stub::*;

#[cfg(windows)]
mod platform_windows;

#[cfg(not(windows))]
mod platform_stub;
