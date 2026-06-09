//! Fake device buffer writer implementation.
//!
//! Pure-memory DeviceBufferWriter implementation for testing device buffer boundary.
//! Uses a frame counter to simulate buffer. No WASAPI, no IO.

use super::{
    frame_bytes, DeviceBufferWriter, WriteError, WriteRequest, WriteResult, WriterCursor,
    WriterState,
};

/// Fake device buffer writer for testing. Simulates buffer behavior with capacity limits.
#[derive(Debug)]
pub struct FakeDeviceBufferWriter {
    capacity: u64,
    buffered_frames: u64,
    ready: bool,
    closed: bool,
    requests_accepted: u64,
    writes_completed: u64,
    frames_written: u64,
    bytes_written: u64,
    errors: u64,
    flush_count: u64,
    would_block_count: u64,
    wrap_count: u64,
    sample_rate: u32,
    channel_count: u16,
}

impl FakeDeviceBufferWriter {
    /// Creates a new writer with the given capacity.
    pub fn new(capacity: u64) -> Self {
        Self {
            capacity,
            buffered_frames: 0,
            ready: true,
            closed: false,
            requests_accepted: 0,
            writes_completed: 0,
            frames_written: 0,
            bytes_written: 0,
            errors: 0,
            flush_count: 0,
            would_block_count: 0,
            wrap_count: 0,
            sample_rate: 44100,
            channel_count: 2,
        }
    }

    /// Creates an empty writer with default capacity.
    pub fn empty() -> Self {
        Self::new(10000)
    }

    /// Creates a writer with specified capacity and format.
    pub fn with_format(capacity: u64, sample_rate: u32, channel_count: u16) -> Self {
        Self {
            capacity,
            sample_rate,
            channel_count,
            ..Self::new(capacity)
        }
    }

    /// Returns the buffer capacity.
    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    /// Returns the current buffered frames.
    pub fn buffered_frames(&self) -> u64 {
        self.buffered_frames
    }

    /// Processes a WritePacket request.
    fn process_write_packet(
        &mut self,
        frame_count: u64,
        _sample_rate: u32,
        channel_count: u16,
    ) -> Result<WriteResult, WriteError> {
        if self.closed {
            self.errors += 1;
            return Err(WriteError::DeviceClosed);
        }
        if !self.ready {
            self.errors += 1;
            return Err(WriteError::WouldBlock);
        }
        if self.buffered_frames + frame_count > self.capacity {
            self.would_block_count += 1;
            return Err(WriteError::WouldBlock);
        }
        let bytes =
            frame_bytes::f32_packet_byte_count(frame_count, channel_count).ok_or_else(|| {
                WriteError::Internal {
                    description: "device buffer packet byte count overflow".to_string(),
                }
            })?;
        self.writes_completed += 1;
        self.frames_written += frame_count;
        self.bytes_written += bytes;
        self.buffered_frames += frame_count;
        Ok(WriteResult::Written {
            frames_written: frame_count,
            bytes_written: bytes,
        })
    }
}

impl DeviceBufferWriter for FakeDeviceBufferWriter {
    fn process_request(&mut self, request: &WriteRequest) -> Result<WriteResult, WriteError> {
        self.requests_accepted += 1;
        match request {
            WriteRequest::WritePacket {
                frame_count,
                sample_rate,
                channel_count,
            } => self.process_write_packet(*frame_count, *sample_rate, *channel_count),
            WriteRequest::Flush => {
                self.buffered_frames = 0;
                self.flush_count += 1;
                Ok(WriteResult::Noop)
            }
            WriteRequest::Close => {
                self.closed = true;
                self.ready = false;
                Ok(WriteResult::Noop)
            }
            WriteRequest::Noop => Ok(WriteResult::Noop),
        }
    }

    fn snapshot(&self) -> WriterState {
        WriterState {
            requests_accepted: self.requests_accepted,
            writes_completed: self.writes_completed,
            frames_written: self.frames_written,
            bytes_written: self.bytes_written,
            errors: self.errors,
            is_closed: self.closed,
            is_ready: self.ready,
            buffer_fill_frames: self.buffered_frames,
            buffer_capacity_frames: self.capacity,
            buffer_wrap_count: self.wrap_count,
            would_block_count: self.would_block_count,
            flush_count: self.flush_count,
            consecutive_would_blocks: 0,
            max_consecutive_would_blocks: 0,
        }
    }

    fn cursor(&self) -> WriterCursor {
        WriterCursor {
            write_position: self.frames_written,
            buffer_capacity: self.capacity,
            buffered_frames: self.buffered_frames,
            sample_rate: self.sample_rate,
            channel_count: self.channel_count,
            total_frames_written: self.frames_written,
            wrap_count: self.wrap_count,
        }
    }

    fn is_ready(&self) -> bool {
        self.ready && !self.closed
    }

    fn is_closed(&self) -> bool {
        self.closed
    }

    fn reset(&mut self) {
        self.ready = true;
        self.closed = false;
        self.requests_accepted = 0;
        self.writes_completed = 0;
        self.frames_written = 0;
        self.bytes_written = 0;
        self.errors = 0;
        self.buffered_frames = 0;
        self.flush_count = 0;
        self.would_block_count = 0;
        self.wrap_count = 0;
    }
}
