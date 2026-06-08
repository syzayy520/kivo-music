//! Sink boundary contract tests.
//!
//! Tests for SinkRequest and SinkResult types.

use crate::playback::output_wasapi::output_thread::sink_boundary::{
    SinkRequest, SinkResult,
};

// ===== SinkRequest tests =====

#[test]
fn sink_request_default_is_noop() {
    let req = SinkRequest::default();
    assert_eq!(req, SinkRequest::Noop);
}

#[test]
fn sink_request_render_equality() {
    let a = SinkRequest::Render {
        frame_count: 1024,
        sample_rate: 44100,
        channel_count: 2,
    };
    let b = SinkRequest::Render {
        frame_count: 1024,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_eq!(a, b);
}

#[test]
fn sink_request_render_inequality() {
    let a = SinkRequest::Render {
        frame_count: 1024,
        sample_rate: 44100,
        channel_count: 2,
    };
    let b = SinkRequest::Render {
        frame_count: 512,
        sample_rate: 44100,
        channel_count: 2,
    };
    assert_ne!(a, b);
}

#[test]
fn sink_request_silence() {
    let req = SinkRequest::WriteSilence { frame_count: 256 };
    match req {
        SinkRequest::WriteSilence { frame_count } => assert_eq!(frame_count, 256),
        _ => panic!("expected WriteSilence"),
    }
}

#[test]
fn sink_request_flush() {
    let req = SinkRequest::Flush;
    assert_eq!(req, SinkRequest::Flush);
}

#[test]
fn sink_request_clone() {
    let original = SinkRequest::Render {
        frame_count: 1024,
        sample_rate: 48000,
        channel_count: 2,
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn sink_request_debug() {
    let req = SinkRequest::Noop;
    let debug = format!("{:?}", req);
    assert_eq!(debug, "Noop");
}

// ===== SinkResult tests =====

#[test]
fn sink_result_default_is_noop() {
    let res = SinkResult::default();
    assert_eq!(res, SinkResult::Noop);
}

#[test]
fn sink_result_success() {
    let res = SinkResult::Success {
        frames_processed: 1024,
        bytes_written: 4096,
    };
    match res {
        SinkResult::Success {
            frames_processed,
            bytes_written,
        } => {
            assert_eq!(frames_processed, 1024);
            assert_eq!(bytes_written, 4096);
        }
        _ => panic!("expected Success"),
    }
}

#[test]
fn sink_result_silence_filled() {
    let res = SinkResult::SilenceFilled { frames_written: 256 };
    match res {
        SinkResult::SilenceFilled { frames_written } => assert_eq!(frames_written, 256),
        _ => panic!("expected SilenceFilled"),
    }
}

#[test]
fn sink_result_skipped() {
    let res = SinkResult::Skipped;
    assert_eq!(res, SinkResult::Skipped);
}

#[test]
fn sink_result_clone() {
    let original = SinkResult::Success {
        frames_processed: 512,
        bytes_written: 2048,
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn sink_result_debug() {
    let res = SinkResult::Noop;
    let debug = format!("{:?}", res);
    assert_eq!(debug, "Noop");
}
