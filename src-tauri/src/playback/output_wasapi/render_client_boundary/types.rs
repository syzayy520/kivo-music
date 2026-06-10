//! Render client boundary pure data types.
//!
//! Pure data types for abstracting WASAPI render client operations.
//! No real Windows API calls, no COM objects, no audio data processing.
//! Device-agnostic boundary for future real WASAPI render client integration.

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

/// Snapshot of the current padding in the buffer.
///
/// Pure metadata — no actual buffer data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PaddingSnapshot {
    /// Number of frames in the buffer that have been played.
    pub padding_frames: u32,
    /// Total buffer capacity in frames.
    pub buffer_capacity_frames: u32,
    /// Number of frames available for writing.
    pub available_frames: u32,
}

impl PaddingSnapshot {
    /// Creates a new padding snapshot.
    pub fn new(padding_frames: u32, buffer_capacity_frames: u32, available_frames: u32) -> Self {
        Self {
            padding_frames,
            buffer_capacity_frames,
            available_frames,
        }
    }

    /// Returns the buffer fill percentage (0-100).
    pub fn fill_percentage(&self) -> u8 {
        if self.buffer_capacity_frames == 0 {
            0
        } else {
            ((self.padding_frames * 100) / self.buffer_capacity_frames).min(100) as u8
        }
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.padding_frames == 0
    }

    /// Returns true if the buffer is full.
    pub fn is_full(&self) -> bool {
        self.buffer_capacity_frames > 0 && self.padding_frames >= self.buffer_capacity_frames
    }
}

/// Snapshot of the available frames in the buffer.
///
/// Pure metadata — no actual buffer data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AvailableFramesSnapshot {
    /// Number of frames available for writing.
    pub available_frames: u32,
    /// Total buffer capacity in frames.
    pub buffer_capacity_frames: u32,
    /// Number of frames currently in the buffer (padding).
    pub padding_frames: u32,
}

impl AvailableFramesSnapshot {
    /// Creates a new available frames snapshot.
    pub fn new(available_frames: u32, buffer_capacity_frames: u32, padding_frames: u32) -> Self {
        Self {
            available_frames,
            buffer_capacity_frames,
            padding_frames,
        }
    }

    /// Returns the available frames as a percentage of capacity (0-100).
    pub fn available_percentage(&self) -> u8 {
        if self.buffer_capacity_frames == 0 {
            0
        } else {
            ((self.available_frames * 100) / self.buffer_capacity_frames).min(100) as u8
        }
    }

    /// Returns true if there are frames available for writing.
    pub fn has_available_frames(&self) -> bool {
        self.available_frames > 0
    }

    /// Returns true if the buffer is full (no available frames).
    pub fn is_full(&self) -> bool {
        self.available_frames == 0 && self.buffer_capacity_frames > 0
    }
}

/// Failure reasons for render client operations.
///
/// Pure metadata — no actual Windows API error codes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RenderClientFailure {
    /// Device was lost (e.g., unplugged).
    DeviceLost,
    /// Buffer is too small for the requested operation.
    BufferTooSmall {
        /// Requested size in frames.
        requested_frames: u32,
        /// Available size in frames.
        available_frames: u32,
    },
    /// Padding information is unavailable.
    PaddingUnavailable,
    /// Render client is unavailable (e.g., not initialized).
    RenderClientUnavailable,
    /// Invalid request parameters.
    InvalidRequest {
        /// Reason for rejection.
        reason: String,
    },
    /// Buffer acquisition failed.
    BufferAcquisitionFailed {
        /// Error description.
        description: String,
    },
    /// Buffer release failed.
    BufferReleaseFailed {
        /// Error description.
        description: String,
    },
    /// Internal error.
    Internal {
        /// Error description.
        description: String,
    },
}

impl std::fmt::Display for RenderClientFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeviceLost => write!(f, "render client device lost"),
            Self::BufferTooSmall {
                requested_frames,
                available_frames,
            } => write!(
                f,
                "render client buffer too small: requested {}, available {}",
                requested_frames, available_frames
            ),
            Self::PaddingUnavailable => write!(f, "render client padding unavailable"),
            Self::RenderClientUnavailable => write!(f, "render client unavailable"),
            Self::InvalidRequest { reason } => {
                write!(f, "render client invalid request: {}", reason)
            }
            Self::BufferAcquisitionFailed { description } => {
                write!(
                    f,
                    "render client buffer acquisition failed: {}",
                    description
                )
            }
            Self::BufferReleaseFailed { description } => {
                write!(f, "render client buffer release failed: {}", description)
            }
            Self::Internal { description } => {
                write!(f, "render client internal error: {}", description)
            }
        }
    }
}

