//! Config contract tests.
//!
//! Tests for config type default values, equality, and basic properties.

use crate::playback::output_wasapi::output_thread::config::{
    BufferConfig, RenderLoopConfig, ThreadConfig,
};

#[test]
fn buffer_config_default_values() {
    let config = BufferConfig::default();
    assert_eq!(config.capacity_frames, 8192);
    assert_eq!(config.low_watermark_frames, 1024);
    assert_eq!(config.high_watermark_frames, 6144);
}

#[test]
fn render_loop_config_default_values() {
    let config = RenderLoopConfig::default();
    assert_eq!(config.frames_per_cycle, 1024);
    assert_eq!(config.max_retries, 3);
    assert_eq!(config.idle_sleep_us, 1000);
}

#[test]
fn thread_config_default_values() {
    let config = ThreadConfig::default();
    assert_eq!(config.sample_rate, 44100);
    assert_eq!(config.channel_count, 2);
    assert_eq!(config.bits_per_sample, 16);
    assert_eq!(config.render_loop, RenderLoopConfig::default());
    assert_eq!(config.buffer, BufferConfig::default());
}

#[test]
fn buffer_config_equality() {
    let a = BufferConfig::default();
    let b = BufferConfig::default();
    let c = BufferConfig {
        capacity_frames: 4096,
        ..Default::default()
    };
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn render_loop_config_equality() {
    let a = RenderLoopConfig::default();
    let b = RenderLoopConfig::default();
    let c = RenderLoopConfig {
        frames_per_cycle: 512,
        ..Default::default()
    };
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn thread_config_equality() {
    let a = ThreadConfig::default();
    let b = ThreadConfig::default();
    let c = ThreadConfig {
        sample_rate: 48000,
        ..Default::default()
    };
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn buffer_config_clone() {
    let a = BufferConfig::default();
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn render_loop_config_copy() {
    let a = RenderLoopConfig::default();
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn thread_config_clone() {
    let a = ThreadConfig::default();
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn buffer_config_debug() {
    let config = BufferConfig::default();
    let debug = format!("{:?}", config);
    assert!(debug.contains("BufferConfig"));
    assert!(debug.contains("capacity_frames"));
}

#[test]
fn render_loop_config_debug() {
    let config = RenderLoopConfig::default();
    let debug = format!("{:?}", config);
    assert!(debug.contains("RenderLoopConfig"));
    assert!(debug.contains("frames_per_cycle"));
}

#[test]
fn thread_config_debug() {
    let config = ThreadConfig::default();
    let debug = format!("{:?}", config);
    assert!(debug.contains("ThreadConfig"));
    assert!(debug.contains("sample_rate"));
}

#[test]
fn buffer_config_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a = BufferConfig::default();
    let b = BufferConfig::default();

    let mut hasher_a = DefaultHasher::new();
    let mut hasher_b = DefaultHasher::new();

    a.hash(&mut hasher_a);
    b.hash(&mut hasher_b);

    assert_eq!(hasher_a.finish(), hasher_b.finish());
}

#[test]
fn render_loop_config_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a = RenderLoopConfig::default();
    let b = RenderLoopConfig::default();

    let mut hasher_a = DefaultHasher::new();
    let mut hasher_b = DefaultHasher::new();

    a.hash(&mut hasher_a);
    b.hash(&mut hasher_b);

    assert_eq!(hasher_a.finish(), hasher_b.finish());
}

#[test]
fn thread_config_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a = ThreadConfig::default();
    let b = ThreadConfig::default();

    let mut hasher_a = DefaultHasher::new();
    let mut hasher_b = DefaultHasher::new();

    a.hash(&mut hasher_a);
    b.hash(&mut hasher_b);

    assert_eq!(hasher_a.finish(), hasher_b.finish());
}

#[test]
fn buffer_config_custom_values() {
    let config = BufferConfig {
        capacity_frames: 16384,
        low_watermark_frames: 2048,
        high_watermark_frames: 12288,
    };
    assert_eq!(config.capacity_frames, 16384);
    assert_eq!(config.low_watermark_frames, 2048);
    assert_eq!(config.high_watermark_frames, 12288);
}

#[test]
fn render_loop_config_custom_values() {
    let config = RenderLoopConfig {
        frames_per_cycle: 2048,
        max_retries: 5,
        idle_sleep_us: 2000,
    };
    assert_eq!(config.frames_per_cycle, 2048);
    assert_eq!(config.max_retries, 5);
    assert_eq!(config.idle_sleep_us, 2000);
}

#[test]
fn thread_config_custom_values() {
    let config = ThreadConfig {
        sample_rate: 96000,
        channel_count: 6,
        bits_per_sample: 24,
        ..Default::default()
    };
    assert_eq!(config.sample_rate, 96000);
    assert_eq!(config.channel_count, 6);
    assert_eq!(config.bits_per_sample, 24);
}