// silent_loop/mod.rs
//
// Silent loop boundary facade for WASAPI silent write loop smoke.
//
// This module provides a cross-platform interface to probe whether
// a fixed-iteration silent write loop can execute after a successful
// Initialize in shared mode, GetService, GetBuffer(1),
// ReleaseBuffer(1, AUDCLNT_BUFFERFLAGS_SILENT), and Start.
//
// **IMPORTANT**: This module does NOT:
//   - Call IsFormatSupported
//   - Call Reset
//   - Produce audible output
//   - Operate outside of explicit opt-in tests
//   - Perform render loop
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
pub mod report_failure_builders;
pub mod report_failure_loop_builders;
pub mod report_failure_prereq_builders;
pub mod report_skipped_builders;
pub mod report_success_builders;
pub mod silent_loop_steps;

#[cfg(windows)]
pub use silent_loop_windows::*;

#[cfg(not(windows))]
pub use silent_loop_stub::*;

#[cfg(windows)]
mod silent_loop_windows;

#[cfg(not(windows))]
mod silent_loop_stub;

// Re-export commonly used types
pub use report::{WasapiSilentLoopSmokeReport, WASAPI_SILENT_LOOP_SMOKE_ENV};
