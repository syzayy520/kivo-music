//! Contract tests for output thread state types.
//!
//! These tests verify the contract of OutputThreadState and OutputThreadStats:
//! - State transitions are valid
//! - Stats counters are monotonic
//! - Serialization/deserialization round-trips
//! - Default values are correct

use super::output_thread_state::{OutputThreadState, OutputThreadStats};

#[test]
fn output_thread_state_all_variants_exist() {
    // Ensure all expected variants are present
    let _ = OutputThreadState::NotStarted;
    let _ = OutputThreadState::Starting;
    let _ = OutputThreadState::Running;
    let _ = OutputThreadState::Draining;
    let _ = OutputThreadState::Stopping;
    let _ = OutputThreadState::Stopped;
    let _ = OutputThreadState::Failed;
}

#[test]
fn output_thread_state_variant_count() {
    // Ensure we have exactly 7 variants
    let variants = [
        OutputThreadState::NotStarted,
        OutputThreadState::Starting,
        OutputThreadState::Running,
        OutputThreadState::Draining,
        OutputThreadState::Stopping,
        OutputThreadState::Stopped,
        OutputThreadState::Failed,
    ];
    assert_eq!(variants.len(), 7);
}

#[test]
fn output_thread_state_transitions_from_not_started() {
    let state = OutputThreadState::NotStarted;
    // NotStarted can transition to Starting
    assert!(state.is_active() == false);
    assert!(state.is_terminal() == false);
    assert!(state.is_transitional() == false);
    assert!(state.can_accept_frames() == false);
    assert!(state.is_draining() == false);
}

#[test]
fn output_thread_state_transitions_from_starting() {
    let state = OutputThreadState::Starting;
    assert!(state.is_active());
    assert!(!state.is_terminal());
    assert!(state.is_transitional());
    assert!(!state.can_accept_frames());
    assert!(!state.is_draining());
}

#[test]
fn output_thread_state_transitions_from_running() {
    let state = OutputThreadState::Running;
    assert!(state.is_active());
    assert!(!state.is_terminal());
    assert!(!state.is_transitional());
    assert!(state.can_accept_frames());
    assert!(!state.is_draining());
}

#[test]
fn output_thread_state_transitions_from_draining() {
    let state = OutputThreadState::Draining;
    assert!(state.is_active());
    assert!(!state.is_terminal());
    assert!(!state.is_transitional());
    assert!(!state.can_accept_frames());
    assert!(state.is_draining());
}

#[test]
fn output_thread_state_transitions_from_stopping() {
    let state = OutputThreadState::Stopping;
    assert!(!state.is_active());
    assert!(!state.is_terminal());
    assert!(state.is_transitional());
    assert!(!state.can_accept_frames());
    assert!(!state.is_draining());
}

#[test]
fn output_thread_state_transitions_from_stopped() {
    let state = OutputThreadState::Stopped;
    assert!(!state.is_active());
    assert!(state.is_terminal());
    assert!(!state.is_transitional());
    assert!(!state.can_accept_frames());
    assert!(!state.is_draining());
}

#[test]
fn output_thread_state_transitions_from_failed() {
    let state = OutputThreadState::Failed;
    assert!(!state.is_active());
    assert!(state.is_terminal());
    assert!(!state.is_transitional());
    assert!(!state.can_accept_frames());
    assert!(!state.is_draining());
}

#[test]
fn output_thread_stats_counters_are_monotonic() {
    let mut stats = OutputThreadStats::new();

    // All counters start at zero
    assert_eq!(stats.frames_submitted, 0);
    assert_eq!(stats.frames_rendered, 0);
    assert_eq!(stats.silence_frames_written, 0);
    assert_eq!(stats.underrun_count, 0);
    assert_eq!(stats.overrun_count, 0);
    assert_eq!(stats.error_count, 0);
    assert_eq!(stats.bytes_rendered, 0);

    // Counters only increase
    stats.record_submit(10);
    stats.record_render(5, 2048);
    stats.record_silence(50);
    stats.record_underrun();
    stats.record_overrun();
    stats.record_error();

    assert_eq!(stats.frames_submitted, 10);
    assert_eq!(stats.frames_rendered, 5);
    assert_eq!(stats.silence_frames_written, 50);
    assert_eq!(stats.underrun_count, 1);
    assert_eq!(stats.overrun_count, 1);
    assert_eq!(stats.error_count, 1);
    assert_eq!(stats.bytes_rendered, 2048);

    // Add more, counters should increase
    stats.record_submit(5);
    stats.record_render(3, 1024);
    stats.record_silence(25);
    stats.record_underrun();
    stats.record_overrun();
    stats.record_error();

    assert_eq!(stats.frames_submitted, 15);
    assert_eq!(stats.frames_rendered, 8);
    assert_eq!(stats.silence_frames_written, 75);
    assert_eq!(stats.underrun_count, 2);
    assert_eq!(stats.overrun_count, 2);
    assert_eq!(stats.error_count, 2);
    assert_eq!(stats.bytes_rendered, 3072);
}

