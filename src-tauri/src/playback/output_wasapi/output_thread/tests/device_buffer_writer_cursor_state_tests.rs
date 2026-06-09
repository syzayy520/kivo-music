//! Device buffer writer cursor and state tests.
//!
//! Tests for WriterCursor and WriterState construction and properties.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    WriterCursor, WriterState,
};

#[test]
fn writer_cursor_default() {
    let cursor = WriterCursor::default();
    assert_eq!(cursor.write_position, 0);
    assert_eq!(cursor.buffer_capacity, 0);
    assert_eq!(cursor.buffered_frames, 0);
    assert_eq!(cursor.sample_rate, 0);
    assert_eq!(cursor.channel_count, 0);
    assert_eq!(cursor.total_frames_written, 0);
    assert_eq!(cursor.wrap_count, 0);
}

#[test]
fn writer_cursor_is_empty() {
    let mut cursor = WriterCursor::default();
    assert!(cursor.is_empty());
    cursor.buffered_frames = 100;
    assert!(!cursor.is_empty());
}

#[test]
fn writer_cursor_is_full() {
    let mut cursor = WriterCursor::default();
    assert!(!cursor.is_full());
    cursor.buffer_capacity = 1000;
    cursor.buffered_frames = 500;
    assert!(!cursor.is_full());
    cursor.buffered_frames = 1000;
    assert!(cursor.is_full());
    cursor.buffered_frames = 1100;
    assert!(cursor.is_full());
}

#[test]
fn writer_cursor_free_frames() {
    let mut cursor = WriterCursor::default();
    assert_eq!(cursor.free_frames(), 0);
    cursor.buffer_capacity = 1000;
    cursor.buffered_frames = 300;
    assert_eq!(cursor.free_frames(), 700);
    cursor.buffered_frames = 1000;
    assert_eq!(cursor.free_frames(), 0);
}

#[test]
fn writer_cursor_fill_percentage() {
    let mut cursor = WriterCursor::default();
    assert_eq!(cursor.fill_percentage(), 0);
    cursor.buffer_capacity = 1000;
    assert_eq!(cursor.fill_percentage(), 0);
    cursor.buffered_frames = 250;
    assert_eq!(cursor.fill_percentage(), 25);
    cursor.buffered_frames = 500;
    assert_eq!(cursor.fill_percentage(), 50);
    cursor.buffered_frames = 1000;
    assert_eq!(cursor.fill_percentage(), 100);
    cursor.buffered_frames = 1100;
    assert_eq!(cursor.fill_percentage(), 100);
}

#[test]
fn writer_cursor_has_wrapped() {
    let mut cursor = WriterCursor::default();
    assert!(!cursor.has_wrapped());
    cursor.wrap_count = 1;
    assert!(cursor.has_wrapped());
}

#[test]
fn writer_cursor_session_frames_written() {
    let mut cursor = WriterCursor::default();
    assert_eq!(cursor.session_frames_written(), 0);
    cursor.total_frames_written = 5000;
    assert_eq!(cursor.session_frames_written(), 5000);
}

#[test]
fn writer_cursor_clone_debug() {
    let mut cursor = WriterCursor::default();
    cursor.write_position = 100;
    cursor.buffer_capacity = 1000;
    let cloned = cursor.clone();
    assert_eq!(cursor, cloned);
    assert!(format!("{:?}", cursor).contains("WriterCursor"));
}

#[test]
fn writer_state_default() {
    let state = WriterState::default();
    assert_eq!(state.requests_accepted, 0);
    assert_eq!(state.writes_completed, 0);
    assert_eq!(state.frames_written, 0);
    assert_eq!(state.bytes_written, 0);
    assert_eq!(state.errors, 0);
    assert!(!state.is_closed);
    assert!(!state.is_ready);
    assert_eq!(state.buffer_fill_frames, 0);
    assert_eq!(state.buffer_capacity_frames, 0);
    assert_eq!(state.buffer_wrap_count, 0);
    assert_eq!(state.would_block_count, 0);
    assert_eq!(state.flush_count, 0);
}

#[test]
fn writer_state_is_buffer_empty() {
    let mut state = WriterState::default();
    assert!(state.is_buffer_empty());
    state.buffer_fill_frames = 100;
    assert!(!state.is_buffer_empty());
}

#[test]
fn writer_state_is_buffer_full() {
    let mut state = WriterState::default();
    assert!(!state.is_buffer_full());
    state.buffer_capacity_frames = 1000;
    state.buffer_fill_frames = 500;
    assert!(!state.is_buffer_full());
    state.buffer_fill_frames = 1000;
    assert!(state.is_buffer_full());
}

#[test]
fn writer_state_buffer_fill_percentage() {
    let mut state = WriterState::default();
    assert_eq!(state.buffer_fill_percentage(), 0);
    state.buffer_capacity_frames = 1000;
    state.buffer_fill_frames = 250;
    assert_eq!(state.buffer_fill_percentage(), 25);
    state.buffer_fill_frames = 500;
    assert_eq!(state.buffer_fill_percentage(), 50);
}

#[test]
fn writer_state_average_write_size() {
    let mut state = WriterState::default();
    assert_eq!(state.average_write_size(), 0);
    state.writes_completed = 10;
    state.frames_written = 1000;
    assert_eq!(state.average_write_size(), 100);
}

#[test]
fn writer_state_has_would_blocks() {
    let mut state = WriterState::default();
    assert!(!state.has_would_blocks());
    state.would_block_count = 1;
    assert!(state.has_would_blocks());
}

#[test]
fn writer_state_has_errors() {
    let mut state = WriterState::default();
    assert!(!state.has_errors());
    state.errors = 1;
    assert!(state.has_errors());
}

#[test]
fn writer_state_clone_debug() {
    let mut state = WriterState::default();
    state.requests_accepted = 10;
    state.writes_completed = 5;
    let cloned = state.clone();
    assert_eq!(state, cloned);
    assert!(format!("{:?}", state).contains("WriterState"));
}
