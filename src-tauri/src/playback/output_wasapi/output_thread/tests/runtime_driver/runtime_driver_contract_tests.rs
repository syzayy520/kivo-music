//! Runtime driver contract tests.
//!
//! Tests for ThreadDriver, DriverStep, and DriverResult pure data types.

use crate::playback::output_wasapi::output_thread::config::ThreadConfig;
use crate::playback::output_wasapi::output_thread::runtime::driver::{
    DriverResult, DriverStep, ThreadDriver,
};
use crate::playback::output_wasapi::output_thread::runtime::thread_handle::ThreadHandle;

// --- ThreadDriver tests ---

#[test]
fn thread_driver_new_stores_fields() {
    let handle = ThreadHandle::new(1, 0);
    let config = ThreadConfig::default();
    let driver = ThreadDriver::new(handle, config.clone());

    assert_eq!(driver.handle, handle);
    assert_eq!(driver.config, config);
    assert_eq!(driver.max_idle_steps, 100);
}

#[test]
fn thread_driver_with_max_idle_steps() {
    let handle = ThreadHandle::new(1, 0);
    let config = ThreadConfig::default();
    let driver = ThreadDriver::new(handle, config).with_max_idle_steps(50);

    assert_eq!(driver.max_idle_steps, 50);
}

#[test]
fn thread_driver_clone() {
    let handle = ThreadHandle::new(1, 0);
    let config = ThreadConfig::default();
    let a = ThreadDriver::new(handle, config);
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn thread_driver_debug() {
    let handle = ThreadHandle::new(1, 0);
    let config = ThreadConfig::default();
    let driver = ThreadDriver::new(handle, config);
    let debug = format!("{:?}", driver);
    assert!(debug.contains("ThreadDriver"));
}

#[test]
fn thread_driver_different_handles_not_equal() {
    let h1 = ThreadHandle::new(1, 0);
    let h2 = ThreadHandle::new(2, 0);
    let config = ThreadConfig::default();
    let d1 = ThreadDriver::new(h1, config.clone());
    let d2 = ThreadDriver::new(h2, config);
    assert_ne!(d1, d2);
}

// --- DriverStep tests ---

#[test]
fn driver_step_variants_distinct() {
    let a = DriverStep::ProcessCommand;
    let b = DriverStep::RenderCycle;
    let c = DriverStep::Idle;
    let d = DriverStep::Drain;
    let e = DriverStep::Shutdown;
    assert_ne!(a, b);
    assert_ne!(b, c);
    assert_ne!(c, d);
    assert_ne!(d, e);
}

#[test]
fn driver_step_clone() {
    let a = DriverStep::RenderCycle;
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn driver_step_debug() {
    let step = DriverStep::ProcessCommand;
    let debug = format!("{:?}", step);
    assert!(debug.contains("ProcessCommand"));
}

#[test]
fn driver_step_hash_eq() {
    use std::collections::HashSet;
    let a = DriverStep::Idle;
    let b = DriverStep::Idle;
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b);
    assert_eq!(set.len(), 1);
}

// --- DriverResult tests ---

#[test]
fn driver_result_variants_distinct() {
    let a = DriverResult::Continue;
    let b = DriverResult::Stop;
    let c = DriverResult::Error;
    let d = DriverResult::Idle;
    assert_ne!(a, b);
    assert_ne!(b, c);
    assert_ne!(c, d);
}

#[test]
fn driver_result_clone() {
    let a = DriverResult::Continue;
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn driver_result_debug() {
    let r = DriverResult::Stop;
    let debug = format!("{:?}", r);
    assert!(debug.contains("Stop"));
}

#[test]
fn driver_result_hash_eq() {
    use std::collections::HashSet;
    let a = DriverResult::Error;
    let b = DriverResult::Error;
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b);
    assert_eq!(set.len(), 1);
}
