// buffer/mod.rs
//
// Buffer boundary facade for WASAPI IAudioRenderClient::GetBuffer + ReleaseBuffer silent smoke.
//
// This module provides a cross-platform interface to probe whether
// IAudioRenderClient::GetBuffer and ReleaseBuffer can be called
// after a successful Initialize in shared mode and GetService.
//
// **IMPORTANT**: This module does NOT:
//   - Call IsFormatSupported
//   - Call GetCurrentPadding
//   - Call Start / Stop / Reset
//   - Produce audible output
//   - Operate outside of explicit opt-in tests

pub mod buffer_steps;
pub mod env;
pub mod format_fields;
pub mod guards;
pub mod report;
pub mod report_builders;
pub mod report_defaults;
pub mod report_failure_builders;
pub mod report_skipped_builders;
pub mod report_success_builders;

#[cfg(windows)]
pub use buffer_windows::*;

#[cfg(not(windows))]
pub use buffer_stub::*;

#[cfg(windows)]
mod buffer_windows;

#[cfg(not(windows))]
mod buffer_stub;

// Re-export commonly used types
pub use report::{WasapiBufferSmokeReport, WASAPI_BUFFER_SMOKE_ENV};
