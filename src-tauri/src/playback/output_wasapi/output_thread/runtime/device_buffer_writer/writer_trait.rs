//! Device buffer writer contract trait.
//!
//! Device-agnostic trait defining the interface for a device buffer writer.
//! No WASAPI, no IO, no actual buffer writing.

use super::{WriteError, WriteRequest, WriteResult, WriterCursor, WriterState};

/// Trait for a device buffer writer that processes write requests.
///
/// This is a contract shell — implementations will be added later.
/// No WASAPI, no IO, no actual buffer writing.
pub trait DeviceBufferWriter {
    /// Process a write request and return a result.
    fn process_request(&mut self, request: &WriteRequest) -> Result<WriteResult, WriteError>;

    /// Get a snapshot of the writer's current state.
    fn snapshot(&self) -> WriterState;

    /// Get the current write cursor position.
    fn cursor(&self) -> WriterCursor;

    /// Check if the writer is ready to accept requests.
    fn is_ready(&self) -> bool;

    /// Check if the writer has been closed.
    fn is_closed(&self) -> bool;

    /// Reset the writer to its initial state.
    fn reset(&mut self);
}
