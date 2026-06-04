use super::output_thread_mock_buffer::OutputThreadMockBuffer;

#[test]
fn empty_open_buffer_snapshot_is_empty_open() {
    let buffer = OutputThreadMockBuffer::empty_open();
    let snap = buffer.snapshot();
    assert_eq!(snap.available_frames, 0);
    assert!(!snap.is_closed);
}

#[test]
fn buffer_with_frames_snapshot_has_frames() {
    let buffer = OutputThreadMockBuffer::with_frames(256);
    let snap = buffer.snapshot();
    assert_eq!(snap.available_frames, 256);
    assert!(!snap.is_closed);
}

#[test]
fn closed_empty_buffer_snapshot_is_closed_empty() {
    let buffer = OutputThreadMockBuffer::closed_empty();
    let snap = buffer.snapshot();
    assert_eq!(snap.available_frames, 0);
    assert!(snap.is_closed);
}

#[test]
fn closed_with_frames_snapshot_can_drain() {
    let buffer = OutputThreadMockBuffer::closed_with_frames(128);
    let snap = buffer.snapshot();
    assert_eq!(snap.available_frames, 128);
    assert!(snap.is_closed);
}

#[test]
fn consume_reduces_available_frames() {
    let buffer = OutputThreadMockBuffer::with_frames(100);
    let after = buffer.consume(30);
    assert_eq!(after.snapshot().available_frames, 70);
}

#[test]
fn consume_more_than_available_clamps_to_zero() {
    let buffer = OutputThreadMockBuffer::with_frames(50);
    let after = buffer.consume(200);
    assert_eq!(after.snapshot().available_frames, 0);
}

#[test]
fn consume_preserves_closed_state() {
    let buffer = OutputThreadMockBuffer::closed_with_frames(100);
    let after = buffer.consume(40);
    assert_eq!(after.snapshot().available_frames, 60);
    assert!(after.snapshot().is_closed);
}
