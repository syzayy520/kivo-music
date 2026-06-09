//! Device buffer writer trait tests.
//!
//! Tests for DeviceBufferWriter trait using a mock implementation.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WriteError, WriteRequest, WriteResult, WriterCursor, WriterState,
};

struct MockWriter {
    ready: bool,
    closed: bool,
    requests: Vec<WriteRequest>,
    writes_completed: u64,
    frames_written: u64,
    bytes_written: u64,
    buffer_capacity: u64,
    buffered_frames: u64,
}

impl MockWriter {
    fn new() -> Self {
        Self {
            ready: true,
            closed: false,
            requests: Vec::new(),
            writes_completed: 0,
            frames_written: 0,
            bytes_written: 0,
            buffer_capacity: 10000,
            buffered_frames: 0,
        }
    }
}

impl DeviceBufferWriter for MockWriter {
    fn process_request(&mut self, request: &WriteRequest) -> Result<WriteResult, WriteError> {
        if self.closed {
            return Err(WriteError::DeviceClosed);
        }
        if !self.ready {
            return Err(WriteError::WouldBlock);
        }
        self.requests.push(request.clone());
        match request {
            WriteRequest::WritePacket {
                frame_count,
                sample_rate,
                channel_count,
            } => {
                let bytes = frame_count * (*sample_rate as u64) * (*channel_count as u64) * 4;
                self.writes_completed += 1;
                self.frames_written += frame_count;
                self.bytes_written += bytes;
                self.buffered_frames += frame_count;
                Ok(WriteResult::Written {
                    frames_written: *frame_count,
                    bytes_written: bytes,
                })
            }
            WriteRequest::Flush => {
                self.buffered_frames = 0;
                Ok(WriteResult::Noop)
            }
            WriteRequest::Close => {
                self.closed = true;
                Ok(WriteResult::Noop)
            }
            WriteRequest::Noop => Ok(WriteResult::Noop),
        }
    }

    fn snapshot(&self) -> WriterState {
        WriterState {
            requests_accepted: self.requests.len() as u64,
            writes_completed: self.writes_completed,
            frames_written: self.frames_written,
            bytes_written: self.bytes_written,
            errors: 0,
            is_closed: self.closed,
            is_ready: self.ready,
            buffer_fill_frames: self.buffered_frames,
            buffer_capacity_frames: self.buffer_capacity,
            buffer_wrap_count: 0,
            would_block_count: 0,
            flush_count: 0,
            consecutive_would_blocks: 0,
            max_consecutive_would_blocks: 0,
        }
    }

    fn cursor(&self) -> WriterCursor {
        WriterCursor {
            write_position: self.frames_written,
            buffer_capacity: self.buffer_capacity,
            buffered_frames: self.buffered_frames,
            sample_rate: 44100,
            channel_count: 2,
            total_frames_written: self.frames_written,
            wrap_count: 0,
        }
    }

    fn is_ready(&self) -> bool {
        self.ready
    }

    fn is_closed(&self) -> bool {
        self.closed
    }

    fn reset(&mut self) {
        self.requests.clear();
        self.writes_completed = 0;
        self.frames_written = 0;
        self.bytes_written = 0;
        self.buffered_frames = 0;
        self.closed = false;
        self.ready = true;
    }
}

#[test]
fn mock_writer_process_write_packet() {
    let mut writer = MockWriter::new();
    let result = writer
        .process_request(&WriteRequest::WritePacket {
            frame_count: 512,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap();
    match result {
        WriteResult::Written {
            frames_written,
            bytes_written,
        } => {
            assert_eq!(frames_written, 512);
            assert!(bytes_written > 0);
        }
        _ => panic!("expected Written"),
    }
}

#[test]
fn mock_writer_flush_and_noop() {
    let mut writer = MockWriter::new();
    assert_eq!(
        writer.process_request(&WriteRequest::Flush).unwrap(),
        WriteResult::Noop
    );
    assert_eq!(
        writer.process_request(&WriteRequest::Noop).unwrap(),
        WriteResult::Noop
    );
}

#[test]
fn mock_writer_closed_returns_error() {
    let mut writer = MockWriter::new();
    writer.closed = true;
    let err = writer
        .process_request(&WriteRequest::WritePacket {
            frame_count: 512,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap_err();
    assert_eq!(err, WriteError::DeviceClosed);
}

#[test]
fn mock_writer_snapshot_tracks_state() {
    let mut writer = MockWriter::new();
    writer.process_request(&WriteRequest::Noop).unwrap();
    writer.process_request(&WriteRequest::Flush).unwrap();
    writer
        .process_request(&WriteRequest::WritePacket {
            frame_count: 512,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap();
    let snap = writer.snapshot();
    assert_eq!(snap.requests_accepted, 3);
    assert_eq!(snap.frames_written, 512);
    assert_eq!(snap.writes_completed, 1);
}

#[test]
fn mock_writer_cursor_advances() {
    let mut writer = MockWriter::new();
    writer
        .process_request(&WriteRequest::WritePacket {
            frame_count: 256,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap();
    assert_eq!(writer.cursor().write_position, 256);
    assert_eq!(writer.cursor().total_frames_written, 256);
}

#[test]
fn mock_writer_ready_and_not_closed() {
    let writer = MockWriter::new();
    assert!(writer.is_ready());
    assert!(!writer.is_closed());
}

#[test]
fn mock_writer_close_and_reset() {
    let mut writer = MockWriter::new();
    writer.process_request(&WriteRequest::Close).unwrap();
    assert!(writer.is_closed());
    writer.reset();
    assert!(!writer.is_closed());
    assert_eq!(writer.snapshot().requests_accepted, 0);
}
