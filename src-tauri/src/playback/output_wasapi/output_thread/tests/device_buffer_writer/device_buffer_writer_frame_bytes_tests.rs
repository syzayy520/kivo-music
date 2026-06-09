//! Device buffer writer frame byte sizing tests.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::frame_bytes::{
    f32_frame_byte_count, f32_packet_byte_count, F32_SAMPLE_BYTES,
};

#[test]
fn f32_sample_byte_count_is_four() {
    assert_eq!(F32_SAMPLE_BYTES, 4);
}

#[test]
fn f32_frame_byte_count_uses_channels_only() {
    assert_eq!(f32_frame_byte_count(1), 4);
    assert_eq!(f32_frame_byte_count(2), 8);
    assert_eq!(f32_frame_byte_count(6), 24);
}

#[test]
fn f32_packet_byte_count_uses_frame_count_and_channels() {
    assert_eq!(f32_packet_byte_count(512, 2), Some(4096));
    assert_eq!(f32_packet_byte_count(256, 1), Some(1024));
}

#[test]
fn f32_packet_byte_count_reports_overflow() {
    assert_eq!(f32_packet_byte_count(u64::MAX, 2), None);
}
