//! Render client boundary trait.
//!
//! Abstract interface for WASAPI render client operations.
//! No real Windows API calls, no COM objects, no audio data processing.
//! Device-agnostic boundary for future real WASAPI render client integration.

use super::types::{
    AvailableFramesSnapshot, BufferAcquireRequest, BufferAcquireResult, BufferReleaseRequest,
    BufferReleaseResult, PaddingSnapshot, RenderClientFailure,
};

/// Trait for abstracting WASAPI render client operations.
///
/// This trait defines the interface for interacting with a render client
/// without making real Windows API calls. Implementations can be:
/// - Real WASAPI render client (future)
/// - Mock/stub for testing
/// - Placeholder for development
///
/// **IMPORTANT**: This trait does NOT:
/// - Call GetBuffer / ReleaseBuffer
/// - Own COM objects
/// - Process audio data
/// - Make real Windows API calls
pub trait RenderClientBoundary {
    /// Acquire a buffer from the render client.
    ///
    /// Returns a `BufferAcquireResult` indicating success or failure.
    /// On success, the buffer is considered acquired until `release_buffer` is called.
    fn acquire_buffer(
        &mut self,
        request: &BufferAcquireRequest,
    ) -> Result<BufferAcquireResult, RenderClientFailure>;

    /// Release a previously acquired buffer.
    ///
    /// Returns a `BufferReleaseResult` indicating success or failure.
    /// Must be called after `acquire_buffer` to release the buffer.
    fn release_buffer(
        &mut self,
        request: &BufferReleaseRequest,
    ) -> Result<BufferReleaseResult, RenderClientFailure>;

    /// Query the current padding in the buffer.
    ///
    /// Returns a `PaddingSnapshot` with the current padding information.
    fn query_padding(&self) -> Result<PaddingSnapshot, RenderClientFailure>;

    /// Query the number of available frames in the buffer.
    ///
    /// Returns an `AvailableFramesSnapshot` with the current available frames.
    fn query_available_frames(&self) -> Result<AvailableFramesSnapshot, RenderClientFailure>;

    /// Check if the render client is ready to accept requests.
    fn is_ready(&self) -> bool;

    /// Check if the render client has encountered an error.
    fn has_error(&self) -> bool;

    /// Get the last error that occurred, if any.
    fn last_error(&self) -> Option<RenderClientFailure>;
}
