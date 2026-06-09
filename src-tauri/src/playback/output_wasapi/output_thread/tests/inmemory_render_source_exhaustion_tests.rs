//! In-memory render source exhaustion behavior tests.
//!
//! Tests for exhaustion detection, error paths, and edge cases.

use crate::playback::output_wasapi::output_thread::runtime::inmemory_render_source::{
    create_empty_source, create_mixed_size_source, create_source_with_eos, create_test_source,
    drain_until_exhausted, verify_repeated_exhaustion_returns_exhausted,
    verify_reset_allows_reread, DrainResult,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSource, RenderSourceRequest, RenderSourceResult,
};

#[test]
fn drain_single_packet() {
    let mut source = create_test_source(1, 512);
    let result = drain_until_exhausted(&mut source, 512, 44100, 2);

    assert_eq!(result.packets, 1);
    assert_eq!(result.frames, 512);
    assert_eq!(result.bytes, 512 * 2 * 4);
    assert!(result.is_exhausted);
}

#[test]
fn drain_multiple_packets() {
    let mut source = create_test_source(5, 256);
    let result = drain_until_exhausted(&mut source, 256, 44100, 2);

    assert_eq!(result.packets, 5);
    assert_eq!(result.frames, 1280);
    assert_eq!(result.bytes, 1280 * 2 * 4);
    assert!(result.is_exhausted);
}

#[test]
fn drain_empty_source() {
    let mut source = create_empty_source();
    let result = drain_until_exhausted(&mut source, 100, 44100, 2);

    assert_eq!(result.packets, 0);
    assert_eq!(result.frames, 0);
    assert_eq!(result.bytes, 0);
    assert!(result.is_exhausted);
}

#[test]
fn drain_source_with_eos() {
    let mut source = create_source_with_eos(3, 100);
    let result = drain_until_exhausted(&mut source, 100, 44100, 2);

    assert_eq!(result.packets, 3);
    assert_eq!(result.frames, 300);
    assert!(result.is_exhausted);
}

#[test]
fn repeated_exhaustion_returns_exhausted() {
    let mut source = create_test_source(1, 100);
    // Drain it.
    drain_until_exhausted(&mut source, 100, 44100, 2);
    assert!(source.is_exhausted());

    // Repeated reads should keep returning exhausted.
    let ok = verify_repeated_exhaustion_returns_exhausted(&mut source, 100, 44100, 2, 10);
    assert!(ok);
}

#[test]
fn reset_allows_reread() {
    let mut source = create_test_source(3, 100);
    let ok = verify_reset_allows_reread(&mut source, 100, 44100, 2);
    assert!(ok);
}

#[test]
fn reset_empty_source() {
    let mut source = create_empty_source();
    source.reset();
    assert!(source.is_ready());
    assert!(!source.is_exhausted());

    // Reading from empty source should exhaust.
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 100,
        sample_rate: 44100,
        channel_count: 2,
    };
    let result = source.process_request(&request).unwrap();
    assert_eq!(result, RenderSourceResult::Exhausted);
}

#[test]
fn mixed_size_packets_drain() {
    let mut source = create_mixed_size_source(&[64, 128, 256, 512]);
    let result = drain_until_exhausted(&mut source, 1024, 44100, 2);

    assert_eq!(result.packets, 4);
    assert_eq!(result.frames, 64 + 128 + 256 + 512);
    assert!(result.is_exhausted);
}

#[test]
fn partial_read_clamps_to_packet_size() {
    let mut source = create_test_source(1, 64);

    // Request more frames than the packet has.
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 1024,
        sample_rate: 44100,
        channel_count: 2,
    };
    let result = source.process_request(&request).unwrap();
    match result {
        RenderSourceResult::Packet {
            frames_provided, ..
        } => {
            assert_eq!(frames_provided, 64); // Clamped to packet size.
        }
        _ => panic!("Expected Packet result"),
    }
}

#[test]
fn snapshot_counts_after_drain() {
    let mut source = create_test_source(4, 128);
    drain_until_exhausted(&mut source, 128, 44100, 2);

    let snapshot = source.snapshot();
    assert_eq!(snapshot.packets_provided, 4);
    assert_eq!(snapshot.frames_read, 512);
    assert_eq!(snapshot.bytes_read, 512 * 2 * 4);
    assert!(snapshot.is_exhausted);
    assert!(!snapshot.is_ready);
    assert_eq!(snapshot.errors, 0);
}

#[test]
fn no_errors_during_normal_drain() {
    let mut source = create_test_source(10, 100);
    drain_until_exhausted(&mut source, 100, 44100, 2);

    let snapshot = source.snapshot();
    assert_eq!(snapshot.errors, 0);
}

#[test]
fn flush_then_read_returns_exhausted() {
    let mut source = create_test_source(3, 100);

    // Flush clears the queue.
    let request = RenderSourceRequest::Flush;
    source.process_request(&request).unwrap();

    // Next read should exhaust.
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 100,
        sample_rate: 44100,
        channel_count: 2,
    };
    let result = source.process_request(&request).unwrap();
    assert_eq!(result, RenderSourceResult::Exhausted);
}

#[test]
fn peek_after_drain_returns_exhausted() {
    let mut source = create_test_source(1, 100);
    drain_until_exhausted(&mut source, 100, 44100, 2);

    let request = RenderSourceRequest::Peek;
    let result = source.process_request(&request).unwrap();
    assert_eq!(result, RenderSourceResult::Exhausted);
}

#[test]
fn cursor_position_tracks_reads() {
    let mut source = create_test_source(3, 100);

    let request = RenderSourceRequest::ReadPacket {
        frame_count: 100,
        sample_rate: 44100,
        channel_count: 2,
    };

    source.process_request(&request).unwrap();
    assert_eq!(source.cursor().position_frames, 100);

    source.process_request(&request).unwrap();
    assert_eq!(source.cursor().position_frames, 200);

    source.process_request(&request).unwrap();
    assert_eq!(source.cursor().position_frames, 300);
}

#[test]
fn drain_result_clone_eq() {
    let r1 = DrainResult {
        packets: 3,
        frames: 300,
        bytes: 1200,
        is_exhausted: true,
    };
    let r2 = r1.clone();
    assert_eq!(r1, r2);
}
