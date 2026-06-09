//! Fake ring buffer render source implementation.
//!
//! Pure-memory RenderSource implementation for testing ring buffer boundary.
//! Uses BufferPacket vector to simulate ring buffer. No WASAPI, no IO.

use super::buffer_packet::BufferPacket;
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSource, RenderSourceError, RenderSourceRequest, RenderSourceResult, SourceCursor,
    SourceSnapshot,
};

/// Fake ring buffer render source for testing. Simulates ring buffer behavior with capacity limits.
#[derive(Debug)]
pub struct FakeRingBufferSource {
    buffer: Vec<BufferPacket>,
    read_index: usize,
    capacity: usize,
    ready: bool,
    exhausted: bool,
    requests_accepted: u64,
    packets_provided: u64,
    frames_read: u64,
    bytes_read: u64,
    errors: u64,
    position_frames: u64,
    sample_rate: u32,
    channel_count: u16,
    sequence_counter: u64,
}

impl FakeRingBufferSource {
    /// Creates a new source with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            read_index: 0,
            capacity,
            ready: true,
            exhausted: false,
            requests_accepted: 0,
            packets_provided: 0,
            frames_read: 0,
            bytes_read: 0,
            errors: 0,
            position_frames: 0,
            sample_rate: 44100,
            channel_count: 2,
            sequence_counter: 0,
        }
    }

    /// Creates an empty source.
    pub fn empty() -> Self {
        Self::new(100)
    }

    /// Creates a source with test packets.
    pub fn with_test_packets(count: usize, frames_per_packet: u64) -> Self {
        let mut source = Self::new(count + 10);
        for _ in 0..count {
            source.push_packet(frames_per_packet);
        }
        source
    }

    /// Pushes a packet into the buffer.
    pub fn push_packet(&mut self, frame_count: u64) {
        if self.buffer.len() < self.capacity {
            let seq = self.sequence_counter;
            self.sequence_counter += 1;
            self.buffer.push(BufferPacket::with_position(
                seq,
                frame_count,
                self.position_frames + self.frames_read,
                seq,
            ));
        }
    }

    /// Pushes an end-of-stream packet.
    pub fn push_eos(&mut self) {
        if self.buffer.len() < self.capacity {
            let seq = self.sequence_counter;
            self.sequence_counter += 1;
            self.buffer.push(BufferPacket::eos(seq));
        }
    }

    /// Returns the buffer length.
    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns true if buffer has EOS.
    pub fn has_eos(&self) -> bool {
        self.buffer.iter().any(|p| p.is_eos)
    }

    /// Returns the capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Processes a ReadPacket request.
    fn process_read_packet(
        &mut self,
        frame_count: u64,
        _sample_rate: u32,
        _channel_count: u16,
    ) -> Result<RenderSourceResult, RenderSourceError> {
        if self.exhausted {
            self.errors += 1;
            return Err(RenderSourceError::SourceExhausted);
        }
        if !self.ready {
            self.errors += 1;
            return Err(RenderSourceError::Internal {
                description: "source not ready".to_string(),
            });
        }
        if self.read_index >= self.buffer.len() {
            self.exhausted = true;
            self.ready = false;
            return Ok(RenderSourceResult::Exhausted);
        }
        let packet = &self.buffer[self.read_index];
        if packet.is_eos {
            self.exhausted = true;
            self.ready = false;
            self.read_index += 1;
            return Ok(RenderSourceResult::Exhausted);
        }
        let frames_provided = packet.frame_count.min(frame_count);
        let bytes_read = frames_provided * self.channel_count as u64 * 4;
        self.packets_provided += 1;
        self.frames_read += frames_provided;
        self.bytes_read += bytes_read;
        self.position_frames += frames_provided;
        self.read_index += 1;
        Ok(RenderSourceResult::Packet {
            frames_provided,
            bytes_read,
        })
    }
}

impl RenderSource for FakeRingBufferSource {
    fn process_request(
        &mut self,
        request: &RenderSourceRequest,
    ) -> Result<RenderSourceResult, RenderSourceError> {
        self.requests_accepted += 1;
        match request {
            RenderSourceRequest::ReadPacket {
                frame_count,
                sample_rate,
                channel_count,
            } => self.process_read_packet(*frame_count, *sample_rate, *channel_count),
            RenderSourceRequest::Peek => {
                if self.exhausted {
                    Ok(RenderSourceResult::Exhausted)
                } else if self.read_index >= self.buffer.len() {
                    Ok(RenderSourceResult::Skipped)
                } else {
                    Ok(RenderSourceResult::Noop)
                }
            }
            RenderSourceRequest::Flush => {
                self.buffer.clear();
                self.read_index = 0;
                Ok(RenderSourceResult::Noop)
            }
            RenderSourceRequest::Noop => Ok(RenderSourceResult::Noop),
        }
    }

    fn snapshot(&self) -> SourceSnapshot {
        SourceSnapshot {
            requests_accepted: self.requests_accepted,
            packets_provided: self.packets_provided,
            frames_read: self.frames_read,
            bytes_read: self.bytes_read,
            errors: self.errors,
            is_exhausted: self.exhausted,
            is_ready: self.ready,
        }
    }

    fn cursor(&self) -> SourceCursor {
        let total_frames: u64 = self.buffer[self.read_index..].iter().map(|p| p.frame_count).sum();
        SourceCursor {
            position_frames: self.position_frames,
            total_frames,
            sample_rate: self.sample_rate,
            channel_count: self.channel_count,
        }
    }

    fn is_ready(&self) -> bool {
        self.ready && !self.exhausted
    }

    fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    fn reset(&mut self) {
        self.ready = true;
        self.exhausted = false;
        self.requests_accepted = 0;
        self.packets_provided = 0;
        self.frames_read = 0;
        self.bytes_read = 0;
        self.errors = 0;
        self.position_frames = 0;
        self.read_index = 0;
    }
}
