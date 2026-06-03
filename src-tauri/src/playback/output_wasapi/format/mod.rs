// format/mod.rs
//
// Format boundary facade for WASAPI IAudioClient GetMixFormat smoke.
//
// This module provides a cross-platform interface to probe whether
// IAudioClient::GetMixFormat can be called and the returned format
// pointer can be safely released via CoTaskMemFree.
//
// **IMPORTANT**: This module does NOT:
//   - Initialize IAudioClient
//   - IsFormatSupported
//   - GetService
//   - Get IAudioRenderClient
//   - GetBuffer / ReleaseBuffer
//   - Start / Stop / Reset
//   - Produce audible output
//   - Operate outside of explicit opt-in tests

pub mod env;
pub mod format_fields;
pub mod guards;
pub mod report;
pub mod report_builders;

#[cfg(windows)]
pub use format_windows::*;

#[cfg(not(windows))]
pub use format_stub::*;

#[cfg(windows)]
mod format_windows;

#[cfg(not(windows))]
mod format_stub;

// Re-export commonly used types
pub use report::{WasapiMixFormatSmokeReport, WASAPI_MIX_FORMAT_SMOKE_ENV};
