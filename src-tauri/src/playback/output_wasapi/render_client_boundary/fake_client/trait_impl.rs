//! RenderClientBoundary trait implementation for FakeRenderClientBoundary.
//!
//! Delegates to simulate_acquire/simulate_release for buffer operations
//! and exposes query/readiness/error state from the fake client.

use super::FakeRenderClientBoundary;
use crate::playback::output_wasapi::render_client_boundary::trait_def::RenderClientBoundary;
use crate::playback::output_wasapi::render_client_boundary::types::{
    AvailableFramesSnapshot, BufferAcquireRequest, BufferAcquireResult, BufferReleaseRequest,
    BufferReleaseResult, PaddingSnapshot, RenderClientFailure,
};

impl RenderClientBoundary for FakeRenderClientBoundary {
    fn acquire_buffer(
        &mut self,
        request: &BufferAcquireRequest,
    ) -> Result<BufferAcquireResult, RenderClientFailure> {
        self.simulate_acquire(request)
    }

    fn release_buffer(
        &mut self,
        request: &BufferReleaseRequest,
    ) -> Result<BufferReleaseResult, RenderClientFailure> {
        self.simulate_release(request)
    }

    fn query_padding(&self) -> Result<PaddingSnapshot, RenderClientFailure> {
        if let Some(failure) = &self.failure_mode {
            return Err(failure.clone());
        }

        if !self.ready {
            return Err(RenderClientFailure::RenderClientUnavailable);
        }

        Ok(PaddingSnapshot::new(
            self.padding,
            self.capacity,
            self.available_frames(),
        ))
    }

    fn query_available_frames(&self) -> Result<AvailableFramesSnapshot, RenderClientFailure> {
        if let Some(failure) = &self.failure_mode {
            return Err(failure.clone());
        }

        if !self.ready {
            return Err(RenderClientFailure::RenderClientUnavailable);
        }

        Ok(AvailableFramesSnapshot::new(
            self.available_frames(),
            self.capacity,
            self.padding,
        ))
    }

    fn is_ready(&self) -> bool {
        self.ready
    }

    fn has_error(&self) -> bool {
        self.has_error
    }

    fn last_error(&self) -> Option<RenderClientFailure> {
        self.last_error.clone()
    }
}
