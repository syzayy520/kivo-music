//! BufferState type tests.

use crate::playback::output_wasapi::output_thread::runtime::ring_buffer_render_source::BufferState;

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
