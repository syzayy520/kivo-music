//! Adapter integration for ring buffer render source.
//!
//! Helper functions for testing the source-to-sink dispatch chain
//! using FakeRingBufferSource. No threads, no IO, no WASAPI.

use super::fake_ring_buffer_source::FakeRingBufferSource;
use crate::playback::output_wasapi::output_thread::runtime::render_source_adapter::{
    dispatch_source_to_sink, invoke_render_source, AdapterContext, AdapterError, AdapterOutcome,
    SourceToSinkOutcome,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::consumer::consumer_contract::SinkConsumer;
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSource, RenderSourceRequest,
};

/// Creates a FakeRingBufferSource with test packets for integration testing.
pub fn create_test_source(packet_count: usize, frames_per_packet: u64) -> FakeRingBufferSource {
    FakeRingBufferSource::with_test_packets(packet_count, frames_per_packet)
}

/// Creates an empty FakeRingBufferSource for exhaustion testing.
pub fn create_empty_source() -> FakeRingBufferSource {
    FakeRingBufferSource::empty()
}

/// Creates a FakeRingBufferSource with EOS packet for testing.
pub fn create_source_with_eos(packet_count: usize, frames_per_packet: u64) -> FakeRingBufferSource {
    let mut source = FakeRingBufferSource::new(packet_count + 10);
    for _ in 0..packet_count {
        source.push_packet(frames_per_packet);
    }
    source.push_eos();
    source
}

/// Invokes a FakeRingBufferSource with a ReadPacket request and returns the outcome.
pub fn invoke_test_read(
    source: &mut FakeRingBufferSource,
    frame_count: u64,
    sample_rate: u32,
    channel_count: u16,
) -> Result<AdapterOutcome, AdapterError> {
    let request = RenderSourceRequest::ReadPacket {
        frame_count,
        sample_rate,
        channel_count,
    };
    let context = AdapterContext {
        frame_count,
        sample_rate,
        channel_count,
        ..Default::default()
    };
    invoke_render_source(source, &request, &context)
}

/// Invokes a FakeRingBufferSource with a Peek request and returns the outcome.
pub fn invoke_test_peek(source: &mut FakeRingBufferSource) -> Result<AdapterOutcome, AdapterError> {
    let request = RenderSourceRequest::Peek;
    let context = AdapterContext::default();
    invoke_render_source(source, &request, &context)
}

/// Invokes a FakeRingBufferSource with a Flush request and returns the outcome.
pub fn invoke_test_flush(
    source: &mut FakeRingBufferSource,
) -> Result<AdapterOutcome, AdapterError> {
    let request = RenderSourceRequest::Flush;
    let context = AdapterContext::default();
    invoke_render_source(source, &request, &context)
}

/// Executes a full source-to-sink dispatch chain with FakeRingBufferSource.
pub fn execute_test_dispatch<S: RenderSource, C: SinkConsumer>(
    source: &mut S,
    consumer: &mut C,
    frame_count: u64,
    sample_rate: u32,
    channel_count: u16,
) -> Result<SourceToSinkOutcome, AdapterError> {
    let request = RenderSourceRequest::ReadPacket {
        frame_count,
        sample_rate,
        channel_count,
    };
    dispatch_source_to_sink(source, consumer, &request, sample_rate, channel_count)
}

/// Reads all packets from a source until exhausted, returning (total_frames, total_bytes).
pub fn read_all_packets(
    source: &mut FakeRingBufferSource,
    frame_count: u64,
    sample_rate: u32,
    channel_count: u16,
) -> (u64, u64) {
    let mut total_frames = 0;
    let mut total_bytes = 0;

    loop {
        match invoke_test_read(source, frame_count, sample_rate, channel_count) {
            Ok(AdapterOutcome::Packet {
                frames_provided,
                bytes_read,
            }) => {
                total_frames += frames_provided;
                total_bytes += bytes_read;
            }
            Ok(AdapterOutcome::Exhausted) => break,
            Ok(_) => break,
            Err(_) => break,
        }
    }

    (total_frames, total_bytes)
}