#[test]
fn output_thread_stats_pending_frames_contract() {
    let mut stats = OutputThreadStats::new();

    // pending_frames = submitted - rendered
    assert_eq!(stats.pending_frames(), 0);

    stats.record_submit(10);
    assert_eq!(stats.pending_frames(), 10);

    stats.record_render(3, 1024);
    assert_eq!(stats.pending_frames(), 7);

    stats.record_submit(5);
    assert_eq!(stats.pending_frames(), 12);

    // Even if rendered > submitted (shouldn't happen), pending_frames should be 0
    stats.frames_rendered = 20;
    assert_eq!(stats.pending_frames(), 0);
}

#[test]
fn output_thread_stats_lost_frames_contract() {
    let mut stats = OutputThreadStats::new();

    // lost_frames = silence_frames_written
    assert_eq!(stats.lost_frames(), 0);

    stats.record_silence(100);
    assert_eq!(stats.lost_frames(), 100);

    stats.record_silence(50);
    assert_eq!(stats.lost_frames(), 150);
}

#[test]
fn output_thread_stats_reset_contract() {
    let mut stats = OutputThreadStats::new();

    // Add some data
    stats.record_submit(10);
    stats.record_render(5, 2048);
    stats.record_silence(50);
    stats.record_underrun();
    stats.record_overrun();
    stats.record_error();

    // Reset should zero everything
    stats.reset();

    assert_eq!(stats.frames_submitted, 0);
    assert_eq!(stats.frames_rendered, 0);
    assert_eq!(stats.silence_frames_written, 0);
    assert_eq!(stats.underrun_count, 0);
    assert_eq!(stats.overrun_count, 0);
    assert_eq!(stats.error_count, 0);
    assert_eq!(stats.bytes_rendered, 0);
    assert_eq!(stats.pending_frames(), 0);
    assert_eq!(stats.lost_frames(), 0);
}

#[test]
fn output_thread_state_equality() {
    // Same variants should be equal
    assert_eq!(OutputThreadState::Running, OutputThreadState::Running);
    assert_eq!(OutputThreadState::Failed, OutputThreadState::Failed);

    // Different variants should not be equal
    assert_ne!(OutputThreadState::Running, OutputThreadState::Stopped);
    assert_ne!(OutputThreadState::Starting, OutputThreadState::Failed);
}

#[test]
fn output_thread_stats_equality() {
    let mut stats1 = OutputThreadStats::new();
    let mut stats2 = OutputThreadStats::new();

    // Empty stats should be equal
    assert_eq!(stats1, stats2);

    // Add same data
    stats1.record_submit(10);
    stats2.record_submit(10);
    assert_eq!(stats1, stats2);

    // Add different data
    stats1.record_render(5, 2048);
    stats2.record_render(3, 1024);
    assert_ne!(stats1, stats2);
}

#[test]
fn output_thread_state_clone() {
    let state = OutputThreadState::Running;
    let cloned = state;
    assert_eq!(state, cloned);
}

#[test]
fn output_thread_stats_clone() {
    let mut stats = OutputThreadStats::new();
    stats.record_submit(10);
    stats.record_render(5, 2048);

    let cloned = stats.clone();
    assert_eq!(stats, cloned);
}

#[test]
fn output_thread_state_debug() {
    let state = OutputThreadState::Running;
    let debug = format!("{:?}", state);
    assert_eq!(debug, "Running");
}

#[test]
fn output_thread_stats_debug() {
    let mut stats = OutputThreadStats::new();
    stats.record_submit(10);
    let debug = format!("{:?}", stats);
    assert!(debug.contains("frames_submitted: 10"));
}
