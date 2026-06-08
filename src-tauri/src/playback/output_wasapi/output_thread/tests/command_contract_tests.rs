//! Command contract tests.
//!
//! Tests for command type variants, default implementations, and basic properties.

use crate::playback::output_wasapi::output_thread::command::{
    DrainCommand, ShutdownCommand, ThreadCommand,
};

#[test]
fn thread_command_variants_exist() {
    // Test that all expected variants exist.
    let shutdown = ThreadCommand::Shutdown(ShutdownCommand::Graceful);
    let drain = ThreadCommand::Drain(DrainCommand::UntilEmpty);
    let flush = ThreadCommand::Flush;
    let pause = ThreadCommand::Pause;
    let resume = ThreadCommand::Resume;

    assert_ne!(shutdown, drain);
    assert_ne!(drain, flush);
    assert_ne!(flush, pause);
    assert_ne!(pause, resume);
}

#[test]
fn shutdown_command_default_is_graceful() {
    let cmd = ShutdownCommand::default();
    assert_eq!(cmd, ShutdownCommand::Graceful);
}

#[test]
fn drain_command_default_is_until_empty() {
    let cmd = DrainCommand::default();
    assert_eq!(cmd, DrainCommand::UntilEmpty);
}

#[test]
fn thread_command_default_is_shutdown() {
    let cmd = ThreadCommand::default();
    assert_eq!(cmd, ThreadCommand::Shutdown(ShutdownCommand::Graceful));
}

#[test]
fn shutdown_command_clone() {
    let cmd = ShutdownCommand::Immediate;
    let cloned = cmd.clone();
    assert_eq!(cmd, cloned);
}

#[test]
fn drain_command_clone() {
    let cmd = DrainCommand::WithTimeout(1000);
    let cloned = cmd.clone();
    assert_eq!(cmd, cloned);
}

#[test]
fn thread_command_clone() {
    let cmd = ThreadCommand::SetVolume(50);
    let cloned = cmd.clone();
    assert_eq!(cmd, cloned);
}

#[test]
fn shutdown_command_debug() {
    let cmd = ShutdownCommand::WithTimeout(5000);
    let debug = format!("{:?}", cmd);
    assert!(debug.contains("WithTimeout"));
}

#[test]
fn drain_command_debug() {
    let cmd = DrainCommand::FrameCount(1024);
    let debug = format!("{:?}", cmd);
    assert!(debug.contains("FrameCount"));
}

#[test]
fn thread_command_debug() {
    let cmd = ThreadCommand::Flush;
    let debug = format!("{:?}", cmd);
    assert!(debug.contains("Flush"));
}

#[test]
fn shutdown_command_equality() {
    let a = ShutdownCommand::Graceful;
    let b = ShutdownCommand::Graceful;
    let c = ShutdownCommand::Immediate;
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn drain_command_equality() {
    let a = DrainCommand::UntilEmpty;
    let b = DrainCommand::UntilEmpty;
    let c = DrainCommand::WithTimeout(100);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn thread_command_equality() {
    let a = ThreadCommand::Pause;
    let b = ThreadCommand::Pause;
    let c = ThreadCommand::Resume;
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn shutdown_command_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a = ShutdownCommand::Graceful;
    let b = ShutdownCommand::Graceful;

    let mut hasher_a = DefaultHasher::new();
    let mut hasher_b = DefaultHasher::new();

    a.hash(&mut hasher_a);
    b.hash(&mut hasher_b);

    assert_eq!(hasher_a.finish(), hasher_b.finish());
}

#[test]
fn drain_command_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a = DrainCommand::UntilEmpty;
    let b = DrainCommand::UntilEmpty;

    let mut hasher_a = DefaultHasher::new();
    let mut hasher_b = DefaultHasher::new();

    a.hash(&mut hasher_a);
    b.hash(&mut hasher_b);

    assert_eq!(hasher_a.finish(), hasher_b.finish());
}

#[test]
fn thread_command_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a = ThreadCommand::Flush;
    let b = ThreadCommand::Flush;

    let mut hasher_a = DefaultHasher::new();
    let mut hasher_b = DefaultHasher::new();

    a.hash(&mut hasher_a);
    b.hash(&mut hasher_b);

    assert_eq!(hasher_a.finish(), hasher_b.finish());
}

#[test]
fn shutdown_command_copy() {
    let a = ShutdownCommand::WithTimeout(2000);
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn drain_command_copy() {
    let a = DrainCommand::FrameCount(512);
    let b = a;
    assert_eq!(a, b);
}
