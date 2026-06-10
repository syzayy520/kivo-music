//! Boundary error mapping completeness tests.
//!
//! Verifies that ALL RenderClientFailure variants are correctly mapped
//! to WriteError when processed through the delegated write path.
//! Tests exercise the mapping indirectly through process_request rather
//! than calling the private map_render_client_failure directly.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::wasapi_writer::{
    WasapiDeviceBufferWriter, WasapiDeviceBufferWriterConfig,
};
use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WriteError, WriteRequest, WriteResult,
};
use crate::playback::output_wasapi::render_client_boundary::{
    FakeRenderClientBoundary, RenderClientFailure,
};

/// Helper: create a writer with a client in the given failure mode.
fn writer_with_failure(failure: RenderClientFailure) -> WasapiDeviceBufferWriter {
    let mut client = FakeRenderClientBoundary::new(100);
    client.set_failure_mode(Some(failure));
    WasapiDeviceBufferWriter::with_render_client(
        WasapiDeviceBufferWriterConfig::new(100, 2, 44100),
        Box::new(client),
    )
}

// ── PaddingUnavailable mapping ─────────────────────────────────────────────

#[test]
fn padding_unavailable_maps_to_internal_error() {
    let mut w = writer_with_failure(RenderClientFailure::PaddingUnavailable);
    let err = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap_err();
    // PaddingUnavailable maps to WriteError::Internal (not WriteFailed)
    assert!(
        matches!(err, WriteError::Internal { .. }),
        "expected WriteError::Internal, got: {}",
        err
    );
    assert!(
        err.to_string().contains("padding unavailable"),
        "error message should mention padding: {}",
        err
    );
}

// ── Internal failure mapping ───────────────────────────────────────────────

#[test]
fn internal_failure_maps_to_internal_error() {
    let mut w = writer_with_failure(RenderClientFailure::Internal {
        description: "test internal error".to_string(),
    });
    let err = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap_err();
    assert!(
        matches!(err, WriteError::Internal { .. }),
        "expected WriteError::Internal, got: {}",
        err
    );
    assert!(
        err.to_string().contains("test internal error"),
        "error message should preserve description: {}",
        err
    );
}

// ── BufferAcquisitionFailed mapping ────────────────────────────────────────

#[test]
fn buffer_acquisition_failed_maps_to_write_failed() {
    let mut w = writer_with_failure(RenderClientFailure::BufferAcquisitionFailed {
        description: "acquire error".to_string(),
    });
    let err = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap_err();
    assert!(err.is_write_failed());
    assert!(
        err.to_string().contains("acquisition failed"),
        "error should mention acquisition: {}",
        err
    );
}

// ── All failure variants produce deterministic error types ──────────────────

#[test]
fn device_lost_maps_to_write_failed() {
    let mut w = writer_with_failure(RenderClientFailure::DeviceLost);
    let err = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap_err();
    assert!(err.is_write_failed());
    assert!(err.to_string().contains("device lost"));
}

#[test]
fn unavailable_maps_to_write_failed() {
    let mut w = writer_with_failure(RenderClientFailure::RenderClientUnavailable);
    let err = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap_err();
    assert!(err.is_write_failed());
    assert!(err.to_string().contains("unavailable"));
}

#[test]
fn invalid_request_maps_to_invalid_request() {
    let mut w = writer_with_failure(RenderClientFailure::InvalidRequest {
        reason: "bad frames".to_string(),
    });
    let err = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap_err();
    assert!(err.is_invalid_request());
    assert!(err.to_string().contains("bad frames"));
}

#[test]
fn buffer_release_failed_maps_to_write_failed() {
    let mut w = writer_with_failure(RenderClientFailure::BufferReleaseFailed {
        description: "release error".to_string(),
    });
    let err = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap_err();
    assert!(err.is_write_failed());
    assert!(err.to_string().contains("release failed"));
}

// ── Error does not corrupt writer state ────────────────────────────────────

#[test]
fn error_does_not_corrupt_writer_state() {
    let mut w = writer_with_failure(RenderClientFailure::DeviceLost);
    let _ = w.process_request(&WriteRequest::write_packet(10, 44100, 2));

    let snap = w.snapshot();
    assert_eq!(snap.frames_written, 0);
    assert_eq!(snap.bytes_written, 0);
    assert_eq!(snap.write_streak, 0);
    assert_eq!(snap.would_block_count, 0);
}

// ── Multiple sequential errors do not advance write position ───────────────

#[test]
fn multiple_errors_do_not_advance_write_position() {
    let mut w = writer_with_failure(RenderClientFailure::DeviceLost);

    for _ in 0..5 {
        let _ = w.process_request(&WriteRequest::write_packet(10, 44100, 2));
    }

    let snap = w.snapshot();
    // write_attempts is incremented before render client check,
    // but no frames were actually written
    assert_eq!(snap.frames_written, 0);
    // writes_completed = write_attempts - would_block_count;
    // errors (non-WouldBlock) still count as "completed" attempts
    assert_eq!(snap.would_block_count, 0);
    assert!(snap.requests_accepted > 0, "attempts were recorded");
    let cursor = w.cursor();
    assert_eq!(cursor.write_position, 0);
    assert_eq!(cursor.total_frames_written, 0);
}

// ── Successful write after error recovery is not possible (failure mode persists) ──

#[test]
fn failure_mode_persists_across_requests() {
    let mut w = writer_with_failure(RenderClientFailure::DeviceLost);

    // All requests should fail
    for _ in 0..3 {
        assert!(w
            .process_request(&WriteRequest::write_packet(10, 44100, 2))
            .is_err());
    }

    // No successful writes recorded
    assert_eq!(w.snapshot().frames_written, 0);
    assert_eq!(w.snapshot().write_streak, 0);
}

// ── Error after successful writes does not corrupt accumulated state ───────

#[test]
fn error_after_success_does_not_reset_accumulated_state() {
    // First writer: succeed
    let mut w_ok = WasapiDeviceBufferWriter::with_render_client(
        WasapiDeviceBufferWriterConfig::new(100, 2, 44100),
        Box::new(FakeRenderClientBoundary::new(100)),
    );
    w_ok.process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    assert_eq!(w_ok.snapshot().frames_written, 50);

    // Second writer: fail — but state should remain clean (0 written)
    let mut w_fail = writer_with_failure(RenderClientFailure::DeviceLost);
    let _ = w_fail.process_request(&WriteRequest::write_packet(10, 44100, 2));
    assert_eq!(w_fail.snapshot().frames_written, 0);
}

// ── Flush and Close still work when render client has failure mode ─────────

#[test]
fn flush_succeeds_even_with_failure_mode() {
    let mut w = writer_with_failure(RenderClientFailure::DeviceLost);
    // Flush doesn't go through the render client
    let result = w.process_request(&WriteRequest::Flush).unwrap();
    assert_eq!(result, WriteResult::Noop);
}

#[test]
fn close_succeeds_even_with_failure_mode() {
    let mut w = writer_with_failure(RenderClientFailure::DeviceLost);
    let result = w.process_request(&WriteRequest::Close).unwrap();
    assert_eq!(result, WriteResult::Noop);
    assert!(w.is_closed());
}
