//! In-memory source exhaustion and error behavior.
//!
//! Exhaustion detection, error path helpers, and edge-case behavior
//! for InMemoryRenderSource. No threads, no IO, no WASAPI.

use super::inmemory_source::InMemoryRenderSource;
use super::source_queue::SourceQueue;
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSource, RenderSourceError, RenderSourceRequest, RenderSourceResult,
};

/// Result of a single source step (read attempt).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceStepOutcome {
    /// A packet was provided.
    Packet {
        frames_provided: u64,
        bytes_read: u64,
    },
    /// Source is exhausted.
    Exhausted,
    /// Request was skipped.
    Skipped,
    /// No-op.
    Noop,
}

/// Run a source until exhaustion and collect statistics.
pub fn drain_until_exhausted(
    source: &mut InMemoryRenderSource,
    frame_count: u64,
    sample_rate: u32,
    channel_count: u16,
) -> DrainResult {
    let mut packets = 0u64;
    let mut frames = 0u64;
    let mut bytes = 0u64;

    loop {
        let request = RenderSourceRequest::ReadPacket {
            frame_count,
            sample_rate,
            channel_count,
        };
        match source.process_request(&request) {
            Ok(RenderSourceResult::Packet {
                frames_provided,
                bytes_read,
            }) => {
                packets += 1;
                frames += frames_provided;
                bytes += bytes_read;
            }
            Ok(RenderSourceResult::Exhausted) => break,
            Ok(_) => break,
            Err(_) => break,
        }
    }

    DrainResult {
        packets,
        frames,
        bytes,
        is_exhausted: source.is_exhausted(),
    }
}

/// Statistics from draining a source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrainResult {
    pub packets: u64,
    pub frames: u64,
    pub bytes: u64,
    pub is_exhausted: bool,
}

/// Test that repeated reads after exhaustion return Exhausted.
pub fn verify_repeated_exhaustion_returns_exhausted(
    source: &mut InMemoryRenderSource,
    frame_count: u64,
    sample_rate: u32,
    channel_count: u16,
    repeat_count: usize,
) -> bool {
    for _ in 0..repeat_count {
        let request = RenderSourceRequest::ReadPacket {
            frame_count,
            sample_rate,
            channel_count,
        };
        match source.process_request(&request) {
            Err(RenderSourceError::SourceExhausted) => continue,
            Ok(RenderSourceResult::Exhausted) => continue,
            _ => return false,
        }
    }
    true
}

/// Verify that reset allows reading again.
pub fn verify_reset_allows_reread(
    source: &mut InMemoryRenderSource,
    frame_count: u64,
    sample_rate: u32,
    channel_count: u16,
) -> bool {
    // Drain first.
    drain_until_exhausted(source, frame_count, sample_rate, channel_count);
    assert!(source.is_exhausted());

    // Reset.
    source.reset();
    assert!(source.is_ready());

    // Should be able to read again (queue still has packets).
    let request = RenderSourceRequest::ReadPacket {
        frame_count,
        sample_rate,
        channel_count,
    };
    match source.process_request(&request) {
        Ok(RenderSourceResult::Packet { .. }) => true,
        Ok(RenderSourceResult::Exhausted) => source.queue_len() == 0, // acceptable if queue was empty
        _ => false,
    }
}

/// Create a source with mixed packet sizes for edge case testing.
pub fn create_mixed_size_source(sizes: &[u64]) -> InMemoryRenderSource {
    let mut queue = SourceQueue::new();
    for &size in sizes {
        queue.push_new(size);
    }
    InMemoryRenderSource::new(queue)
}
