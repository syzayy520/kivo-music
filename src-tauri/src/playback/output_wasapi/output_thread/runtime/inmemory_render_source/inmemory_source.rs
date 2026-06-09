//! In-memory render source implementation.
//!
//! Pure-memory implementation of the RenderSource trait for testing.
//! Uses SourceQueue to manage packet metadata. No actual audio data.

use super::source_queue::SourceQueue;
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSource, RenderSourceError, RenderSourceRequest, RenderSourceResult, SourceCursor,
    SourceSnapshot,
};

/// In-memory render source for testing.
///
/// Implements RenderSource trait using a queue of packet metadata.
/// No actual audio data, no WASAPI, no IO.
#[derive(Debug)]
pub struct InMemoryRenderSource {
    /// Packet queue.
    queue: SourceQueue,
    /// Whether the source is ready to serve requests.
    ready: bool,
    /// Whether the source is fully exhausted.
    exhausted: bool,
    /// Total requests accepted.
    requests_accepted: u64,
    /// Total packets provided.
    packets_provided: u64,
    /// Total frames read.
    frames_read: u64,
    /// Total bytes read.
    bytes_read: u64,
    /// Total errors encountered.
    errors: u64,
    /// Current read position in frames.
    position_frames: u64,
    /// Sample rate in Hz.
    sample_rate: u32,
    /// Channel count.
    channel_count: u16,
}

impl InMemoryRenderSource {
    /// Creates a new in-memory render source with the given queue.
    pub fn new(queue: SourceQueue) -> Self {
        Self {
            queue,
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
        }
    }

    /// Creates a new empty in-memory render source.
    pub fn empty() -> Self {
        Self::new(SourceQueue::new())
    }

    /// Creates a new in-memory render source with test packets.
    pub fn with_test_packets(count: u64, frames_per_packet: u64) -> Self {
        let mut queue = SourceQueue::new();
        for _ in 0..count {
            queue.push_new(frames_per_packet);
        }
        Self::new(queue)
    }

    /// Returns the current queue length.
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    /// Returns true if the queue has an end-of-stream packet.
    pub fn has_eos(&self) -> bool {
        self.queue.has_eos()
    }

    /// Sets the sample rate for cursor reporting.
    pub fn with_sample_rate(mut self, sample_rate: u32) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    /// Sets the channel count for cursor reporting.
    pub fn with_channel_count(mut self, channel_count: u16) -> Self {
        self.channel_count = channel_count;
        self
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

        match self.queue.pop() {
            Some(packet) => {
                if packet.is_eos {
                    self.exhausted = true;
                    self.ready = false;
                    Ok(RenderSourceResult::Exhausted)
                } else {
                    let frames_provided = packet.frame_count.min(frame_count);
                    let bytes_read = frames_provided * self.channel_count as u64 * 4; // f32 format

                    self.packets_provided += 1;
                    self.frames_read += frames_provided;
                    self.bytes_read += bytes_read;
                    self.position_frames += frames_provided;

                    Ok(RenderSourceResult::Packet {
                        frames_provided,
                        bytes_read,
                    })
                }
            }
            None => {
                self.exhausted = true;
                self.ready = false;
                Ok(RenderSourceResult::Exhausted)
            }
        }
    }
}

impl RenderSource for InMemoryRenderSource {
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
                } else if self.queue.is_empty() {
                    Ok(RenderSourceResult::Skipped)
                } else {
                    Ok(RenderSourceResult::Noop)
                }
            }
            RenderSourceRequest::Flush => {
                self.queue.clear();
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
        SourceCursor {
            position_frames: self.position_frames,
            total_frames: self.queue.total_frames(),
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
        // Note: queue is not cleared on reset
    }
}