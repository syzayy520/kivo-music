// reset_boundary/mod.rs
//
// Reset boundary facade for WASAPI reset smoke.
//
// This module provides a cross-platform interface to probe whether
// IAudioClient::Reset can succeed after a successful
// Initialize in shared mode, GetService, GetBuffer(1),
// ReleaseBuffer(1, AUDCLNT_BUFFERFLAGS_SILENT), Start,
// GetCurrentPadding, and Stop.
//
// **IMPORTANT**: This module does NOT:
//   - Call IsFormatSupported
//   - Produce audible output
//   - Operate outside of explicit opt-in tests
//   - Perform silent loop
//   - Write non-silent audio data
//   - Create threads or async runtime
//   - Register callbacks
//   - Connect OutputSink
//   - Expose PlaybackCapabilities

pub mod env;
pub mod format_fields;
pub mod guards;
pub mod report;
pub mod report_builders;
pub mod report_defaults;
pub mod report_failure_buffer_builders;
pub mod report_failure_prereq_builders;
pub mod report_failure_reset_builders;
pub mod report_skipped_builders;
pub mod report_success_builders;
pub mod reset_boundary_steps;

#[cfg(windows)]
pub use reset_boundary_windows::*;

#[cfg(not(windows))]
pub use reset_boundary_stub::*;

#[cfg(windows)]
mod reset_boundary_windows;

#[cfg(not(windows))]
mod reset_boundary_stub;

// Re-export commonly used types
pub use report::{WasapiResetBoundarySmokeReport, WASAPI_RESET_SMOKE_ENV};
