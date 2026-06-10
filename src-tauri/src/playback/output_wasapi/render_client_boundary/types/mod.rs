//! Render client boundary pure data types.
//!
//! Pure data types for abstracting WASAPI render client operations.
//! No real Windows API calls, no COM objects, no audio data processing.
//! Device-agnostic boundary for future real WASAPI render client integration.

mod buffer_request;
mod buffer_result;
mod failure;
mod snapshot;

pub use buffer_request::{BufferAcquireRequest, BufferReleaseRequest};
pub use buffer_result::{BufferAcquireResult, BufferReleaseResult};
pub use failure::RenderClientFailure;
pub use snapshot::{AvailableFramesSnapshot, PaddingSnapshot};

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
