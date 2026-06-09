//! Render packet audio tests.
//!
//! Tests for PacketTiming and AudioPacket types.

use crate::playback::output_wasapi::output_thread::sink_boundary::{
    AudioPacket, PacketFormat, PacketTiming, SampleFormat,
};

// ===== PacketTiming tests =====

#[test]
fn packet_timing_default() {
    let timing = PacketTiming::default();
    assert_eq!(timing.timestamp_us, 0);
    assert_eq!(timing.sequence, 0);
    assert_eq!(timing.duration_us, 0);
}

#[test]
fn packet_timing_custom() {
    let timing = PacketTiming {
        timestamp_us: 1000000,
        sequence: 42,
        duration_us: 23219,
    };
    assert_eq!(timing.timestamp_us, 1000000);
    assert_eq!(timing.sequence, 42);
    assert_eq!(timing.duration_us, 23219);
}

#[test]
fn packet_timing_equality() {
    let a = PacketTiming {
        timestamp_us: 100,
        sequence: 1,
        duration_us: 50,
    };
    let b = PacketTiming {
        timestamp_us: 100,
        sequence: 1,
        duration_us: 50,
    };
    assert_eq!(a, b);
}

#[test]
fn packet_timing_inequality() {
    let a = PacketTiming::default();
    let b = PacketTiming {
        sequence: 1,
        ..Default::default()
    };
    assert_ne!(a, b);
}

#[test]
fn packet_timing_clone() {
    let original = PacketTiming::default();
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn packet_timing_debug() {
    let timing = PacketTiming::default();
    let debug = format!("{:?}", timing);
    assert!(debug.contains("timestamp_us"));
}

// ===== AudioPacket tests =====

#[test]
fn audio_packet_default() {
    let pkt = AudioPacket::default();
    assert_eq!(pkt.frame_count, 0);
    assert_eq!(pkt.format, PacketFormat::default());
    assert_eq!(pkt.timing, PacketTiming::default());
}

#[test]
fn audio_packet_custom() {
    let pkt = AudioPacket {
        frame_count: 1024,
        format: PacketFormat {
            sample_rate: 48000,
            channel_count: 2,
            sample_format: SampleFormat::Float32,
            bits_per_sample: 32,
        },
        timing: PacketTiming {
            timestamp_us: 1000000,
            sequence: 1,
            duration_us: 21333,
        },
    };
    assert_eq!(pkt.frame_count, 1024);
    assert_eq!(pkt.format.sample_rate, 48000);
    assert_eq!(pkt.timing.sequence, 1);
}

#[test]
fn audio_packet_equality() {
    let a = AudioPacket {
        frame_count: 512,
        format: PacketFormat::default(),
        timing: PacketTiming::default(),
    };
    let b = AudioPacket {
        frame_count: 512,
        format: PacketFormat::default(),
        timing: PacketTiming::default(),
    };
    assert_eq!(a, b);
}

#[test]
fn audio_packet_inequality() {
    let a = AudioPacket::default();
    let b = AudioPacket {
        frame_count: 100,
        ..Default::default()
    };
    assert_ne!(a, b);
}

#[test]
fn audio_packet_clone() {
    let original = AudioPacket::default();
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn audio_packet_debug() {
    let pkt = AudioPacket::default();
    let debug = format!("{:?}", pkt);
    assert!(debug.contains("frame_count"));
}
