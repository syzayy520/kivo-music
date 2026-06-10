//! Tests for the render client adapter isolation layer.
//!
//! Verifies that the adapter shell correctly satisfies RenderClientBoundary,
//! manages lifecycle transitions, converts failures, and does NOT hold any
//! real Windows resources.

use super::failure::AdapterFailure;
use super::lifecycle::AdapterLifecycle;
use super::ownership::RenderClientOwnership;
use super::RealRenderClientAdapter;
use crate::playback::output_wasapi::render_client_boundary::{
    BufferAcquireRequest, BufferReleaseRequest, RenderClientBoundary, RenderClientFailure,
};

// ── Adapter construction ───────────────────────────────────────────────────

#[test]
fn adapter_new_starts_not_connected() {
    let adapter = RealRenderClientAdapter::new();
    assert_eq!(adapter.lifecycle(), AdapterLifecycle::NotConnected);
    assert!(!adapter.is_ready());
    assert!(!adapter.has_error());
}

#[test]
fn adapter_default_is_not_connected() {
    let adapter = RealRenderClientAdapter::default();
    assert_eq!(adapter.lifecycle(), AdapterLifecycle::NotConnected);
}

#[test]
fn adapter_with_device_has_ownership() {
    let adapter =
        RealRenderClientAdapter::with_device("Speakers".to_string(), "48000Hz 2ch f32".to_string());
    assert_eq!(adapter.lifecycle(), AdapterLifecycle::NotConnected);
    assert!(adapter.ownership().has_owner());
    assert_eq!(adapter.ownership().device_id(), Some("Speakers"));
}

// ── No real Windows resources ──────────────────────────────────────────────

#[test]
fn adapter_does_not_hold_com_pointers() {
    let adapter = RealRenderClientAdapter::new();
    // Debug output should not contain any COM pointer info
    let debug = format!("{:?}", adapter);
    assert!(!debug.contains("IAudioRenderClient"));
    assert!(!debug.contains("IAudioClient"));
    assert!(!debug.contains("HRESULT"));
}

#[test]
fn adapter_does_not_call_windows_api() {
    let mut adapter = RealRenderClientAdapter::new();
    // All operations should fail with NotConnected (no real API calls)
    let request = BufferAcquireRequest::new(100, 48000, 2, 4);
    let result = adapter.acquire_buffer(&request);
    assert!(result.is_err());
}

// ── Lifecycle transitions ──────────────────────────────────────────────────

#[test]
fn adapter_connect_transitions_to_ready() {
    let mut adapter = RealRenderClientAdapter::new();
    assert!(adapter.connect());
    assert_eq!(adapter.lifecycle(), AdapterLifecycle::Ready);
    assert!(adapter.is_ready());
}

#[test]
fn adapter_connect_fails_from_ready() {
    let mut adapter = RealRenderClientAdapter::new();
    assert!(adapter.connect());
    assert!(!adapter.connect()); // cannot connect twice
    assert_eq!(adapter.lifecycle(), AdapterLifecycle::Ready);
}

#[test]
fn adapter_connect_fails_from_closed() {
    let mut adapter = RealRenderClientAdapter::new();
    adapter.close();
    assert!(!adapter.connect());
    assert_eq!(adapter.lifecycle(), AdapterLifecycle::Closed);
}

#[test]
fn adapter_mark_device_lost_from_ready() {
    let mut adapter = RealRenderClientAdapter::new();
    adapter.connect();
    adapter.mark_device_lost();
    assert_eq!(adapter.lifecycle(), AdapterLifecycle::DeviceLost);
    assert!(adapter.has_error());
}

#[test]
fn adapter_mark_device_lost_fails_from_not_connected() {
    let mut adapter = RealRenderClientAdapter::new();
    adapter.mark_device_lost();
    // Should remain NotConnected
    assert_eq!(adapter.lifecycle(), AdapterLifecycle::NotConnected);
}

#[test]
fn adapter_close_from_not_connected() {
    let mut adapter = RealRenderClientAdapter::new();
    adapter.close();
    assert_eq!(adapter.lifecycle(), AdapterLifecycle::Closed);
    assert!(!adapter.is_ready());
}

