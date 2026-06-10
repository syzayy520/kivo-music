//! Real WASAPI render client adapter shell.
//!
//! Provides a pure shell that satisfies `RenderClientBoundary` without
//! making real Windows API calls. All buffer operations return errors
//! indicating the adapter is not connected (since no real device is wired).
//!
//! **IMPORTANT**: This struct does NOT:
//! - Call GetBuffer / ReleaseBuffer
//! - Hold COM pointers (IAudioRenderClient)
//! - Make real Windows API calls
//! - Process audio data
//!
//! It exists as an isolation boundary for future real WASAPI integration.

use super::failure::AdapterFailure;
use super::lifecycle::AdapterLifecycle;
use super::ownership::RenderClientOwnership;
use crate::playback::output_wasapi::render_client_boundary::{
    AvailableFramesSnapshot, BufferAcquireRequest, BufferAcquireResult, BufferReleaseRequest,
    BufferReleaseResult, PaddingSnapshot, RenderClientBoundary, RenderClientFailure,
};

/// Real WASAPI render client adapter shell.
///
/// Satisfies `RenderClientBoundary` as a pure shell. All operations
/// return errors indicating the adapter is not connected to a real device.
/// Future real WASAPI integration will add COM pointers here.
#[derive(Debug)]
pub struct RealRenderClientAdapter {
    lifecycle: AdapterLifecycle,
    ownership: RenderClientOwnership,
    last_error: Option<AdapterFailure>,
}

impl RealRenderClientAdapter {
    /// Creates a new adapter in NotConnected state.
    pub fn new() -> Self {
        Self {
            lifecycle: AdapterLifecycle::NotConnected,
            ownership: RenderClientOwnership::new(),
            last_error: None,
        }
    }

    /// Creates an adapter with device ownership info.
    pub fn with_device(device_id: String, format_description: String) -> Self {
        Self {
            lifecycle: AdapterLifecycle::NotConnected,
            ownership: RenderClientOwnership::with_device(device_id, format_description),
            last_error: None,
        }
    }

    /// Returns the current lifecycle state.
    pub fn lifecycle(&self) -> AdapterLifecycle {
        self.lifecycle
    }

    /// Returns a reference to the ownership info.
    pub fn ownership(&self) -> &RenderClientOwnership {
        &self.ownership
    }

    /// Returns the last adapter failure, if any.
    pub fn adapter_last_error(&self) -> Option<&AdapterFailure> {
        self.last_error.as_ref()
    }

    /// Transitions the adapter to Ready state.
    ///
    /// Returns true if the transition succeeded.
    pub fn connect(&mut self) -> bool {
        let success = self.lifecycle.transition_to_ready();
        if !success {
            self.last_error = Some(AdapterFailure::InvalidLifecycleState {
                current: self.lifecycle.to_string(),
                required: "NotConnected".to_string(),
            });
        }
        success
    }

    /// Transitions the adapter to DeviceLost state.
    pub fn mark_device_lost(&mut self) {
        if !self.lifecycle.transition_to_device_lost() {
            self.last_error = Some(AdapterFailure::InvalidLifecycleState {
                current: self.lifecycle.to_string(),
                required: "Ready".to_string(),
            });
        } else {
            self.last_error = Some(AdapterFailure::DeviceLost);
            self.ownership.clear();
        }
    }

    /// Transitions the adapter to Closed state.
    pub fn close(&mut self) {
        if !self.lifecycle.transition_to_closed() {
            self.last_error = Some(AdapterFailure::InvalidLifecycleState {
                current: self.lifecycle.to_string(),
                required: "NotConnected or Ready".to_string(),
            });
        } else {
            self.ownership.clear();
        }
    }

    /// Returns the failure for operations when not in Ready state.
    fn not_ready_failure(&self) -> RenderClientFailure {
        match self.lifecycle {
            AdapterLifecycle::NotConnected => {
                AdapterFailure::NotConnected.to_render_client_failure()
            }
            AdapterLifecycle::DeviceLost => AdapterFailure::DeviceLost.to_render_client_failure(),
            AdapterLifecycle::Closed => RenderClientFailure::RenderClientUnavailable,
            AdapterLifecycle::Ready => unreachable!("should not be called when Ready"),
        }
    }
}

impl Default for RealRenderClientAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderClientBoundary for RealRenderClientAdapter {
    fn acquire_buffer(
        &mut self,
        _request: &BufferAcquireRequest,
    ) -> Result<BufferAcquireResult, RenderClientFailure> {
        // Shell: always fails — no real device connected
        Err(self.not_ready_failure())
    }

    fn release_buffer(
        &mut self,
        _request: &BufferReleaseRequest,
    ) -> Result<BufferReleaseResult, RenderClientFailure> {
        // Shell: always fails — no real device connected
        Err(self.not_ready_failure())
    }

    fn query_padding(&self) -> Result<PaddingSnapshot, RenderClientFailure> {
        // Shell: always fails — no real device connected
        Err(self.not_ready_failure())
    }

    fn query_available_frames(&self) -> Result<AvailableFramesSnapshot, RenderClientFailure> {
        // Shell: always fails — no real device connected
        Err(self.not_ready_failure())
    }

    fn is_ready(&self) -> bool {
        self.lifecycle == AdapterLifecycle::Ready
    }

    fn has_error(&self) -> bool {
        self.last_error.is_some()
    }

    fn last_error(&self) -> Option<RenderClientFailure> {
        self.last_error
            .as_ref()
            .map(|e| e.to_render_client_failure())
    }
}
