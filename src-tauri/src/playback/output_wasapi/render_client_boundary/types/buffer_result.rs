//! Buffer result types for render client boundary.
//!
//! Pure metadata — no actual buffer pointer or data.

/// Result of acquiring a buffer from the render client.
///
/// Pure metadata — no actual buffer pointer or data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BufferAcquireResult {
    /// Number of frames actually acquired.
    pub frames_acquired: u32,
    /// Total bytes acquired.
    pub bytes_acquired: u64,
    /// Whether the buffer was successfully acquired.
    pub success: bool,
    /// Buffer capacity in frames.
    pub buffer_capacity_frames: u32,
}

impl BufferAcquireResult {
    /// Creates a successful buffer acquire result.
    pub fn success(frames_acquired: u32, bytes_acquired: u64, buffer_capacity_frames: u32) -> Self {
        Self {
            frames_acquired,
            bytes_acquired,
            success: true,
            buffer_capacity_frames,
        }
    }

    /// Creates a failed buffer acquire result.
    pub fn failure(buffer_capacity_frames: u32) -> Self {
        Self {
            frames_acquired: 0,
            bytes_acquired: 0,
            success: false,
            buffer_capacity_frames,
        }
    }

    /// Returns true if the buffer was successfully acquired.
    pub fn is_success(&self) -> bool {
        self.success
    }
}

/// Result of releasing a buffer back to the render client.
///
/// Pure metadata — no actual buffer manipulation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BufferReleaseResult {
    /// Whether the release was successful.
    pub success: bool,
    /// Number of frames actually released.
    pub frames_released: u32,
}

impl BufferReleaseResult {
    /// Creates a successful buffer release result.
    pub fn success(frames_released: u32) -> Self {
        Self {
            success: true,
            frames_released,
        }
    }

    /// Creates a failed buffer release result.
    pub fn failure() -> Self {
        Self {
            success: false,
            frames_released: 0,
        }
    }

    /// Returns true if the release was successful.
    pub fn is_success(&self) -> bool {
        self.success
    }
}
