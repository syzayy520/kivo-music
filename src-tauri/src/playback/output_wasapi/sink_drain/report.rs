//! Report types for RingBuffer drain operations.

/// Report from a successful RingBuffer drain operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WasapiRingBufferDrainReport {
    /// Maximum frames requested to drain.
    pub(super) requested_frames: u32,
    /// Frames actually peeked from RingBuffer.
    pub(super) peeked_frames: u32,
    /// Frames successfully rendered to WASAPI buffer.
    pub(super) rendered_frames: u32,
    /// Frames consumed from RingBuffer after successful render.
    pub(super) consumed_frames: u32,
    /// Bytes written to render buffer.
    pub(super) bytes_rendered: usize,
    /// pending_frames before drain.
    pub(super) pending_before: usize,
    /// pending_frames after drain.
    pub(super) pending_after: usize,
}
