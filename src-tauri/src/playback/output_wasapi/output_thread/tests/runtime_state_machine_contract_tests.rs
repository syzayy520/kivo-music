//! Runtime state machine contract tests.
//!
//! Tests for Transition, TransitionResult, and TransitionError pure data types.

use crate::playback::output_wasapi::output_thread::runtime::state_machine::{
    Transition, TransitionError, TransitionResult,
};
use crate::playback::output_wasapi::output_thread::state::OutputThreadLifecycle;

// --- Transition tests ---

#[test]
fn transition_new_stores_fields() {
    let t = Transition::new(
        OutputThreadLifecycle::NotStarted,
        OutputThreadLifecycle::Starting,
    );
    assert_eq!(t.from, OutputThreadLifecycle::NotStarted);
    assert_eq!(t.to, OutputThreadLifecycle::Starting);
}

#[test]
fn transition_identity_true() {
    let t = Transition::new(
        OutputThreadLifecycle::Running,
        OutputThreadLifecycle::Running,
    );
    assert!(t.is_identity());
}

#[test]
fn transition_identity_false() {
    let t = Transition::new(
        OutputThreadLifecycle::Running,
        OutputThreadLifecycle::Stopped,
    );
    assert!(!t.is_identity());
}

#[test]
fn transition_clone() {
    let a = Transition::new(
        OutputThreadLifecycle::NotStarted,
        OutputThreadLifecycle::Starting,
    );
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn transition_debug() {
    let t = Transition::new(
        OutputThreadLifecycle::Running,
        OutputThreadLifecycle::Stopped,
    );
    let debug = format!("{:?}", t);
    assert!(debug.contains("Transition"));
    assert!(debug.contains("Running"));
    assert!(debug.contains("Stopped"));
}

#[test]
fn transition_hash_eq() {
    use std::collections::HashSet;
    let a = Transition::new(
        OutputThreadLifecycle::Running,
        OutputThreadLifecycle::Stopped,
    );
    let b = Transition::new(
        OutputThreadLifecycle::Running,
        OutputThreadLifecycle::Stopped,
    );
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b);
    assert_eq!(set.len(), 1);
}

// --- TransitionError tests ---

#[test]
fn transition_error_variants_distinct() {
    let a = TransitionError::InvalidTransition;
    let b = TransitionError::ConcurrencyConflict;
    let c = TransitionError::TimeoutExpired;
    assert_ne!(a, b);
    assert_ne!(b, c);
    assert_ne!(a, c);
}

#[test]
fn transition_error_clone() {
    let a = TransitionError::ConcurrencyConflict;
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn transition_error_debug() {
    let e = TransitionError::TimeoutExpired;
    let debug = format!("{:?}", e);
    assert!(debug.contains("TimeoutExpired"));
}

#[test]
fn transition_error_hash_eq() {
    use std::collections::HashSet;
    let a = TransitionError::InvalidTransition;
    let b = TransitionError::InvalidTransition;
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b);
    assert_eq!(set.len(), 1);
}

// --- TransitionResult tests ---

#[test]
fn transition_result_applied() {
    let t = Transition::new(
        OutputThreadLifecycle::NotStarted,
        OutputThreadLifecycle::Starting,
    );
    let r = TransitionResult::Applied(t);
    assert!(r.is_applied());
    assert!(!r.is_rejected());
    assert!(!r.is_already_at_target());
}

#[test]
fn transition_result_rejected() {
    let t = Transition::new(
        OutputThreadLifecycle::Stopped,
        OutputThreadLifecycle::Running,
    );
    let r = TransitionResult::Rejected(t, TransitionError::InvalidTransition);
    assert!(!r.is_applied());
    assert!(r.is_rejected());
    assert!(!r.is_already_at_target());
}

#[test]
fn transition_result_already_at_target() {
    let r = TransitionResult::AlreadyAtTarget;
    assert!(!r.is_applied());
    assert!(!r.is_rejected());
    assert!(r.is_already_at_target());
}

#[test]
fn transition_result_clone() {
    let t = Transition::new(
        OutputThreadLifecycle::Running,
        OutputThreadLifecycle::Stopped,
    );
    let a = TransitionResult::Applied(t);
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn transition_result_debug() {
    let r = TransitionResult::AlreadyAtTarget;
    let debug = format!("{:?}", r);
    assert!(debug.contains("AlreadyAtTarget"));
}

#[test]
fn transition_result_hash_eq() {
    use std::collections::HashSet;
    let a = TransitionResult::AlreadyAtTarget;
    let b = TransitionResult::AlreadyAtTarget;
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b);
    assert_eq!(set.len(), 1);
}
