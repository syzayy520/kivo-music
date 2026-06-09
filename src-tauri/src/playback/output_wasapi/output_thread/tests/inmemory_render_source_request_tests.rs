//! In-memory render source request processing tests.
//!
//! Tests for RenderSource trait request processing: ReadPacket, Peek, Flush, Noop.

use crate::playback::output_wasapi::output_thread::runtime::inmemory_render_source::{
    InMemoryRenderSource, SourceQueue,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSource, RenderSourceRequest, RenderSourceResult,
};

#[test]
fn inmemory_source_process_read_packet_success() {
    let mut source = InMemoryRenderSource::with_test_packets(2, 1024);
    
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 1024,
        sample_rate: 44100,
        channel_count: 2,
    };
    
    let result = source.process_request(&request).unwrap();
    match result {
        RenderSourceResult::Packet {
            frames_provided,
            bytes_read,
        } => {
            assert_eq!(frames_provided, 1024);
            assert_eq!(bytes_read, 1024 * 2 * 4);
        }
        _ => panic!("Expected Packet result"),
    }
    
    let snapshot = source.snapshot();
    assert_eq!(snapshot.requests_accepted, 1);
    assert_eq!(snapshot.packets_provided, 1);
    assert_eq!(snapshot.frames_read, 1024);
    assert_eq!(snapshot.bytes_read, 1024 * 2 * 4);
    assert!(!snapshot.is_exhausted);
}

#[test]
fn inmemory_source_process_read_packet_exhausted() {
    let mut source = InMemoryRenderSource::empty();
    
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 1024,
        sample_rate: 44100,
        channel_count: 2,
    };
    
    let result = source.process_request(&request).unwrap();
    assert_eq!(result, RenderSourceResult::Exhausted);
    
    let snapshot = source.snapshot();
    assert!(snapshot.is_exhausted);
    assert!(!snapshot.is_ready);
}

#[test]
fn inmemory_source_process_peek_empty() {
    let mut source = InMemoryRenderSource::empty();
    
    let request = RenderSourceRequest::Peek;
    let result = source.process_request(&request).unwrap();
    assert_eq!(result, RenderSourceResult::Skipped);
}

#[test]
fn inmemory_source_process_peek_with_packets() {
    let mut source = InMemoryRenderSource::with_test_packets(1, 100);
    
    let request = RenderSourceRequest::Peek;
    let result = source.process_request(&request).unwrap();
    assert_eq!(result, RenderSourceResult::Noop);
}

#[test]
fn inmemory_source_process_flush() {
    let mut source = InMemoryRenderSource::with_test_packets(3, 100);
    assert_eq!(source.queue_len(), 3);
    
    let request = RenderSourceRequest::Flush;
    let result = source.process_request(&request).unwrap();
    assert_eq!(result, RenderSourceResult::Noop);
    assert_eq!(source.queue_len(), 0);
}

#[test]
fn inmemory_source_process_noop() {
    let mut source = InMemoryRenderSource::empty();
    
    let request = RenderSourceRequest::Noop;
    let result = source.process_request(&request).unwrap();
    assert_eq!(result, RenderSourceResult::Noop);
    
    let snapshot = source.snapshot();
    assert_eq!(snapshot.requests_accepted, 1);
}

#[test]
fn inmemory_source_read_until_exhausted() {
    let mut source = InMemoryRenderSource::with_test_packets(3, 100);
    
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 100,
        sample_rate: 44100,
        channel_count: 2,
    };
    
    for i in 0..3 {
        let result = source.process_request(&request).unwrap();
        match result {
            RenderSourceResult::Packet { frames_provided, .. } => {
                assert_eq!(frames_provided, 100);
            }
            _ => panic!("Expected Packet result on iteration {}", i),
        }
    }
    
    let result = source.process_request(&request).unwrap();
    assert_eq!(result, RenderSourceResult::Exhausted);
    
    let snapshot = source.snapshot();
    assert_eq!(snapshot.packets_provided, 3);
    assert_eq!(snapshot.frames_read, 300);
    assert!(snapshot.is_exhausted);
}

#[test]
fn inmemory_source_reset() {
    let mut source = InMemoryRenderSource::with_test_packets(2, 100);
    
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 100,
        sample_rate: 44100,
        channel_count: 2,
    };
    source.process_request(&request).unwrap();
    
    source.reset();
    
    assert!(source.is_ready());
    assert!(!source.is_exhausted());
    
    let snapshot = source.snapshot();
    assert_eq!(snapshot.requests_accepted, 0);
    assert_eq!(snapshot.packets_provided, 0);
    assert_eq!(snapshot.frames_read, 0);
    assert_eq!(source.queue_len(), 1);
}

#[test]
fn inmemory_source_partial_frame_read() {
    let mut source = InMemoryRenderSource::with_test_packets(1, 1024);
    
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 512,
        sample_rate: 44100,
        channel_count: 2,
    };
    
    let result = source.process_request(&request).unwrap();
    match result {
        RenderSourceResult::Packet {
            frames_provided, ..
        } => {
            assert_eq!(frames_provided, 512);
        }
        _ => panic!("Expected Packet result"),
    }
}

#[test]
fn inmemory_source_eos_packet() {
    let mut queue = SourceQueue::new();
    queue.push_eos();
    
    let mut source = InMemoryRenderSource::new(queue);
    
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 100,
        sample_rate: 44100,
        channel_count: 2,
    };
    
    let result = source.process_request(&request).unwrap();
    assert_eq!(result, RenderSourceResult::Exhausted);
    assert!(source.is_exhausted());
}