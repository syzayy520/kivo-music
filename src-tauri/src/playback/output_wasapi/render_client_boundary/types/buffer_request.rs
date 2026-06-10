//! Buffer request types for render client boundary.
//!
//! Pure metadata — no actual buffer manipulation.

/// Request to acquire a buffer from the render client.
///
/// Pure metadata — no actual buffer manipulation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BufferAcquireRequest {
    /// Number of frames to acquire.
    pub frame_count: u32,
    /// Sample rate in Hz.
    pub sample_rate: u32,
    /// Number of audio channels.
    pub channel_count: u16,
    /// Bytes per sample (e.g., 4 for f32).
    pub bytes_per_sample: u16,
}

impl BufferAcquireRequest {
    /// Creates a new buffer acquire request.
    pub fn new(
        frame_count: u32,
        sample_rate: u32,
        channel_count: u16,
        bytes_per_sample: u16,
    ) -> Self {
        Self {
            frame_count,
            sample_rate,
            channel_count,
            bytes_per_sample,
        }
    }

    /// Returns the total bytes required for the buffer.
    pub fn total_bytes(&self) -> u64 {
        self.frame_count as u64 * self.channel_count as u64 * self.bytes_per_sample as u64
    }
}

/// Request to release a buffer back to the render client.
///
/// Pure metadata — no actual buffer manipulation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BufferReleaseRequest {
    /// Number of frames written to the buffer.
    pub frames_written: u32,
    /// Flags for the release operation (e.g., 0 for normal release).
    pub flags: u32,
}

impl BufferReleaseRequest {
    /// Creates a new buffer release request.
    pub fn new(frames_written: u32, flags: u32) -> Self {
        Self {
            frames_written,
            flags,
        }
    }

    /// Creates a normal release request.
    pub fn normal(frames_written: u32) -> Self {
        Self::new(frames_written, 0)
    }
}
