//! Buffer configuration type.
//!
//! Defines the runtime configuration for the output buffer.

/// Output buffer configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferConfig {
    /// Buffer capacity in frames.
    pub capacity_frames: u32,
    /// Low watermark in frames (triggers refill).
    pub low_watermark_frames: u32,
    /// High watermark in frames (triggers backpressure).
    pub high_watermark_frames: u32,
}

impl Default for BufferConfig {
    fn default() -> Self {
        Self {
            capacity_frames: 8192,
            low_watermark_frames: 1024,
            high_watermark_frames: 6144,
        }
    }
}
