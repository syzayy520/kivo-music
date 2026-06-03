// render_client/mod.rs
//
// Render client boundary facade for WASAPI IAudioClient::GetService smoke.
//
// This module provides a cross-platform interface to probe whether
// IAudioClient::GetService can obtain an IAudioRenderClient after
// a successful Initialize in shared mode.
//
// **IMPORTANT**: This module does NOT:
//   - Call IsFormatSupported
//   - Call GetBuffer / ReleaseBuffer
//   - Call Start / Stop / Reset
//   - Produce audible output
//   - Operate outside of explicit opt-in tests

pub mod env;
pub mod format_fields;
pub mod guards;
pub mod render_client_steps;
pub mod report;
pub mod report_builders;
pub mod report_defaults;
pub mod report_failure_builders;
pub mod report_skipped_builders;
pub mod report_success_builders;

#[cfg(windows)]
pub use render_client_windows::*;

#[cfg(not(windows))]
pub use render_client_stub::*;

#[cfg(windows)]
mod render_client_windows;

#[cfg(not(windows))]
mod render_client_stub;

// Re-export commonly used types
pub use report::{WasapiRenderClientSmokeReport, WASAPI_RENDER_CLIENT_SMOKE_ENV};
