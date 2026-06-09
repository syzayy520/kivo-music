//! Runtime spawn boundary contract tests.
//!
//! Tests for SpawnRequest, SpawnResult, and SpawnRejectReason pure data types.

use crate::playback::output_wasapi::output_thread::config::ThreadConfig;
use crate::playback::output_wasapi::output_thread::runtime::spawn::{
    SpawnRejectReason, SpawnRequest, SpawnResult,
};
use crate::playback::output_wasapi::output_thread::runtime::thread_handle::ThreadHandle;

// --- SpawnRequest tests ---

#[test]
fn spawn_request_new_defaults_auto_start() {
    let handle = ThreadHandle::new(1, 0);
    let config = ThreadConfig::default();
    let req = SpawnRequest::new(handle, config.clone());

    assert_eq!(req.handle, handle);
    assert_eq!(req.config, config);
    assert!(req.auto_start);
}

#[test]
fn spawn_request_with_auto_start_false() {
    let handle = ThreadHandle::new(1, 0);
    let config = ThreadConfig::default();
    let req = SpawnRequest::new(handle, config).with_auto_start(false);

    assert!(!req.auto_start);
}

#[test]
fn spawn_request_clone() {
    let handle = ThreadHandle::new(1, 0);
    let config = ThreadConfig::default();
    let a = SpawnRequest::new(handle, config);
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn spawn_request_debug() {
    let handle = ThreadHandle::new(1, 0);
    let config = ThreadConfig::default();
    let req = SpawnRequest::new(handle, config);
    let debug = format!("{:?}", req);
    assert!(debug.contains("SpawnRequest"));
}

#[test]
fn spawn_request_different_handles_not_equal() {
    let h1 = ThreadHandle::new(1, 0);
    let h2 = ThreadHandle::new(2, 0);
    let config = ThreadConfig::default();
    let r1 = SpawnRequest::new(h1, config.clone());
    let r2 = SpawnRequest::new(h2, config);
    assert_ne!(r1, r2);
}

// --- SpawnResult tests ---

#[test]
fn spawn_result_spawned() {
    let handle = ThreadHandle::new(1, 0);
    let result = SpawnResult::Spawned { handle };
    assert_eq!(result, SpawnResult::Spawned { handle });
}

#[test]
fn spawn_result_rejected() {
    let result = SpawnResult::Rejected {
        reason: SpawnRejectReason::AlreadyExists,
    };
    match result {
        SpawnResult::Rejected { reason } => assert_eq!(reason, SpawnRejectReason::AlreadyExists),
        _ => panic!("Expected Rejected"),
    }
}

#[test]
fn spawn_result_clone() {
    let handle = ThreadHandle::new(1, 0);
    let a = SpawnResult::Spawned { handle };
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn spawn_result_debug() {
    let result = SpawnResult::Rejected {
        reason: SpawnRejectReason::ResourceExhausted,
    };
    let debug = format!("{:?}", result);
    assert!(debug.contains("ResourceExhausted"));
}

#[test]
fn spawn_result_hash_eq() {
    use std::collections::HashSet;
    let a = SpawnResult::Rejected {
        reason: SpawnRejectReason::InvalidConfig,
    };
    let b = SpawnResult::Rejected {
        reason: SpawnRejectReason::InvalidConfig,
    };
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b);
    assert_eq!(set.len(), 1);
}

// --- SpawnRejectReason tests ---

#[test]
fn spawn_reject_reason_variants_distinct() {
    let a = SpawnRejectReason::AlreadyExists;
    let b = SpawnRejectReason::ResourceExhausted;
    let c = SpawnRejectReason::InvalidConfig;
    assert_ne!(a, b);
    assert_ne!(b, c);
    assert_ne!(a, c);
}

#[test]
fn spawn_reject_reason_clone() {
    let a = SpawnRejectReason::AlreadyExists;
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn spawn_reject_reason_debug() {
    let r = SpawnRejectReason::ResourceExhausted;
    let debug = format!("{:?}", r);
    assert!(debug.contains("ResourceExhausted"));
}

#[test]
fn spawn_reject_reason_hash_eq() {
    use std::collections::HashSet;
    let a = SpawnRejectReason::InvalidConfig;
    let b = SpawnRejectReason::InvalidConfig;
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b);
    assert_eq!(set.len(), 1);
}
