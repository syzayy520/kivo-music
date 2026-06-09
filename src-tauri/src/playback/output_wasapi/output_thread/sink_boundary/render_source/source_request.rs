//! Render source request type.
//!
//! Represents a request from the output thread runtime to a render source.
//! Pure data — no behavior, no IO, no WASAPI.

/// Request from output thread runtime to render source.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum RenderSourceRequest {
    /// Read a batch of audio frames from the source.
    ReadPacket {
        /// Number of frames to read.
        frame_count: u64,
        /// Sample rate in Hz.
        sample_rate: u32,
        /// Number of audio channels.
        channel_count: u16,
    },
    /// Peek at source state without consuming data.
    Peek,
    /// Flush source internal buffers.
    Flush,
    /// No-op request (idle cycle).
    #[default]
    Noop,
}
