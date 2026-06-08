//! Sink request type.
//!
//! Represents a request from the output thread runtime to a sink consumer.
//! Pure data — no behavior, no IO, no WASAPI.

/// Request from output thread runtime to sink consumer.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum SinkRequest {
    /// Render a batch of frames.
    Render {
        /// Number of frames to render.
        frame_count: u64,
        /// Sample rate in Hz.
        sample_rate: u32,
        /// Number of audio channels.
        channel_count: u16,
    },
    /// Write silence to fill underrun.
    WriteSilence {
        /// Number of silent frames.
        frame_count: u64,
    },
    /// Flush pending buffers.
    Flush,
    /// No-op request (idle cycle).
    #[default]
    Noop,
}
