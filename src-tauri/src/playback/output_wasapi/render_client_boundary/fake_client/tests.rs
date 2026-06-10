//! Unit tests for FakeRenderClientBoundary.

use super::FakeRenderClientBoundary;
use crate::playback::output_wasapi::render_client_boundary::{
    types::{BufferAcquireRequest, BufferReleaseRequest, RenderClientFailure},
    RenderClientBoundary,
};

#[test]
fn fake_render_client_new() {
    let client = FakeRenderClientBoundary::new(1024);
    assert!(client.is_ready());
    assert!(!client.has_error());
    assert_eq!(client.capacity(), 1024);
    assert_eq!(client.padding(), 0);
    assert_eq!(client.available_frames(), 1024);
}

#[test]
fn fake_render_client_empty() {
    let client = FakeRenderClientBoundary::empty();
    assert!(client.is_ready());
    assert_eq!(client.capacity(), 1024);
}

#[test]
fn fake_render_client_acquire_buffer() {
    let mut client = FakeRenderClientBoundary::new(1024);
    let request = BufferAcquireRequest::new(100, 44100, 2, 4);
    let result = client.acquire_buffer(&request).unwrap();
    assert!(result.is_success());
    assert_eq!(result.frames_acquired, 100);
    assert_eq!(result.bytes_acquired, 100 * 2 * 4);
    assert_eq!(result.buffer_capacity_frames, 1024);
}

#[test]
fn fake_render_client_release_buffer() {
    let mut client = FakeRenderClientBoundary::new(1024);
    let acquire_request = BufferAcquireRequest::new(100, 44100, 2, 4);
    client.acquire_buffer(&acquire_request).unwrap();

    let release_request = BufferReleaseRequest::normal(100);
    let result = client.release_buffer(&release_request).unwrap();
    assert!(result.is_success());
    assert_eq!(result.frames_released, 100);
    assert_eq!(client.padding(), 100);
}

#[test]
fn fake_render_client_query_padding() {
    let mut client = FakeRenderClientBoundary::new(1024);
    let acquire_request = BufferAcquireRequest::new(100, 44100, 2, 4);
    client.acquire_buffer(&acquire_request).unwrap();

    let release_request = BufferReleaseRequest::normal(100);
    client.release_buffer(&release_request).unwrap();

    let padding = client.query_padding().unwrap();
    assert_eq!(padding.padding_frames, 100);
    assert_eq!(padding.buffer_capacity_frames, 1024);
    assert_eq!(padding.available_frames, 924);
}

#[test]
fn fake_render_client_query_available_frames() {
    let mut client = FakeRenderClientBoundary::new(1024);
    let acquire_request = BufferAcquireRequest::new(100, 44100, 2, 4);
    client.acquire_buffer(&acquire_request).unwrap();

    let release_request = BufferReleaseRequest::normal(100);
    client.release_buffer(&release_request).unwrap();

    let available = client.query_available_frames().unwrap();
    assert_eq!(available.available_frames, 924);
    assert_eq!(available.buffer_capacity_frames, 1024);
    assert_eq!(available.padding_frames, 100);
}

#[test]
fn fake_render_client_failure_mode() {
    let mut client = FakeRenderClientBoundary::new(1024);
    client.set_failure_mode(Some(RenderClientFailure::DeviceLost));

    let request = BufferAcquireRequest::new(100, 44100, 2, 4);
    let result = client.acquire_buffer(&request);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), RenderClientFailure::DeviceLost);
}

#[test]
fn fake_render_client_not_ready() {
    let mut client = FakeRenderClientBoundary::new(1024);
    client.set_ready(false);

    let request = BufferAcquireRequest::new(100, 44100, 2, 4);
    let result = client.acquire_buffer(&request);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        RenderClientFailure::RenderClientUnavailable
    );
}

#[test]
fn fake_render_client_buffer_too_small() {
    let mut client = FakeRenderClientBoundary::new(100);
    let request = BufferAcquireRequest::new(200, 44100, 2, 4);
    let result = client.acquire_buffer(&request);
    assert!(result.is_err());
    match result.unwrap_err() {
        RenderClientFailure::BufferTooSmall {
            requested_frames,
            available_frames,
        } => {
            assert_eq!(requested_frames, 200);
            assert_eq!(available_frames, 100);
        }
        _ => panic!("expected BufferTooSmall"),
    }
}

#[test]
fn fake_render_client_already_acquired() {
    let mut client = FakeRenderClientBoundary::new(1024);
    let request = BufferAcquireRequest::new(100, 44100, 2, 4);
    client.acquire_buffer(&request).unwrap();

    let request2 = BufferAcquireRequest::new(50, 44100, 2, 4);
    let result = client.acquire_buffer(&request2);
    assert!(result.is_err());
    match result.unwrap_err() {
        RenderClientFailure::BufferAcquisitionFailed { description } => {
            assert!(description.contains("already acquired"));
        }
        _ => panic!("expected BufferAcquisitionFailed"),
    }
}

#[test]
fn fake_render_client_release_without_acquire() {
    let mut client = FakeRenderClientBoundary::new(1024);
    let request = BufferReleaseRequest::normal(100);
    let result = client.release_buffer(&request);
    assert!(result.is_err());
    match result.unwrap_err() {
        RenderClientFailure::BufferReleaseFailed { description } => {
            assert!(description.contains("not acquired"));
        }
        _ => panic!("expected BufferReleaseFailed"),
    }
}

#[test]
fn fake_render_client_invalid_request() {
    let mut client = FakeRenderClientBoundary::new(1024);
    let request = BufferAcquireRequest::new(0, 44100, 2, 4);
    let result = client.acquire_buffer(&request);
    assert!(result.is_err());
    match result.unwrap_err() {
        RenderClientFailure::InvalidRequest { reason } => {
            assert!(reason.contains("frame_count must be > 0"));
        }
        _ => panic!("expected InvalidRequest"),
    }
}

#[test]
fn fake_render_client_multiple_acquires() {
    let mut client = FakeRenderClientBoundary::new(1024);

    let request1 = BufferAcquireRequest::new(100, 44100, 2, 4);
    client.acquire_buffer(&request1).unwrap();
    let release1 = BufferReleaseRequest::normal(100);
    client.release_buffer(&release1).unwrap();

    let request2 = BufferAcquireRequest::new(200, 44100, 2, 4);
    client.acquire_buffer(&request2).unwrap();
    let release2 = BufferReleaseRequest::normal(200);
    client.release_buffer(&release2).unwrap();

    assert_eq!(client.padding(), 300);
    assert_eq!(client.available_frames(), 724);
}

#[test]
fn fake_render_client_partial_release() {
    let mut client = FakeRenderClientBoundary::new(1024);
    let acquire_request = BufferAcquireRequest::new(100, 44100, 2, 4);
    client.acquire_buffer(&acquire_request).unwrap();

    let release_request = BufferReleaseRequest::normal(50);
    client.release_buffer(&release_request).unwrap();

    assert_eq!(client.padding(), 50);
    assert_eq!(client.available_frames(), 974);
}

#[test]
fn fake_render_client_set_padding() {
    let mut client = FakeRenderClientBoundary::new(1024);
    client.set_padding(500);
    assert_eq!(client.padding(), 500);
    assert_eq!(client.available_frames(), 524);
}

#[test]
fn fake_render_client_padding_capped_at_capacity() {
    let mut client = FakeRenderClientBoundary::new(100);
    client.set_padding(200);
    assert_eq!(client.padding(), 100);
    assert_eq!(client.available_frames(), 0);
}
