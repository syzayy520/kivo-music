//! FakeRenderClientBoundary struct definition, constructors, and accessors.
//!
//! Holds the mutable state for a fake render client that simulates
//! buffer acquire/release/query operations without real WASAPI calls.

use crate::playback::output_wasapi::render_client_boundary::RenderClientFailure;

/// Fake render client boundary for testing.
///
/// Simulates render client behavior with configurable capacity and failure modes.
/// Pure memory implementation, no real WASAPI dependencies.
#[derive(Debug)]
pub struct FakeRenderClientBoundary {
    /// Buffer capacity in frames.
    pub(super) capacity: u32,
    /// Current padding (frames in buffer).
    pub(super) padding: u32,
    /// Whether the client is ready.
    pub(super) ready: bool,
    /// Whether the client has an error.
    pub(super) has_error: bool,
    /// Last error that occurred.
    pub(super) last_error: Option<RenderClientFailure>,
    /// Whether the buffer is currently acquired.
    pub(super) buffer_acquired: bool,
    /// Number of frames acquired in current acquisition.
    pub(super) acquired_frames: u32,
    /// Failure mode for testing (if set, acquire_buffer will fail).
    pub(super) failure_mode: Option<RenderClientFailure>,
}

impl FakeRenderClientBoundary {
    /// Creates a new fake render client with the given capacity.
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            padding: 0,
            ready: true,
            has_error: false,
            last_error: None,
            buffer_acquired: false,
            acquired_frames: 0,
            failure_mode: None,
        }
    }

    /// Creates an empty fake render client with default capacity.
    pub fn empty() -> Self {
        Self::new(1024)
    }

    /// Creates a fake render client with specified capacity.
    pub fn with_capacity(capacity: u32) -> Self {
        Self::new(capacity)
    }

    /// Sets the failure mode for testing.
    pub fn set_failure_mode(&mut self, failure: Option<RenderClientFailure>) {
        self.failure_mode = failure;
    }

    /// Sets the ready state.
    pub fn set_ready(&mut self, ready: bool) {
        self.ready = ready;
    }

    /// Sets the padding value (for testing).
    pub fn set_padding(&mut self, padding: u32) {
        self.padding = padding.min(self.capacity);
    }

    /// Returns the current padding.
    pub fn padding(&self) -> u32 {
        self.padding
    }

    /// Returns the buffer capacity.
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Returns the number of available frames.
    pub fn available_frames(&self) -> u32 {
        self.capacity.saturating_sub(self.padding)
    }
}
