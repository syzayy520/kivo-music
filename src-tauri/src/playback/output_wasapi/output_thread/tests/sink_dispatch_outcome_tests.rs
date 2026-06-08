//! Sink dispatch outcome tests.
//!
//! Tests for DispatchOutcome and DispatchError types.

use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::{
    DispatchError, DispatchOutcome,
};

// ── DispatchOutcome tests ──────────────────────────────────────────────

#[test]
fn dispatch_outcome_success_has_frame_count() {
    let outcome = DispatchOutcome::Success {
        frames_processed: 100,
        bytes_written: 400,
    };
    match outcome {
        DispatchOutcome::Success {
            frames_processed,
            bytes_written,
        } => {
            assert_eq!(frames_processed, 100);
            assert_eq!(bytes_written, 400);
        }
        _ => panic!("expected Success"),
    }
}

#[test]
fn dispatch_outcome_success_clone() {
    let a = DispatchOutcome::Success {
        frames_processed: 50,
        bytes_written: 200,
    };
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn dispatch_outcome_success_equality() {
    let a = DispatchOutcome::Success {
        frames_processed: 10,
        bytes_written: 40,
    };
    let b = DispatchOutcome::Success {
        frames_processed: 10,
        bytes_written: 40,
    };
    assert_eq!(a, b);
}

#[test]
fn dispatch_outcome_success_inequality() {
    let a = DispatchOutcome::Success {
        frames_processed: 10,
        bytes_written: 40,
    };
    let b = DispatchOutcome::Success {
        frames_processed: 20,
        bytes_written: 80,
    };
    assert_ne!(a, b);
}

#[test]
fn dispatch_outcome_silence_filled() {
    let outcome = DispatchOutcome::SilenceFilled {
        frames_written: 256,
    };
    match outcome {
        DispatchOutcome::SilenceFilled { frames_written } => assert_eq!(frames_written, 256),
        _ => panic!("expected SilenceFilled"),
    }
}

#[test]
fn dispatch_outcome_skipped() {
    let outcome = DispatchOutcome::Skipped;
    assert_eq!(outcome, DispatchOutcome::Skipped);
}

#[test]
fn dispatch_outcome_noop_is_default() {
    let outcome = DispatchOutcome::default();
    assert_eq!(outcome, DispatchOutcome::Noop);
}

#[test]
fn dispatch_outcome_failed() {
    let outcome = DispatchOutcome::Failed;
    assert_eq!(outcome, DispatchOutcome::Failed);
}

#[test]
fn dispatch_outcome_debug() {
    let outcome = DispatchOutcome::Success {
        frames_processed: 1,
        bytes_written: 4,
    };
    let dbg = format!("{:?}", outcome);
    assert!(dbg.contains("Success"));
}

#[test]
fn dispatch_outcome_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a = DispatchOutcome::Noop;
    let b = DispatchOutcome::Noop;
    let mut h1 = DefaultHasher::new();
    let mut h2 = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

// ── DispatchError tests ────────────────────────────────────────────────

#[test]
fn dispatch_error_buffer_underrun() {
    let err = DispatchError::BufferUnderrun { frames_missing: 64 };
    match err {
        DispatchError::BufferUnderrun { frames_missing } => assert_eq!(frames_missing, 64),
        _ => panic!("expected BufferUnderrun"),
    }
}

#[test]
fn dispatch_error_device_lost() {
    let err = DispatchError::DeviceLost;
    assert_eq!(err, DispatchError::DeviceLost);
}

#[test]
fn dispatch_error_invalid_request() {
    let err = DispatchError::InvalidRequest {
        reason: "bad format".into(),
    };
    match err {
        DispatchError::InvalidRequest { reason } => assert_eq!(reason, "bad format"),
        _ => panic!("expected InvalidRequest"),
    }
}

#[test]
fn dispatch_error_internal() {
    let err = DispatchError::Internal {
        description: "something broke".into(),
    };
    match err {
        DispatchError::Internal { description } => assert_eq!(description, "something broke"),
        _ => panic!("expected Internal"),
    }
}

#[test]
fn dispatch_error_display_buffer_underrun() {
    let err = DispatchError::BufferUnderrun { frames_missing: 32 };
    let msg = format!("{}", err);
    assert!(msg.contains("32"));
    assert!(msg.contains("underrun"));
}

#[test]
fn dispatch_error_display_device_lost() {
    let err = DispatchError::DeviceLost;
    let msg = format!("{}", err);
    assert!(msg.contains("device lost"));
}

#[test]
fn dispatch_error_display_invalid_request() {
    let err = DispatchError::InvalidRequest {
        reason: "test".into(),
    };
    let msg = format!("{}", err);
    assert!(msg.contains("test"));
}

#[test]
fn dispatch_error_display_internal() {
    let err = DispatchError::Internal {
        description: "oops".into(),
    };
    let msg = format!("{}", err);
    assert!(msg.contains("oops"));
}

#[test]
fn dispatch_error_clone() {
    let a = DispatchError::DeviceLost;
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn dispatch_error_equality() {
    let a = DispatchError::BufferUnderrun { frames_missing: 10 };
    let b = DispatchError::BufferUnderrun { frames_missing: 10 };
    assert_eq!(a, b);
}

#[test]
fn dispatch_error_debug() {
    let err = DispatchError::Internal {
        description: "x".into(),
    };
    let dbg = format!("{:?}", err);
    assert!(dbg.contains("Internal"));
}
