// initialize/mod.rs
//
// Initialize boundary facade for WASAPI IAudioClient::Initialize shared-mode smoke.
//
// This module provides a cross-platform interface to probe whether
// IAudioClient::Initialize can be called in shared mode.
//
// **IMPORTANT**: This module does NOT:
//   - Call IsFormatSupported
//   - Call GetService
//   - Get IAudioRenderClient
//   - Call GetBuffer / ReleaseBuffer
//   - Call Start / Stop / Reset
//   - Produce audible output
//   - Operate outside of explicit opt-in tests

pub mod env;
pub mod format_fields;
pub mod guards;
pub mod initialize_steps;
pub mod report;
pub mod report_builders;
pub mod report_defaults;
pub mod report_failure_builders;
pub mod report_skipped_builders;
pub mod report_success_builders;

#[cfg(windows)]
pub use initialize_windows::*;

#[cfg(not(windows))]
pub use initialize_stub::*;

#[cfg(windows)]
mod initialize_windows;

#[cfg(not(windows))]
mod initialize_stub;

// Re-export commonly used types
pub use report::{WasapiClientInitializeSmokeReport, WASAPI_CLIENT_INIT_SMOKE_ENV};
