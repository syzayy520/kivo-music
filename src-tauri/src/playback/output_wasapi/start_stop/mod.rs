// start_stop/mod.rs
//
// Start/Stop boundary facade for WASAPI IAudioClient::Start + Stop silent smoke.
//
// This module provides a cross-platform interface to probe whether
// IAudioClient::Start and Stop can be called after a successful
// Initialize in shared mode, GetService, GetBuffer(1), and
// ReleaseBuffer(1, AUDCLNT_BUFFERFLAGS_SILENT).
//
// **IMPORTANT**: This module does NOT:
//   - Call IsFormatSupported
//   - Call GetCurrentPadding
//   - Call Reset
//   - Produce audible output
//   - Operate outside of explicit opt-in tests

pub mod env;
pub mod format_fields;
pub mod guards;
pub mod report;
pub mod report_builders;
pub mod report_defaults;
pub mod report_failure_buffer_builders;
pub mod report_failure_builders;
pub mod report_failure_prereq_builders;
pub mod report_failure_start_stop_builders;
pub mod report_skipped_builders;
pub mod report_success_builders;
pub mod start_stop_steps;

#[cfg(windows)]
pub use start_stop_windows::*;

#[cfg(not(windows))]
pub use start_stop_stub::*;

#[cfg(windows)]
mod start_stop_windows;

#[cfg(not(windows))]
mod start_stop_stub;

// Re-export commonly used types
pub use report::{WasapiStartStopSmokeReport, WASAPI_START_STOP_SMOKE_ENV};
