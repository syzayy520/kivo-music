//! Ring buffer render source type tests.
//!
//! Tests for BufferPacket, BufferCursor, and BufferState construction and properties.

use crate::playback::output_wasapi::output_thread::runtime::ring_buffer_render_source::{
    BufferCursor, BufferPacket, BufferState,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::render_packet::PacketFormat;

#[test]
fn buffer_packet_new() {
    let packet = BufferPacket::new(1, 512);
    assert_eq!(packet.packet_id, 1);
    assert_eq!(packet.frame_count, 512);
    assert!(!packet.is_eos);
    assert_eq!(packet.buffer_write_position, 0);
    assert_eq!(packet.sequence_number, 0);
}

#[test]
fn buffer_packet_with_position() {
    let packet = BufferPacket::with_position(2, 1024, 2048, 5);
    assert_eq!(packet.packet_id, 2);
    assert_eq!(packet.frame_count, 1024);
    assert_eq!(packet.buffer_write_position, 2048);
    assert_eq!(packet.sequence_number, 5);
}

#[test]
fn buffer_packet_eos() {
    let packet = BufferPacket::eos(3);
    assert_eq!(packet.packet_id, 3);
    assert!(packet.is_eos);
    assert!(packet.is_end_of_stream());
}

#[test]
fn buffer_packet_byte_size() {
    let mut packet = BufferPacket::new(1, 100);
    packet.format = PacketFormat {
        channel_count: 2,
        ..Default::default()
    };
    // 100 frames * 2 channels * 4 bytes (f32) = 800 bytes
    assert_eq!(packet.byte_size(), 800);
}

#[test]
fn buffer_packet_buffer_end_position() {
    let packet = BufferPacket::with_position(1, 512, 1000, 1);
    assert_eq!(packet.buffer_end_position(), 1512);
}

#[test]
fn buffer_packet_default() {
    let packet = BufferPacket::default();
    assert_eq!(packet.packet_id, 0);
    assert_eq!(packet.frame_count, 0);
    assert!(!packet.is_eos);
    assert_eq!(packet.buffer_write_position, 0);
    assert_eq!(packet.sequence_number, 0);
}

#[test]
fn buffer_packet_clone() {
    let packet = BufferPacket::new(1, 512);
    let cloned = packet.clone();
    assert_eq!(packet, cloned);
}

#[test]
fn buffer_packet_debug_format() {
    let packet = BufferPacket::new(1, 512);
    let debug_str = format!("{:?}", packet);
    assert!(debug_str.contains("BufferPacket"));
    assert!(debug_str.contains("packet_id: 1"));
}

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

#[test]
fn buffer_state_default() {
    let state = BufferState::default();
    assert_eq!(state.requests_accepted, 0);
    assert_eq!(state.packets_provided, 0);
    assert_eq!(state.frames_read, 0);
    assert_eq!(state.bytes_read, 0);
    assert_eq!(state.errors, 0);
    assert!(!state.is_exhausted);
    assert!(!state.is_ready);
    assert_eq!(state.packets_in_buffer, 0);
    assert_eq!(state.buffer_fill_frames, 0);
    assert_eq!(state.buffer_capacity_frames, 0);
    assert_eq!(state.buffer_wrap_count, 0);
    assert_eq!(state.underrun_count, 0);
    assert_eq!(state.overrun_count, 0);
}

#[test]
fn buffer_state_is_buffer_empty() {
    let mut state = BufferState::default();
    assert!(state.is_buffer_empty());

    state.buffer_fill_frames = 100;
    assert!(!state.is_buffer_empty());
}

#[test]
fn buffer_state_is_buffer_full() {
    let mut state = BufferState::default();
    assert!(!state.is_buffer_full());

    state.buffer_capacity_frames = 1000;
    state.buffer_fill_frames = 500;
    assert!(!state.is_buffer_full());

    state.buffer_fill_frames = 1000;
    assert!(state.is_buffer_full());

    state.buffer_fill_frames = 1100;
    assert!(state.is_buffer_full());
}

#[test]
fn buffer_state_buffer_fill_percentage() {
    let mut state = BufferState::default();
    assert_eq!(state.buffer_fill_percentage(), 0);

    state.buffer_capacity_frames = 1000;
    state.buffer_fill_frames = 0;
    assert_eq!(state.buffer_fill_percentage(), 0);

    state.buffer_fill_frames = 250;
    assert_eq!(state.buffer_fill_percentage(), 25);

    state.buffer_fill_frames = 500;
    assert_eq!(state.buffer_fill_percentage(), 50);

    state.buffer_fill_frames = 1000;
    assert_eq!(state.buffer_fill_percentage(), 100);

    state.buffer_fill_frames = 1100;
    assert_eq!(state.buffer_fill_percentage(), 100);
}

#[test]
fn buffer_state_average_packet_size() {
    let mut state = BufferState::default();
    assert_eq!(state.average_packet_size(), 0);

    state.packets_provided = 10;
    state.frames_read = 1000;
    assert_eq!(state.average_packet_size(), 100);

    state.packets_provided = 0;
    state.frames_read = 1000;
    assert_eq!(state.average_packet_size(), 0);
}

#[test]
fn buffer_state_has_underruns() {
    let mut state = BufferState::default();
    assert!(!state.has_underruns());

    state.underrun_count = 1;
    assert!(state.has_underruns());
}

#[test]
fn buffer_state_has_overruns() {
    let mut state = BufferState::default();
    assert!(!state.has_overruns());

    state.overrun_count = 1;
    assert!(state.has_overruns());
}

#[test]
fn buffer_state_clone() {
    let mut state = BufferState::default();
    state.requests_accepted = 10;
    state.packets_provided = 5;
    let cloned = state.clone();
    assert_eq!(state, cloned);
}

#[test]
fn buffer_state_debug_format() {
    let state = BufferState::default();
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("BufferState"));
}
