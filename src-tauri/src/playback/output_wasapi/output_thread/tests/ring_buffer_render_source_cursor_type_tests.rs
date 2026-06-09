//! BufferCursor type tests.

use crate::playback::output_wasapi::output_thread::runtime::ring_buffer_render_source::BufferCursor;

#[test]
fn buffer_cursor_default() {
    let cursor = BufferCursor::default();
    assert_eq!(cursor.read_position, 0);
    assert_eq!(cursor.write_position, 0);
    assert_eq!(cursor.buffer_capacity, 0);
    assert_eq!(cursor.available_frames, 0);
    assert_eq!(cursor.sample_rate, 0);
    assert_eq!(cursor.channel_count, 0);
    assert_eq!(cursor.wrap_count, 0);
}

#[test]
fn buffer_cursor_is_empty() {
    let mut cursor = BufferCursor::default();
    assert!(cursor.is_empty());

    cursor.available_frames = 100;
    assert!(!cursor.is_empty());
}

#[test]
fn buffer_cursor_is_full() {
    let mut cursor = BufferCursor::default();
    assert!(!cursor.is_full());

    cursor.buffer_capacity = 1000;
    cursor.available_frames = 500;
    assert!(!cursor.is_full());

    cursor.available_frames = 1000;
    assert!(cursor.is_full());

    cursor.available_frames = 1100;
    assert!(cursor.is_full());
}

#[test]
fn buffer_cursor_free_frames() {
    let mut cursor = BufferCursor::default();
    assert_eq!(cursor.free_frames(), 0);

    cursor.buffer_capacity = 1000;
    cursor.available_frames = 300;
    assert_eq!(cursor.free_frames(), 700);

    cursor.available_frames = 1000;
    assert_eq!(cursor.free_frames(), 0);

    cursor.available_frames = 1100;
    assert_eq!(cursor.free_frames(), 0);
}

#[test]
fn buffer_cursor_fill_percentage() {
    let mut cursor = BufferCursor::default();
    assert_eq!(cursor.fill_percentage(), 0);

    cursor.buffer_capacity = 1000;
    cursor.available_frames = 0;
    assert_eq!(cursor.fill_percentage(), 0);

    cursor.available_frames = 250;
    assert_eq!(cursor.fill_percentage(), 25);

    cursor.available_frames = 500;
    assert_eq!(cursor.fill_percentage(), 50);

    cursor.available_frames = 1000;
    assert_eq!(cursor.fill_percentage(), 100);

    cursor.available_frames = 1100;
    assert_eq!(cursor.fill_percentage(), 100);
}

#[test]
fn buffer_cursor_has_wrapped() {
    let mut cursor = BufferCursor::default();
    assert!(!cursor.has_wrapped());

    cursor.wrap_count = 1;
    assert!(cursor.has_wrapped());
}

#[test]
fn buffer_cursor_total_frames_read() {
    let mut cursor = BufferCursor::default();
    assert_eq!(cursor.total_frames_read(), 0);

    cursor.read_position = 12345;
    assert_eq!(cursor.total_frames_read(), 12345);
}

#[test]
fn buffer_cursor_clone() {
    let mut cursor = BufferCursor::default();
    cursor.read_position = 100;
    cursor.write_position = 200;
    cursor.buffer_capacity = 1000;
    let cloned = cursor.clone();
    assert_eq!(cursor, cloned);
}

#[test]
fn buffer_cursor_debug_format() {
    let cursor = BufferCursor::default();
    let debug_str = format!("{:?}", cursor);
    assert!(debug_str.contains("BufferCursor"));
}