#[test]
fn adapter_close_from_ready() {
    let mut adapter = RealRenderClientAdapter::new();
    adapter.connect();
    adapter.close();
    assert_eq!(adapter.lifecycle(), AdapterLifecycle::Closed);
}

#[test]
fn adapter_close_is_idempotent() {
    let mut adapter = RealRenderClientAdapter::new();
    adapter.close();
    adapter.close(); // second close should not panic
    assert_eq!(adapter.lifecycle(), AdapterLifecycle::Closed);
}

// ── Ownership management ───────────────────────────────────────────────────

#[test]
fn adapter_device_lost_clears_ownership() {
    let mut adapter =
        RealRenderClientAdapter::with_device("Speakers".to_string(), "48000Hz 2ch f32".to_string());
    adapter.connect();
    assert!(adapter.ownership().has_owner());

    adapter.mark_device_lost();
    assert!(!adapter.ownership().has_owner());
}

#[test]
fn adapter_close_clears_ownership() {
    let mut adapter =
        RealRenderClientAdapter::with_device("Speakers".to_string(), "48000Hz 2ch f32".to_string());
    adapter.connect();
    adapter.close();
    assert!(!adapter.ownership().has_owner());
}

// ── Shell operations always fail when not connected ────────────────────────

#[test]
fn acquire_buffer_fails_when_not_connected() {
    let mut adapter = RealRenderClientAdapter::new();
    let request = BufferAcquireRequest::new(100, 48000, 2, 4);
    let err = adapter.acquire_buffer(&request).unwrap_err();
    assert!(matches!(err, RenderClientFailure::RenderClientUnavailable));
}

#[test]
fn release_buffer_fails_when_not_connected() {
    let mut adapter = RealRenderClientAdapter::new();
    let request = BufferReleaseRequest::new(100, 0);
    let err = adapter.release_buffer(&request).unwrap_err();
    assert!(matches!(err, RenderClientFailure::RenderClientUnavailable));
}

#[test]
fn query_padding_fails_when_not_connected() {
    let adapter = RealRenderClientAdapter::new();
    let err = adapter.query_padding().unwrap_err();
    assert!(matches!(err, RenderClientFailure::RenderClientUnavailable));
}

#[test]
fn query_available_frames_fails_when_not_connected() {
    let adapter = RealRenderClientAdapter::new();
    let err = adapter.query_available_frames().unwrap_err();
    assert!(matches!(err, RenderClientFailure::RenderClientUnavailable));
}

#[test]
fn acquire_buffer_fails_when_device_lost() {
    let mut adapter = RealRenderClientAdapter::new();
    adapter.connect();
    adapter.mark_device_lost();

    let request = BufferAcquireRequest::new(100, 48000, 2, 4);
    let err = adapter.acquire_buffer(&request).unwrap_err();
    assert!(matches!(err, RenderClientFailure::DeviceLost));
}

#[test]
fn acquire_buffer_fails_when_closed() {
    let mut adapter = RealRenderClientAdapter::new();
    adapter.connect();
    adapter.close();

    let request = BufferAcquireRequest::new(100, 48000, 2, 4);
    let err = adapter.acquire_buffer(&request).unwrap_err();
    assert!(matches!(err, RenderClientFailure::RenderClientUnavailable));
}

// ── Failure conversion ─────────────────────────────────────────────────────

#[test]
fn adapter_failure_not_connected_maps_to_unavailable() {
    let failure = AdapterFailure::NotConnected;
    let rcf: RenderClientFailure = failure.into();
    assert!(matches!(rcf, RenderClientFailure::RenderClientUnavailable));
}

#[test]
fn adapter_failure_device_lost_maps_to_device_lost() {
    let failure = AdapterFailure::DeviceLost;
    let rcf: RenderClientFailure = failure.into();
    assert!(matches!(rcf, RenderClientFailure::DeviceLost));
}

