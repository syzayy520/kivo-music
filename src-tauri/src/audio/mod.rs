//! Audio subsystem facade.
//!
//! Keep this module as the stable public surface.
//! Internals are split into submodules for maintainability.

pub mod commands;
pub mod diagnostics;
pub mod errors;
pub mod logging;
pub mod validation;

pub mod buffer;
pub mod decoder_ffmpeg;
pub mod output_wasapi;
pub mod pipeline;
pub mod state;

// WASAPI Shared smoke harness (platform-gated internally).
pub mod diag;
pub mod error;
pub mod ring;
pub mod smoke;

// Stable re-exports for early bring-up / examples.
pub use diag::DiagWriter;
pub use diagnostics::AudioDiagnostics;
pub use errors::{AudioError, AudioResult};
pub use smoke::{
    run_smoke_test_blocking, spawn_smoke_test, SmokeMode, SmokeTestConfig, SmokeTestSummary,
};
