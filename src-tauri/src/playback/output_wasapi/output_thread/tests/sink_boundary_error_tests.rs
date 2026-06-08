//! Sink boundary error tests.
//!
//! Tests for SinkError type.

use crate::playback::output_wasapi::output_thread::sink_boundary::SinkError;

#[test]
fn sink_error_buffer_underrun() {
    let err = SinkError::BufferUnderrun { frames_missing: 100 };
    match err {
        SinkError::BufferUnderrun { frames_missing } => assert_eq!(frames_missing, 100),
        _ => panic!("expected BufferUnderrun"),
    }
}

#[test]
fn sink_error_device_lost() {
    let err = SinkError::DeviceLost;
    assert_eq!(err, SinkError::DeviceLost);
}

#[test]
fn sink_error_invalid_request() {
    let err = SinkError::InvalidRequest {
        reason: "bad format".to_string(),
    };
    match err {
        SinkError::InvalidRequest { reason } => assert_eq!(reason, "bad format"),
        _ => panic!("expected InvalidRequest"),
    }
}

#[test]
fn sink_error_internal() {
    let err = SinkError::Internal {
        description: "something broke".to_string(),
    };
    match err {
        SinkError::Internal { description } => assert_eq!(description, "something broke"),
        _ => panic!("expected Internal"),
    }
}

#[test]
fn sink_error_display_buffer_underrun() {
    let err = SinkError::BufferUnderrun { frames_missing: 50 };
    let display = format!("{}", err);
    assert!(display.contains("50"));
    assert!(display.contains("underrun"));
}

#[test]
fn sink_error_display_device_lost() {
    let err = SinkError::DeviceLost;
    let display = format!("{}", err);
    assert!(display.contains("device lost"));
}

#[test]
fn sink_error_display_invalid_request() {
    let err = SinkError::InvalidRequest {
        reason: "test".to_string(),
    };
    let display = format!("{}", err);
    assert!(display.contains("test"));
}

#[test]
fn sink_error_display_internal() {
    let err = SinkError::Internal {
        description: "oops".to_string(),
    };
    let display = format!("{}", err);
    assert!(display.contains("oops"));
}

#[test]
fn sink_error_clone() {
    let original = SinkError::BufferUnderrun { frames_missing: 10 };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn sink_error_debug() {
    let err = SinkError::DeviceLost;
    let debug = format!("{:?}", err);
    assert_eq!(debug, "DeviceLost");
}

#[test]
fn sink_error_equality() {
    let a = SinkError::Internal {
        description: "x".to_string(),
    };
    let b = SinkError::Internal {
        description: "x".to_string(),
    };
    assert_eq!(a, b);
}

#[test]
fn sink_error_inequality() {
    let a = SinkError::DeviceLost;
    let b = SinkError::BufferUnderrun { frames_missing: 1 };
    assert_ne!(a, b);
}