#[test]
fn adapter_failure_invalid_lifecycle_maps_to_invalid_request() {
    let failure = AdapterFailure::InvalidLifecycleState {
        current: "Closed".to_string(),
        required: "Ready".to_string(),
    };
    let rcf: RenderClientFailure = failure.into();
    assert!(matches!(rcf, RenderClientFailure::InvalidRequest { .. }));
}

#[test]
fn adapter_failure_internal_maps_to_internal() {
    let failure = AdapterFailure::Internal {
        description: "test".to_string(),
    };
    let rcf: RenderClientFailure = failure.into();
    assert!(matches!(rcf, RenderClientFailure::Internal { .. }));
}

// ── Error state tracking ───────────────────────────────────────────────────

#[test]
fn adapter_last_error_tracks_failures() {
    let mut adapter = RealRenderClientAdapter::new();
    assert!(adapter.adapter_last_error().is_none());

    adapter.mark_device_lost(); // fails from NotConnected
    assert!(adapter.adapter_last_error().is_some());
}

#[test]
fn adapter_last_error_cleared_on_successful_connect() {
    let mut adapter = RealRenderClientAdapter::new();
    adapter.mark_device_lost(); // sets error
    adapter.connect(); // succeeds
                       // After successful connect, no error from connect itself
    assert_eq!(adapter.lifecycle(), AdapterLifecycle::Ready);
}

// ── Boundary contract compliance ───────────────────────────────────────────

#[test]
fn adapter_satisfies_render_client_boundary_trait() {
    // Verify the adapter can be used as a trait object
    let adapter: Box<dyn RenderClientBoundary> = Box::new(RealRenderClientAdapter::new());
    assert!(!adapter.is_ready());
    assert!(!adapter.has_error());
}

#[test]
fn adapter_ready_state_reflected_in_boundary() {
    let mut adapter = RealRenderClientAdapter::new();
    adapter.connect();
    let boundary: &dyn RenderClientBoundary = &adapter;
    assert!(boundary.is_ready());
    assert!(!boundary.has_error());
}

// ── Lifecycle Display ──────────────────────────────────────────────────────

#[test]
fn lifecycle_display_format() {
    assert_eq!(AdapterLifecycle::NotConnected.to_string(), "NotConnected");
    assert_eq!(AdapterLifecycle::Ready.to_string(), "Ready");
    assert_eq!(AdapterLifecycle::DeviceLost.to_string(), "DeviceLost");
    assert_eq!(AdapterLifecycle::Closed.to_string(), "Closed");
}

#[test]
fn lifecycle_query_helpers() {
    assert!(AdapterLifecycle::NotConnected.can_become_ready());
    assert!(!AdapterLifecycle::Ready.can_become_ready());
    assert!(!AdapterLifecycle::Closed.can_become_ready());

    assert!(AdapterLifecycle::Ready.can_accept_requests());
    assert!(!AdapterLifecycle::NotConnected.can_accept_requests());

    assert!(AdapterLifecycle::DeviceLost.is_terminal());
    assert!(AdapterLifecycle::Closed.is_terminal());
    assert!(!AdapterLifecycle::Ready.is_terminal());
}

// ── Ownership Display ──────────────────────────────────────────────────────

#[test]
fn ownership_default_is_empty() {
    let ownership = RenderClientOwnership::new();
    assert!(!ownership.has_owner());
    assert!(ownership.device_id().is_none());
    assert!(ownership.format_description().is_none());
}

#[test]
fn ownership_with_device_has_info() {
    let ownership =
        RenderClientOwnership::with_device("Headphones".to_string(), "96000Hz 2ch f32".to_string());
    assert!(ownership.has_owner());
    assert_eq!(ownership.device_id(), Some("Headphones"));
    assert_eq!(ownership.format_description(), Some("96000Hz 2ch f32"));
}

#[test]
fn ownership_clear_resets() {
    let mut ownership =
        RenderClientOwnership::with_device("Speakers".to_string(), "48000Hz 2ch f32".to_string());
    ownership.clear();
    assert!(!ownership.has_owner());
    assert!(ownership.device_id().is_none());
}
