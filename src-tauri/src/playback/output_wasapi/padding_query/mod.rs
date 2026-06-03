// padding_query/mod.rs
//
// Padding query boundary facade for WASAPI IAudioClient::GetCurrentPadding smoke.
//
// This module provides a cross-platform interface to probe whether
// IAudioClient::GetCurrentPadding can be called after a successful
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
pub mod padding_query_steps;
pub mod report;
pub mod report_builders;
pub mod report_defaults;
pub mod report_failure_buffer_builders;
pub mod report_failure_builders;
pub mod report_failure_padding_builders;
pub mod report_failure_prereq_builders;
pub mod report_skipped_builders;
pub mod report_success_builders;

#[cfg(windows)]
pub use padding_query_windows::*;

#[cfg(not(windows))]
pub use padding_query_stub::*;

#[cfg(windows)]
mod padding_query_windows;

#[cfg(not(windows))]
mod padding_query_stub;

// Re-export commonly used types
pub use report::{WasapiPaddingQuerySmokeReport, WASAPI_PADDING_QUERY_SMOKE_ENV};