impl RenderClientFailure {
    /// Returns true if this is a fatal error (device lost).
    pub fn is_fatal(&self) -> bool {
        matches!(self, Self::DeviceLost)
    }

    /// Returns true if this is a recoverable error.
    pub fn is_recoverable(&self) -> bool {
        !self.is_fatal()
    }

    /// Returns true if this is a buffer size error.
    pub fn is_buffer_size_error(&self) -> bool {
        matches!(self, Self::BufferTooSmall { .. })
    }

    /// Returns true if this is a padding error.
    pub fn is_padding_error(&self) -> bool {
        matches!(self, Self::PaddingUnavailable)
    }

    /// Returns true if this is a render client availability error.
    pub fn is_availability_error(&self) -> bool {
        matches!(self, Self::RenderClientUnavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_acquire_request_total_bytes() {
        let request = BufferAcquireRequest::new(100, 44100, 2, 4);
        assert_eq!(request.total_bytes(), 100 * 2 * 4);
    }

    #[test]
    fn buffer_acquire_result_success() {
        let result = BufferAcquireResult::success(100, 800, 1024);
        assert!(result.is_success());
        assert_eq!(result.frames_acquired, 100);
        assert_eq!(result.bytes_acquired, 800);
        assert_eq!(result.buffer_capacity_frames, 1024);
    }

    #[test]
    fn buffer_acquire_result_failure() {
        let result = BufferAcquireResult::failure(1024);
        assert!(!result.is_success());
        assert_eq!(result.frames_acquired, 0);
        assert_eq!(result.bytes_acquired, 0);
        assert_eq!(result.buffer_capacity_frames, 1024);
    }

    #[test]
    fn buffer_release_request_normal() {
        let request = BufferReleaseRequest::normal(100);
        assert_eq!(request.frames_written, 100);
        assert_eq!(request.flags, 0);
    }

    #[test]
    fn buffer_release_result_success() {
        let result = BufferReleaseResult::success(100);
        assert!(result.is_success());
        assert_eq!(result.frames_released, 100);
    }

    #[test]
    fn buffer_release_result_failure() {
        let result = BufferReleaseResult::failure();
        assert!(!result.is_success());
        assert_eq!(result.frames_released, 0);
    }

    #[test]
    fn padding_snapshot_fill_percentage() {
        let snapshot = PaddingSnapshot::new(50, 100, 50);
        assert_eq!(snapshot.fill_percentage(), 50);
    }

    #[test]
    fn padding_snapshot_empty() {
        let snapshot = PaddingSnapshot::new(0, 100, 100);
        assert!(snapshot.is_empty());
        assert!(!snapshot.is_full());
    }

    #[test]
    fn padding_snapshot_full() {
        let snapshot = PaddingSnapshot::new(100, 100, 0);
        assert!(!snapshot.is_empty());
        assert!(snapshot.is_full());
    }

    #[test]
    fn available_frames_snapshot_percentage() {
        let snapshot = AvailableFramesSnapshot::new(50, 100, 50);
        assert_eq!(snapshot.available_percentage(), 50);
    }

    #[test]
    fn available_frames_snapshot_has_available() {
        let snapshot = AvailableFramesSnapshot::new(50, 100, 50);
        assert!(snapshot.has_available_frames());
        assert!(!snapshot.is_full());
    }

    #[test]
    fn available_frames_snapshot_full() {
        let snapshot = AvailableFramesSnapshot::new(0, 100, 100);
        assert!(!snapshot.has_available_frames());
        assert!(snapshot.is_full());
    }

    #[test]
    fn render_client_failure_fatal() {
        assert!(RenderClientFailure::DeviceLost.is_fatal());
        assert!(!RenderClientFailure::DeviceLost.is_recoverable());
    }

    #[test]
    fn render_client_failure_recoverable() {
        let failure = RenderClientFailure::BufferTooSmall {
            requested_frames: 100,
            available_frames: 50,
        };
        assert!(!failure.is_fatal());
        assert!(failure.is_recoverable());
        assert!(failure.is_buffer_size_error());
    }

    #[test]
    fn render_client_failure_padding() {
        assert!(RenderClientFailure::PaddingUnavailable.is_padding_error());
    }

    #[test]
    fn render_client_failure_availability() {
        assert!(RenderClientFailure::RenderClientUnavailable.is_availability_error());
    }
}
