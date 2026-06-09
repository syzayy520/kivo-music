//! Sink dispatch context tests.
//!
//! Tests for DispatchContext type.

use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::DispatchContext;

// ── DispatchContext tests ──────────────────────────────────────────────

#[test]
fn dispatch_context_default() {
    let ctx = DispatchContext::default();
    assert_eq!(ctx.driver_result, DriverResult::Idle);
    assert_eq!(ctx.frame_count, 0);
    assert_eq!(ctx.sample_rate, 44100);
    assert_eq!(ctx.channel_count, 2);
}

#[test]
fn dispatch_context_custom() {
    let ctx = DispatchContext {
        driver_result: DriverResult::Continue,
        frame_count: 960,
        sample_rate: 48000,
        channel_count: 2,
    };
    assert_eq!(ctx.driver_result, DriverResult::Continue);
    assert_eq!(ctx.frame_count, 960);
    assert_eq!(ctx.sample_rate, 48000);
    assert_eq!(ctx.channel_count, 2);
}

#[test]
fn dispatch_context_clone() {
    let a = DispatchContext {
        driver_result: DriverResult::Stop,
        frame_count: 100,
        sample_rate: 44100,
        channel_count: 1,
    };
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn dispatch_context_equality() {
    let a = DispatchContext::default();
    let b = DispatchContext::default();
    assert_eq!(a, b);
}

#[test]
fn dispatch_context_debug() {
    let ctx = DispatchContext::default();
    let dbg = format!("{:?}", ctx);
    assert!(dbg.contains("DispatchContext"));
}

#[test]
fn dispatch_context_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a = DispatchContext::default();
    let b = DispatchContext::default();
    let mut h1 = DefaultHasher::new();
    let mut h2 = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}
