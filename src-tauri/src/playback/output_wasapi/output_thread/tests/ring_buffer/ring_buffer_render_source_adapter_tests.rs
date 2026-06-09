//! Ring buffer render source adapter integration tests.
//!
//! Tests for FakeRingBufferSource integration with adapter functions.

use crate::playback::output_wasapi::output_thread::runtime::render_source_adapter::AdapterOutcome;
use crate::playback::output_wasapi::output_thread::runtime::ring_buffer_render_source::{
    create_empty_source, create_source_with_eos, create_test_source, execute_test_dispatch,
    invoke_test_flush, invoke_test_peek, invoke_test_read, read_all_packets,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::RenderSource;
use crate::playback::output_wasapi::output_thread::sink_boundary::SinkResult;
use crate::playback::output_wasapi::output_thread::tests::render_source_adapter::render_source_adapter_helpers::MockSinkConsumer;

#[test]
fn create_test_source_helper() {
    let source = create_test_source(3, 1024);
    assert!(source.is_ready());
    assert_eq!(source.buffer_len(), 3);
}

#[test]
fn create_empty_source_helper() {
    let source = create_empty_source();
    assert!(source.is_ready());
    assert!(!source.is_exhausted());
    assert_eq!(source.buffer_len(), 0);
}

#[test]
fn create_source_with_eos_helper() {
    let source = create_source_with_eos(2, 512);
    assert!(source.has_eos());
    assert_eq!(source.buffer_len(), 3); // 2 normal + 1 eos
}

#[test]
fn adapter_invoke_test_read_success() {
    let mut source = create_test_source(2, 1024);
    let outcome = invoke_test_read(&mut source, 1024, 44100, 2).unwrap();

    match outcome {
        AdapterOutcome::Packet {
            frames_provided,
            bytes_read,
        } => {
            assert_eq!(frames_provided, 1024);
            assert_eq!(bytes_read, 1024 * 2 * 4);
        }
        _ => panic!("Expected Packet outcome"),
    }
}

#[test]
fn adapter_invoke_test_read_exhausted() {
    let mut source = create_empty_source();
    let outcome = invoke_test_read(&mut source, 1024, 44100, 2).unwrap();
    assert_eq!(outcome, AdapterOutcome::Exhausted);
}

#[test]
fn adapter_invoke_test_peek_with_packets() {
    let mut source = create_test_source(1, 100);
    let outcome = invoke_test_peek(&mut source).unwrap();
    assert_eq!(outcome, AdapterOutcome::Noop);
}

#[test]
fn adapter_invoke_test_peek_empty() {
    let mut source = create_empty_source();
    let outcome = invoke_test_peek(&mut source).unwrap();
    assert_eq!(outcome, AdapterOutcome::Skipped);
}

#[test]
fn adapter_invoke_test_flush() {
    let mut source = create_test_source(3, 100);
    assert_eq!(source.buffer_len(), 3);

    let outcome = invoke_test_flush(&mut source).unwrap();
    assert_eq!(outcome, AdapterOutcome::Noop);
    assert_eq!(source.buffer_len(), 0);
}

#[test]
fn adapter_read_all_packets_helper() {
    let mut source = create_test_source(3, 100);
    let (total_frames, total_bytes) = read_all_packets(&mut source, 100, 44100, 2);

    assert_eq!(total_frames, 300);
    assert_eq!(total_bytes, 300 * 2 * 4);
    assert!(source.is_exhausted());
}

#[test]
fn adapter_read_all_packets_empty() {
    let mut source = create_empty_source();
    let (total_frames, total_bytes) = read_all_packets(&mut source, 100, 44100, 2);

    assert_eq!(total_frames, 0);
    assert_eq!(total_bytes, 0);
}

#[test]
fn adapter_read_all_packets_with_eos() {
    let mut source = create_source_with_eos(2, 100);
    let (total_frames, total_bytes) = read_all_packets(&mut source, 100, 44100, 2);

    assert_eq!(total_frames, 200);
    assert_eq!(total_bytes, 200 * 2 * 4);
    assert!(source.is_exhausted());
}

#[test]
fn adapter_execute_test_dispatch_success() {
    let mut source = create_test_source(1, 1024);
    let mut consumer = MockSinkConsumer::new().with_result(Ok(SinkResult::default()));

    let outcome = execute_test_dispatch(&mut source, &mut consumer, 1024, 44100, 2).unwrap();

    match outcome.source_outcome {
        AdapterOutcome::Packet {
            frames_provided, ..
        } => assert_eq!(frames_provided, 1024),
        _ => panic!("Expected Packet source outcome"),
    }
}

#[test]
fn adapter_execute_test_dispatch_exhausted_source() {
    let mut source = create_empty_source();
    let mut consumer = MockSinkConsumer::new().with_result(Ok(SinkResult::default()));

    let outcome = execute_test_dispatch(&mut source, &mut consumer, 1024, 44100, 2).unwrap();
    assert_eq!(outcome.source_outcome, AdapterOutcome::Exhausted);
}

#[test]
fn adapter_invoke_test_read_multiple_packets() {
    let mut source = create_test_source(3, 512);

    // Read first packet
    let outcome1 = invoke_test_read(&mut source, 512, 44100, 2).unwrap();
    assert_eq!(
        outcome1,
        AdapterOutcome::Packet {
            frames_provided: 512,
            bytes_read: 512 * 2 * 4,
        }
    );

    // Read second packet
    let outcome2 = invoke_test_read(&mut source, 512, 44100, 2).unwrap();
    assert_eq!(
        outcome2,
        AdapterOutcome::Packet {
            frames_provided: 512,
            bytes_read: 512 * 2 * 4,
        }
    );

    // Read third packet
    let outcome3 = invoke_test_read(&mut source, 512, 44100, 2).unwrap();
    assert_eq!(
        outcome3,
        AdapterOutcome::Packet {
            frames_provided: 512,
            bytes_read: 512 * 2 * 4,
        }
    );

    // Should be exhausted
    let outcome4 = invoke_test_read(&mut source, 512, 44100, 2).unwrap();
    assert_eq!(outcome4, AdapterOutcome::Exhausted);
}

#[test]
fn adapter_source_snapshot_after_adapter_use() {
    let mut source = create_test_source(2, 100);

    invoke_test_read(&mut source, 100, 44100, 2).unwrap();
    invoke_test_read(&mut source, 100, 44100, 2).unwrap();
    // Third read triggers exhaustion (buffer is empty).
    invoke_test_read(&mut source, 100, 44100, 2).unwrap();

    let snapshot = source.snapshot();
    assert_eq!(snapshot.requests_accepted, 3);
    assert_eq!(snapshot.packets_provided, 2);
    assert_eq!(snapshot.frames_read, 200);
    assert_eq!(snapshot.bytes_read, 200 * 2 * 4);
    assert!(snapshot.is_exhausted);
}

#[test]
fn adapter_source_cursor_after_adapter_use() {
    let mut source = create_test_source(2, 100);

    invoke_test_read(&mut source, 100, 44100, 2).unwrap();

    let cursor = source.cursor();
    assert_eq!(cursor.position_frames, 100);
    // total_frames from buffer sums remaining items only.
    assert_eq!(cursor.total_frames, 100);
}
