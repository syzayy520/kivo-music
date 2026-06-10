//! Render client boundary contract types.
//!
//! Pure data types and traits for abstracting WASAPI render client operations.
//! No real Windows API calls, no COM objects, no audio data processing.
//! Device-agnostic boundary for future real WASAPI render client integration.

pub mod trait_def;
pub mod types;

// Re-export commonly used types
pub use trait_def::RenderClientBoundary;
pub use types::{
    AvailableFramesSnapshot, BufferAcquireRequest, BufferAcquireResult, BufferReleaseRequest,
    BufferReleaseResult, PaddingSnapshot, RenderClientFailure,
};
