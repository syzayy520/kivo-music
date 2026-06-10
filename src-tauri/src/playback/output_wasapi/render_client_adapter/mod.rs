//! Real WASAPI render client adapter isolation layer.
//!
//! Provides a pure shell adapter that satisfies `RenderClientBoundary` without
//! making real Windows API calls. This family isolates future real WASAPI
//! concerns (COM lifecycle, GetBuffer/ReleaseBuffer, IAudioRenderClient ownership)
//! from the boundary contract and the device buffer writer.
//!
//! **IMPORTANT**: This module does NOT:
//! - Call GetBuffer / ReleaseBuffer
//! - Own COM objects (IAudioRenderClient)
//! - Make real Windows API calls
//! - Process audio data
//!
//! It exists as an isolation shell for future real WASAPI integration.

mod failure;
mod lifecycle;
mod ownership;
mod shell;

#[cfg(test)]
mod tests;

pub use failure::AdapterFailure;
pub use lifecycle::AdapterLifecycle;
pub use ownership::RenderClientOwnership;
pub use shell::RealRenderClientAdapter;
