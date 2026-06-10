//! Buffer simulation logic for FakeRenderClientBoundary.
//!
//! Implements simulate_acquire and simulate_release which model
//! the behavior of a real WASAPI render client's buffer operations
//! without any actual Windows API calls.

use super::FakeRenderClientBoundary;
use crate::playback::output_wasapi::render_client_boundary::types::{
    BufferAcquireRequest, BufferAcquireResult, BufferReleaseRequest, BufferReleaseResult,
    RenderClientFailure,
};

impl FakeRenderClientBoundary {
    /// Simulates a buffer acquisition.
    ///
    /// Checks failure mode, readiness, request validity, and available space.
    /// On success, marks the buffer as acquired.
    pub(super) fn simulate_acquire(
        &mut self,
        request: &BufferAcquireRequest,
    ) -> Result<BufferAcquireResult, RenderClientFailure> {
        if let Some(failure) = &self.failure_mode {
            return Err(failure.clone());
        }

        if self.buffer_acquired {
            return Err(RenderClientFailure::BufferAcquisitionFailed {
                description: "buffer already acquired".to_string(),
            });
        }

        if !self.ready {
            return Err(RenderClientFailure::RenderClientUnavailable);
        }

        if request.frame_count == 0 {
            return Err(RenderClientFailure::InvalidRequest {
                reason: "frame_count must be > 0".to_string(),
            });
        }

        let available = self.available_frames();
        if request.frame_count > available {
            return Err(RenderClientFailure::BufferTooSmall {
                requested_frames: request.frame_count,
                available_frames: available,
            });
        }

        self.buffer_acquired = true;
        self.acquired_frames = request.frame_count;

        Ok(BufferAcquireResult::success(
            request.frame_count,
            request.total_bytes(),
            self.capacity,
        ))
    }

    /// Simulates a buffer release.
    ///
    /// Checks failure mode, acquisition state, and request validity.
    /// On success, updates padding and releases the buffer.
    pub(super) fn simulate_release(
        &mut self,
        request: &BufferReleaseRequest,
    ) -> Result<BufferReleaseResult, RenderClientFailure> {
        if let Some(failure) = &self.failure_mode {
            return Err(failure.clone());
        }

        if !self.buffer_acquired {
            return Err(RenderClientFailure::BufferReleaseFailed {
                description: "buffer not acquired".to_string(),
            });
        }

        if request.frames_written > self.acquired_frames {
            return Err(RenderClientFailure::InvalidRequest {
                reason: format!(
                    "frames_written ({}) > acquired_frames ({})",
                    request.frames_written, self.acquired_frames
                ),
            });
        }

        self.padding += request.frames_written;
        self.buffer_acquired = false;
        self.acquired_frames = 0;

        Ok(BufferReleaseResult::success(request.frames_written))
    }
}
