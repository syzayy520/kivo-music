//! BufferPacket type tests.

use crate::playback::output_wasapi::output_thread::runtime::ring_buffer_render_source::BufferPacket;
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
