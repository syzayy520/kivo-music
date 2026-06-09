//! Render source trait tests.
//!
//! Tests for RenderSource trait using a mock implementation.

use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSource, RenderSourceError, RenderSourceRequest, RenderSourceResult, SourceCursor,
    SourceSnapshot,
};
struct MockSource {
    ready: bool,
    exhausted: bool,
    requests: Vec<RenderSourceRequest>,
    packets_provided: u64,
    frames_read: u64,
    bytes_read: u64,
    position: u64,
    total_frames: u64,
}
impl MockSource {
    fn new() -> Self {
        Self {
            ready: true,
            exhausted: false,
            requests: Vec::new(),
            packets_provided: 0,
            frames_read: 0,
            bytes_read: 0,
            position: 0,
            total_frames: 10000,
        }
    }
}
impl RenderSource for MockSource {
    fn process_request(
        &mut self,
        request: &RenderSourceRequest,
    ) -> Result<RenderSourceResult, RenderSourceError> {
        if self.exhausted {
            return Err(RenderSourceError::SourceExhausted);
        }
        self.requests.push(request.clone());
        match request {
            RenderSourceRequest::ReadPacket {
                frame_count,
                sample_rate,
                channel_count,
            } => {
                let bytes = frame_count * (*sample_rate as u64) * (*channel_count as u64) * 2;
                self.packets_provided += 1;
                self.frames_read += frame_count;
                self.bytes_read += bytes;
                self.position += frame_count;
                if self.position >= self.total_frames {
                    self.exhausted = true;
                }
                Ok(RenderSourceResult::Packet {
                    frames_provided: *frame_count,
                    bytes_read: bytes,
                })
            }
            _ => Ok(RenderSourceResult::Noop),
        }
    }
    fn snapshot(&self) -> SourceSnapshot {
        SourceSnapshot {
            requests_accepted: self.requests.len() as u64,
            packets_provided: self.packets_provided,
            frames_read: self.frames_read,
            bytes_read: self.bytes_read,
            errors: 0,
            is_exhausted: self.exhausted,
            is_ready: self.ready,
        }
    }
    fn cursor(&self) -> SourceCursor {
        SourceCursor {
            position_frames: self.position,
            total_frames: self.total_frames,
            sample_rate: 44100,
            channel_count: 2,
        }
    }
    fn is_ready(&self) -> bool {
        self.ready
    }
    fn is_exhausted(&self) -> bool {
        self.exhausted
    }
    fn reset(&mut self) {
        self.requests.clear();
        self.packets_provided = 0;
        self.frames_read = 0;
        self.bytes_read = 0;
        self.position = 0;
        self.exhausted = false;
        self.ready = true;
    }
}

#[test]
fn mock_source_process_read_packet() {
    let mut source = MockSource::new();
    let result = source
        .process_request(&RenderSourceRequest::ReadPacket {
            frame_count: 1024,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap();
    match result {
        RenderSourceResult::Packet {
            frames_provided,
            bytes_read,
        } => {
            assert_eq!(frames_provided, 1024);
            assert!(bytes_read > 0);
        }
        _ => panic!("expected Packet"),
    }
}

#[test]
fn mock_source_process_peek_flush_noop() {
    let mut source = MockSource::new();
    assert_eq!(
        source.process_request(&RenderSourceRequest::Peek).unwrap(),
        RenderSourceResult::Noop
    );
    assert_eq!(
        source.process_request(&RenderSourceRequest::Flush).unwrap(),
        RenderSourceResult::Noop
    );
    assert_eq!(
        source.process_request(&RenderSourceRequest::Noop).unwrap(),
        RenderSourceResult::Noop
    );
}

#[test]
fn mock_source_exhausted_returns_error() {
    let mut source = MockSource::new();
    source.exhausted = true;
    let err = source
        .process_request(&RenderSourceRequest::ReadPacket {
            frame_count: 1024,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap_err();
    assert_eq!(err, RenderSourceError::SourceExhausted);
}

#[test]
fn mock_source_snapshot_tracks_requests_and_frames() {
    let mut source = MockSource::new();
    source.process_request(&RenderSourceRequest::Noop).unwrap();
    source.process_request(&RenderSourceRequest::Flush).unwrap();
    source
        .process_request(&RenderSourceRequest::ReadPacket {
            frame_count: 512,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap();
    let snap = source.snapshot();
    assert_eq!(snap.requests_accepted, 3);
    assert_eq!(snap.frames_read, 512);
    assert_eq!(snap.packets_provided, 1);
}

#[test]
fn mock_source_cursor_advances() {
    let mut source = MockSource::new();
    source
        .process_request(&RenderSourceRequest::ReadPacket {
            frame_count: 256,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap();
    assert_eq!(source.cursor().position_frames, 256);
}

#[test]
fn mock_source_ready_and_not_exhausted_initially() {
    let source = MockSource::new();
    assert!(source.is_ready());
    assert!(!source.is_exhausted());
}

#[test]
fn mock_source_becomes_exhausted() {
    let mut source = MockSource::new();
    source.total_frames = 100;
    source
        .process_request(&RenderSourceRequest::ReadPacket {
            frame_count: 100,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap();
    assert!(source.is_exhausted());
}

#[test]
fn mock_source_reset() {
    let mut source = MockSource::new();
    source
        .process_request(&RenderSourceRequest::ReadPacket {
            frame_count: 512,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap();
    source.reset();
    let snap = source.snapshot();
    assert_eq!(snap.requests_accepted, 0);
    assert_eq!(snap.frames_read, 0);
    assert!(!source.is_exhausted());
}

#[test]
fn mock_source_display_error() {
    assert_eq!(
        format!("{}", RenderSourceError::SourceExhausted),
        "source exhausted"
    );
}
