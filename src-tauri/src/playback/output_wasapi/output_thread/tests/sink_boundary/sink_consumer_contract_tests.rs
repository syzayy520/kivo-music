//! Sink consumer contract tests.
//!
//! Tests for SinkConsumer trait and ConsumerSnapshot type.

use crate::playback::output_wasapi::output_thread::sink_boundary::{
    ConsumerSnapshot, SinkConsumer, SinkError, SinkRequest, SinkResult,
};

// ===== ConsumerSnapshot tests =====

#[test]
fn consumer_snapshot_default() {
    let snap = ConsumerSnapshot::default();
    assert_eq!(snap.requests_processed, 0);
    assert_eq!(snap.frames_rendered, 0);
    assert_eq!(snap.bytes_written, 0);
    assert_eq!(snap.errors, 0);
    assert!(!snap.is_ready);
}

#[test]
fn consumer_snapshot_custom() {
    let snap = ConsumerSnapshot {
        requests_processed: 100,
        frames_rendered: 1024,
        bytes_written: 4096,
        errors: 2,
        is_ready: true,
    };
    assert_eq!(snap.requests_processed, 100);
    assert_eq!(snap.frames_rendered, 1024);
    assert_eq!(snap.bytes_written, 4096);
    assert_eq!(snap.errors, 2);
    assert!(snap.is_ready);
}

#[test]
fn consumer_snapshot_clone() {
    let original = ConsumerSnapshot {
        requests_processed: 50,
        frames_rendered: 512,
        bytes_written: 2048,
        errors: 1,
        is_ready: true,
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn consumer_snapshot_debug() {
    let snap = ConsumerSnapshot::default();
    let debug = format!("{:?}", snap);
    assert!(debug.contains("requests_processed"));
}

#[test]
fn consumer_snapshot_equality() {
    let a = ConsumerSnapshot {
        requests_processed: 10,
        frames_rendered: 100,
        bytes_written: 400,
        errors: 0,
        is_ready: true,
    };
    let b = ConsumerSnapshot {
        requests_processed: 10,
        frames_rendered: 100,
        bytes_written: 400,
        errors: 0,
        is_ready: true,
    };
    assert_eq!(a, b);
}

#[test]
fn consumer_snapshot_inequality() {
    let a = ConsumerSnapshot::default();
    let b = ConsumerSnapshot {
        is_ready: true,
        ..Default::default()
    };
    assert_ne!(a, b);
}

// ===== SinkConsumer trait tests =====

/// A mock consumer for testing the trait.
struct MockConsumer {
    ready: bool,
    requests: Vec<SinkRequest>,
}

impl MockConsumer {
    fn new() -> Self {
        Self {
            ready: true,
            requests: Vec::new(),
        }
    }
}

impl SinkConsumer for MockConsumer {
    fn process_request(&mut self, request: &SinkRequest) -> Result<SinkResult, SinkError> {
        self.requests.push(request.clone());
        match request {
            SinkRequest::Render { frame_count, .. } => Ok(SinkResult::Success {
                frames_processed: *frame_count,
                bytes_written: frame_count * 4,
            }),
            SinkRequest::WriteSilence { frame_count } => Ok(SinkResult::SilenceFilled {
                frames_written: *frame_count,
            }),
            SinkRequest::Flush => Ok(SinkResult::Noop),
            SinkRequest::Noop => Ok(SinkResult::Noop),
        }
    }

    fn snapshot(&self) -> ConsumerSnapshot {
        ConsumerSnapshot {
            requests_processed: self.requests.len() as u64,
            frames_rendered: 0,
            bytes_written: 0,
            errors: 0,
            is_ready: self.ready,
        }
    }

    fn is_ready(&self) -> bool {
        self.ready
    }

    fn reset(&mut self) {
        self.requests.clear();
        self.ready = true;
    }
}

#[test]
fn mock_consumer_process_render() {
    let mut consumer = MockConsumer::new();
    let request = SinkRequest::Render {
        frame_count: 1024,
        sample_rate: 44100,
        channel_count: 2,
    };
    let result = consumer.process_request(&request).unwrap();
    assert_eq!(
        result,
        SinkResult::Success {
            frames_processed: 1024,
            bytes_written: 4096,
        }
    );
}

#[test]
fn mock_consumer_process_silence() {
    let mut consumer = MockConsumer::new();
    let request = SinkRequest::WriteSilence { frame_count: 256 };
    let result = consumer.process_request(&request).unwrap();
    assert_eq!(
        result,
        SinkResult::SilenceFilled {
            frames_written: 256
        }
    );
}

#[test]
fn mock_consumer_process_flush() {
    let mut consumer = MockConsumer::new();
    let request = SinkRequest::Flush;
    let result = consumer.process_request(&request).unwrap();
    assert_eq!(result, SinkResult::Noop);
}

#[test]
fn mock_consumer_process_noop() {
    let mut consumer = MockConsumer::new();
    let request = SinkRequest::Noop;
    let result = consumer.process_request(&request).unwrap();
    assert_eq!(result, SinkResult::Noop);
}

#[test]
fn mock_consumer_snapshot_tracks_requests() {
    let mut consumer = MockConsumer::new();
    consumer.process_request(&SinkRequest::Noop).unwrap();
    consumer.process_request(&SinkRequest::Flush).unwrap();
    let snap = consumer.snapshot();
    assert_eq!(snap.requests_processed, 2);
}

#[test]
fn mock_consumer_is_ready() {
    let consumer = MockConsumer::new();
    assert!(consumer.is_ready());
}

#[test]
fn mock_consumer_reset() {
    let mut consumer = MockConsumer::new();
    consumer.process_request(&SinkRequest::Noop).unwrap();
    consumer.reset();
    let snap = consumer.snapshot();
    assert_eq!(snap.requests_processed, 0);
}

#[test]
fn mock_consumer_snapshot_ready_state() {
    let mut consumer = MockConsumer::new();
    consumer.ready = false;
    let snap = consumer.snapshot();
    assert!(!snap.is_ready);
}
