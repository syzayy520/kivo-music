use super::output_thread_errors::{OutputThreadErrorKind, OutputThreadErrorSummary};

// ---------------------------------------------------------------------------
// OutputThreadErrorKind classification
// ---------------------------------------------------------------------------

#[test]
fn error_kind_codes_are_stable() {
    assert_eq!(OutputThreadErrorKind::ThreadPanic.as_code(), "thread_panic");
    assert_eq!(
        OutputThreadErrorKind::ShutdownTimeout.as_code(),
        "shutdown_timeout"
    );
    assert_eq!(
        OutputThreadErrorKind::OutputThreadNotRunning.as_code(),
        "output_thread_not_running"
    );
    assert_eq!(
        OutputThreadErrorKind::RingBufferNotReady.as_code(),
        "ring_buffer_not_ready"
    );
    assert_eq!(OutputThreadErrorKind::BufferFull.as_code(), "buffer_full");
    assert_eq!(
        OutputThreadErrorKind::DeviceNotReady.as_code(),
        "device_not_ready"
    );
    assert_eq!(OutputThreadErrorKind::DeviceLost.as_code(), "device_lost");
    assert_eq!(
        OutputThreadErrorKind::RenderFailed.as_code(),
        "render_failed"
    );
    assert_eq!(
        OutputThreadErrorKind::UnsupportedFormat.as_code(),
        "unsupported_format"
    );
    assert_eq!(OutputThreadErrorKind::Closed.as_code(), "closed");
}

#[test]
fn lifecycle_errors_are_classified() {
    assert!(OutputThreadErrorKind::ThreadPanic.is_lifecycle_error());
    assert!(OutputThreadErrorKind::ShutdownTimeout.is_lifecycle_error());
    assert!(OutputThreadErrorKind::OutputThreadNotRunning.is_lifecycle_error());
    assert!(OutputThreadErrorKind::RingBufferNotReady.is_lifecycle_error());
    assert!(OutputThreadErrorKind::Closed.is_lifecycle_error());
}

#[test]
fn device_errors_are_classified() {
    assert!(OutputThreadErrorKind::DeviceNotReady.is_device_error());
    assert!(OutputThreadErrorKind::DeviceLost.is_device_error());
    assert!(OutputThreadErrorKind::RenderFailed.is_device_error());
    assert!(OutputThreadErrorKind::UnsupportedFormat.is_device_error());
}

#[test]
fn backpressure_error_is_classified() {
    assert!(OutputThreadErrorKind::BufferFull.is_backpressure());
    // Verify non-backpressure kinds return false.
    assert!(!OutputThreadErrorKind::ThreadPanic.is_backpressure());
    assert!(!OutputThreadErrorKind::DeviceLost.is_backpressure());
}

// ---------------------------------------------------------------------------
// OutputThreadErrorSummary
// ---------------------------------------------------------------------------

#[test]
fn error_summary_preserves_kind_and_message() {
    let summary = OutputThreadErrorSummary::new(
        OutputThreadErrorKind::DeviceLost,
        "audio device disconnected",
    );
    assert_eq!(summary.kind, OutputThreadErrorKind::DeviceLost);
    assert_eq!(summary.message, "audio device disconnected");
}

#[test]
fn error_summary_accepts_string_like_message() {
    let summary =
        OutputThreadErrorSummary::new(OutputThreadErrorKind::BufferFull, "buffer full".to_string());
    assert_eq!(summary.kind, OutputThreadErrorKind::BufferFull);
    assert_eq!(summary.message, "buffer full");
}
