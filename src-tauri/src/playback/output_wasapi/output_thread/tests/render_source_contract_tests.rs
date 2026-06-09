//! Render source contract tests.
//!
//! Tests for RenderSourceRequest, RenderSourceResult, and RenderSourceError types.

use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSourceError, RenderSourceRequest, RenderSourceResult,
};

// ===== RenderSourceRequest tests =====

#[test]
fn render_source_request_default_is_noop() {
    let req = RenderSourceRequest::default();
    assert_eq!(req, RenderSourceRequest::Noop);
}

#[test]
fn render_source_request_read_packet_equality() {
    let a = RenderSourceRequest::ReadPacket {
        frame_count: 1024,
        sample_rate: 44100,
        channel_count: 2,
    };
    let b = RenderSourceRequest::ReadPacket {
        frame_count: 1024,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_eq!(a, b);
}

#[test]
fn render_source_request_read_packet_inequality() {
    let a = RenderSourceRequest::ReadPacket {
        frame_count: 1024,
        sample_rate: 44100,
        channel_count: 2,
    };
    let b = RenderSourceRequest::ReadPacket {
        frame_count: 512,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_ne!(a, b);
}

#[test]
fn render_source_request_peek() {
    let req = RenderSourceRequest::Peek;
    assert_eq!(req, RenderSourceRequest::Peek);
}

#[test]
fn render_source_request_flush() {
    let req = RenderSourceRequest::Flush;
    assert_eq!(req, RenderSourceRequest::Flush);
}

#[test]
fn render_source_request_clone() {
    let original = RenderSourceRequest::ReadPacket {
        frame_count: 1024,
        sample_rate: 48000,
        channel_count: 2,
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn render_source_request_debug() {
    let req = RenderSourceRequest::Noop;
    let debug = format!("{:?}", req);
    assert_eq!(debug, "Noop");
}

// ===== RenderSourceResult tests =====

#[test]
fn render_source_result_default_is_noop() {
    let res = RenderSourceResult::default();
    assert_eq!(res, RenderSourceResult::Noop);
}

#[test]
fn render_source_result_packet() {
    let res = RenderSourceResult::Packet {
        frames_provided: 1024,
        bytes_read: 4096,
    };
    match res {
        RenderSourceResult::Packet {
            frames_provided,
            bytes_read,
        } => {
            assert_eq!(frames_provided, 1024);
            assert_eq!(bytes_read, 4096);
        }
        _ => panic!("expected Packet"),
    }
}

#[test]
fn render_source_result_exhausted() {
    let res = RenderSourceResult::Exhausted;
    assert_eq!(res, RenderSourceResult::Exhausted);
}

#[test]
fn render_source_result_clone() {
    let original = RenderSourceResult::Packet {
        frames_provided: 512,
        bytes_read: 2048,
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn render_source_result_debug() {
    let res = RenderSourceResult::Noop;
    let debug = format!("{:?}", res);
    assert_eq!(debug, "Noop");
}

// ===== RenderSourceError tests =====

#[test]
fn render_source_error_source_exhausted_display() {
    let err = RenderSourceError::SourceExhausted;
    assert_eq!(format!("{}", err), "source exhausted");
}

#[test]
fn render_source_error_format_mismatch_display() {
    let err = RenderSourceError::FormatMismatch {
        expected: "f32".to_string(),
        actual: "i16".to_string(),
    };
    assert_eq!(format!("{}", err), "format mismatch: expected f32, got i16");
}

#[test]
fn render_source_error_source_closed_display() {
    let err = RenderSourceError::SourceClosed;
    assert_eq!(format!("{}", err), "source closed");
}

#[test]
fn render_source_error_internal_display() {
    let err = RenderSourceError::Internal {
        description: "buffer overflow".to_string(),
    };
    assert_eq!(format!("{}", err), "internal source error: buffer overflow");
}

#[test]
fn render_source_error_clone() {
    let original = RenderSourceError::Internal {
        description: "test".to_string(),
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn render_source_error_debug() {
    let err = RenderSourceError::SourceExhausted;
    let debug = format!("{:?}", err);
    assert_eq!(debug, "SourceExhausted");
}

// ===== Cross-type integration tests =====

#[test]
fn render_source_types_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let req_a = RenderSourceRequest::Noop;
    let req_b = RenderSourceRequest::Noop;
    let mut h1 = DefaultHasher::new();
    let mut h2 = DefaultHasher::new();
    req_a.hash(&mut h1);
    req_b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}
