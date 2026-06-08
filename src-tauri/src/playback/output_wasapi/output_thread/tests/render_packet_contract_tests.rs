//! Render packet contract tests.
//!
//! Tests for SampleFormat and PacketFormat types.

use crate::playback::output_wasapi::output_thread::sink_boundary::{
    PacketFormat, SampleFormat,
};

// ===== SampleFormat tests =====

#[test]
fn sample_format_default_is_float32() {
    let fmt = SampleFormat::default();
    assert_eq!(fmt, SampleFormat::Float32);
}

#[test]
fn sample_format_variants_exist() {
    let _ = SampleFormat::Float32;
    let _ = SampleFormat::Int16;
    let _ = SampleFormat::Int24;
    let _ = SampleFormat::Int32;
}

#[test]
fn sample_format_clone() {
    let original = SampleFormat::Int16;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn sample_format_debug() {
    let fmt = SampleFormat::Float32;
    let debug = format!("{:?}", fmt);
    assert_eq!(debug, "Float32");
}

// ===== PacketFormat tests =====

#[test]
fn packet_format_default() {
    let fmt = PacketFormat::default();
    assert_eq!(fmt.sample_rate, 44100);
    assert_eq!(fmt.channel_count, 2);
    assert_eq!(fmt.sample_format, SampleFormat::Float32);
    assert_eq!(fmt.bits_per_sample, 32);
}

#[test]
fn packet_format_custom() {
    let fmt = PacketFormat {
        sample_rate: 48000,
        channel_count: 1,
        sample_format: SampleFormat::Int16,
        bits_per_sample: 16,
    };
    assert_eq!(fmt.sample_rate, 48000);
    assert_eq!(fmt.channel_count, 1);
    assert_eq!(fmt.sample_format, SampleFormat::Int16);
    assert_eq!(fmt.bits_per_sample, 16);
}

#[test]
fn packet_format_equality() {
    let a = PacketFormat {
        sample_rate: 44100,
        channel_count: 2,
        sample_format: SampleFormat::Float32,
        bits_per_sample: 32,
    };
    let b = PacketFormat {
        sample_rate: 44100,
        channel_count: 2,
        sample_format: SampleFormat::Float32,
        bits_per_sample: 32,
    };
    assert_eq!(a, b);
}

#[test]
fn packet_format_inequality() {
    let a = PacketFormat::default();
    let b = PacketFormat {
        sample_rate: 48000,
        ..Default::default()
    };
    assert_ne!(a, b);
}

#[test]
fn packet_format_clone() {
    let original = PacketFormat::default();
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn packet_format_debug() {
    let fmt = PacketFormat::default();
    let debug = format!("{:?}", fmt);
    assert!(debug.contains("44100"));
}
