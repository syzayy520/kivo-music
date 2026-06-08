//! Contract tests for output thread state types.
//!
//! These tests verify the contract of all state types:
//! - State transitions are valid
//! - Stats counters are monotonic
//! - Serialization/deserialization round-trips
//! - Default values are correct

use crate::playback::output_wasapi::output_thread::state::{
    BufferConsumptionState, BufferConsumptionStats, FailureStats, FlushBarrierState,
    FlushBarrierStats, OutputThreadFailureKind, OutputThreadLifecycle, RenderActivity,
    RenderActivityStats,
};

// ============================================================================
// OutputThreadLifecycle tests
// ============================================================================

#[test]
fn thread_lifecycle_all_variants_exist() {
    let _ = OutputThreadLifecycle::NotStarted;
    let _ = OutputThreadLifecycle::Starting;
    let _ = OutputThreadLifecycle::Running;
    let _ = OutputThreadLifecycle::Draining;
    let _ = OutputThreadLifecycle::Stopping;
    let _ = OutputThreadLifecycle::Stopped;
    let _ = OutputThreadLifecycle::Failed;
}

#[test]
fn thread_lifecycle_variant_count() {
    let variants = [
        OutputThreadLifecycle::NotStarted,
        OutputThreadLifecycle::Starting,
        OutputThreadLifecycle::Running,
        OutputThreadLifecycle::Draining,
        OutputThreadLifecycle::Stopping,
        OutputThreadLifecycle::Stopped,
        OutputThreadLifecycle::Failed,
    ];
    assert_eq!(variants.len(), 7);
}

#[test]
fn thread_lifecycle_transitions_from_not_started() {
    let state = OutputThreadLifecycle::NotStarted;
    assert!(!state.is_active());
    assert!(!state.is_terminal());
    assert!(!state.is_transitional());
    assert!(!state.can_accept_frames());
    assert!(!state.is_draining());
}

#[test]
fn thread_lifecycle_transitions_from_starting() {
    let state = OutputThreadLifecycle::Starting;
    assert!(state.is_active());
    assert!(!state.is_terminal());
    assert!(state.is_transitional());
    assert!(!state.can_accept_frames());
    assert!(!state.is_draining());
}

#[test]
fn thread_lifecycle_transitions_from_running() {
    let state = OutputThreadLifecycle::Running;
    assert!(state.is_active());
    assert!(!state.is_terminal());
    assert!(!state.is_transitional());
    assert!(state.can_accept_frames());
    assert!(!state.is_draining());
}

#[test]
fn thread_lifecycle_transitions_from_draining() {
    let state = OutputThreadLifecycle::Draining;
    assert!(state.is_active());
    assert!(!state.is_terminal());
    assert!(!state.is_transitional());
    assert!(!state.can_accept_frames());
    assert!(state.is_draining());
}

#[test]
fn thread_lifecycle_transitions_from_stopping() {
    let state = OutputThreadLifecycle::Stopping;
    assert!(!state.is_active());
    assert!(!state.is_terminal());
    assert!(state.is_transitional());
    assert!(!state.can_accept_frames());
    assert!(!state.is_draining());
}

#[test]
fn thread_lifecycle_transitions_from_stopped() {
    let state = OutputThreadLifecycle::Stopped;
    assert!(!state.is_active());
    assert!(state.is_terminal());
    assert!(!state.is_transitional());
    assert!(!state.can_accept_frames());
    assert!(!state.is_draining());
}

#[test]
fn thread_lifecycle_transitions_from_failed() {
    let state = OutputThreadLifecycle::Failed;
    assert!(!state.is_active());
    assert!(state.is_terminal());
    assert!(!state.is_transitional());
    assert!(!state.can_accept_frames());
    assert!(!state.is_draining());
}

#[test]
fn thread_lifecycle_equality() {
    assert_eq!(
        OutputThreadLifecycle::Running,
        OutputThreadLifecycle::Running
    );
    assert_eq!(OutputThreadLifecycle::Failed, OutputThreadLifecycle::Failed);
    assert_ne!(
        OutputThreadLifecycle::Running,
        OutputThreadLifecycle::Stopped
    );
    assert_ne!(
        OutputThreadLifecycle::Starting,
        OutputThreadLifecycle::Failed
    );
}

#[test]
fn thread_lifecycle_clone() {
    let state = OutputThreadLifecycle::Running;
    let cloned = state;
    assert_eq!(state, cloned);
}

#[test]
fn thread_lifecycle_debug() {
    let state = OutputThreadLifecycle::Running;
    let debug = format!("{:?}", state);
    assert_eq!(debug, "Running");
}

// ============================================================================
// RenderActivity tests
// ============================================================================

#[test]
fn render_activity_all_variants_exist() {
    let _ = RenderActivity::Idle;
    let _ = RenderActivity::Rendering;
    let _ = RenderActivity::Silenced;
    let _ = RenderActivity::Paused;
    let _ = RenderActivity::Error;
}

#[test]
fn render_activity_stats_counters_are_monotonic() {
    let mut stats = RenderActivityStats::new();
    assert_eq!(stats.frames_submitted, 0);
    assert_eq!(stats.frames_rendered, 0);
    assert_eq!(stats.bytes_rendered, 0);

    stats.record_submit(10);
    stats.record_render(5, 2048);
    assert_eq!(stats.frames_submitted, 10);
    assert_eq!(stats.frames_rendered, 5);
    assert_eq!(stats.bytes_rendered, 2048);

    stats.record_submit(5);
    stats.record_render(3, 1024);
    assert_eq!(stats.frames_submitted, 15);
    assert_eq!(stats.frames_rendered, 8);
    assert_eq!(stats.bytes_rendered, 3072);
}

#[test]
fn render_activity_stats_pending_frames_contract() {
    let mut stats = RenderActivityStats::new();
    assert_eq!(stats.pending_frames(), 0);

    stats.record_submit(10);
    assert_eq!(stats.pending_frames(), 10);

    stats.record_render(3, 1024);
    assert_eq!(stats.pending_frames(), 7);

    stats.record_submit(5);
    assert_eq!(stats.pending_frames(), 12);
}

#[test]
fn render_activity_stats_reset_contract() {
    let mut stats = RenderActivityStats::new();
    stats.record_submit(10);
    stats.record_render(5, 2048);
    stats.reset();
    assert_eq!(stats.frames_submitted, 0);
    assert_eq!(stats.frames_rendered, 0);
    assert_eq!(stats.bytes_rendered, 0);
}

// ============================================================================
// BufferConsumption tests
// ============================================================================

#[test]
fn buffer_consumption_all_variants_exist() {
    let _ = BufferConsumptionState::Empty;
    let _ = BufferConsumptionState::Available;
    let _ = BufferConsumptionState::Full;
    let _ = BufferConsumptionState::Underrun;
    let _ = BufferConsumptionState::Overrun;
}

#[test]
fn buffer_consumption_stats_counters_are_monotonic() {
    let mut stats = BufferConsumptionStats::new();
    assert_eq!(stats.silence_frames_written, 0);
    assert_eq!(stats.underrun_count, 0);
    assert_eq!(stats.overrun_count, 0);

    stats.record_silence(50);
    stats.record_underrun();
    stats.record_overrun();
    assert_eq!(stats.silence_frames_written, 50);
    assert_eq!(stats.underrun_count, 1);
    assert_eq!(stats.overrun_count, 1);

    stats.record_silence(25);
    stats.record_underrun();
    stats.record_overrun();
    assert_eq!(stats.silence_frames_written, 75);
    assert_eq!(stats.underrun_count, 2);
    assert_eq!(stats.overrun_count, 2);
}

#[test]
fn buffer_consumption_stats_lost_frames_contract() {
    let mut stats = BufferConsumptionStats::new();
    assert_eq!(stats.lost_frames(), 0);

    stats.record_silence(100);
    assert_eq!(stats.lost_frames(), 100);

    stats.record_silence(50);
    assert_eq!(stats.lost_frames(), 150);
}

#[test]
fn buffer_consumption_stats_reset_contract() {
    let mut stats = BufferConsumptionStats::new();
    stats.record_silence(50);
    stats.record_underrun();
    stats.record_overrun();
    stats.reset();
    assert_eq!(stats.silence_frames_written, 0);
    assert_eq!(stats.underrun_count, 0);
    assert_eq!(stats.overrun_count, 0);
}

// ============================================================================
// FlushBarrier tests
// ============================================================================

#[test]
fn flush_barrier_all_variants_exist() {
    let _ = FlushBarrierState::NotFlushing;
    let _ = FlushBarrierState::Requested;
    let _ = FlushBarrierState::Flushing;
    let _ = FlushBarrierState::Complete;
    let _ = FlushBarrierState::Error;
}

#[test]
fn flush_barrier_stats_counters_are_monotonic() {
    let mut stats = FlushBarrierStats::new();
    assert_eq!(stats.flush_count, 0);
    assert_eq!(stats.flush_errors, 0);

    stats.record_flush();
    stats.record_flush_error();
    assert_eq!(stats.flush_count, 1);
    assert_eq!(stats.flush_errors, 1);

    stats.record_flush();
    assert_eq!(stats.flush_count, 2);
    assert_eq!(stats.flush_errors, 1);
}

#[test]
fn flush_barrier_stats_reset_contract() {
    let mut stats = FlushBarrierStats::new();
    stats.record_flush();
    stats.record_flush_error();
    stats.reset();
    assert_eq!(stats.flush_count, 0);
    assert_eq!(stats.flush_errors, 0);
}

// ============================================================================
// OutputThreadFailureKind tests
// ============================================================================

#[test]
fn failure_kind_all_variants_exist() {
    let _ = OutputThreadFailureKind::SpawnFailed;
    let _ = OutputThreadFailureKind::RuntimeError;
    let _ = OutputThreadFailureKind::DrainError;
    let _ = OutputThreadFailureKind::StopError;
    let _ = OutputThreadFailureKind::Timeout;
    let _ = OutputThreadFailureKind::Unknown;
}

#[test]
fn failure_stats_counters_are_monotonic() {
    let mut stats = FailureStats::new();
    assert_eq!(stats.error_count, 0);
    assert_eq!(stats.last_failure, None);

    stats.record_error(OutputThreadFailureKind::SpawnFailed);
    assert_eq!(stats.error_count, 1);
    assert_eq!(
        stats.last_failure,
        Some(OutputThreadFailureKind::SpawnFailed)
    );

    stats.record_unknown_error();
    assert_eq!(stats.error_count, 2);
    assert_eq!(stats.last_failure, Some(OutputThreadFailureKind::Unknown));
}

#[test]
fn failure_stats_reset_contract() {
    let mut stats = FailureStats::new();
    stats.record_error(OutputThreadFailureKind::RuntimeError);
    stats.reset();
    assert_eq!(stats.error_count, 0);
    assert_eq!(stats.last_failure, None);
}

// ============================================================================
// Cross-type serialization round-trip tests
// ============================================================================

#[test]
fn thread_lifecycle_serialization_round_trip() {
    let state = OutputThreadLifecycle::Running;
    let json = serde_json::to_string(&state).unwrap();
    let deserialized: OutputThreadLifecycle = serde_json::from_str(&json).unwrap();
    assert_eq!(state, deserialized);
}

#[test]
fn render_activity_serialization_round_trip() {
    let state = RenderActivity::Rendering;
    let json = serde_json::to_string(&state).unwrap();
    let deserialized: RenderActivity = serde_json::from_str(&json).unwrap();
    assert_eq!(state, deserialized);
}

#[test]
fn buffer_consumption_serialization_round_trip() {
    let state = BufferConsumptionState::Underrun;
    let json = serde_json::to_string(&state).unwrap();
    let deserialized: BufferConsumptionState = serde_json::from_str(&json).unwrap();
    assert_eq!(state, deserialized);
}

#[test]
fn flush_barrier_serialization_round_trip() {
    let state = FlushBarrierState::Flushing;
    let json = serde_json::to_string(&state).unwrap();
    let deserialized: FlushBarrierState = serde_json::from_str(&json).unwrap();
    assert_eq!(state, deserialized);
}

#[test]
fn failure_kind_serialization_round_trip() {
    let kind = OutputThreadFailureKind::RuntimeError;
    let json = serde_json::to_string(&kind).unwrap();
    let deserialized: OutputThreadFailureKind = serde_json::from_str(&json).unwrap();
    assert_eq!(kind, deserialized);
}

#[test]
fn render_activity_stats_serialization_round_trip() {
    let mut stats = RenderActivityStats::new();
    stats.record_submit(10);
    stats.record_render(5, 2048);
    let json = serde_json::to_string(&stats).unwrap();
    let deserialized: RenderActivityStats = serde_json::from_str(&json).unwrap();
    assert_eq!(stats, deserialized);
}

#[test]
fn buffer_consumption_stats_serialization_round_trip() {
    let mut stats = BufferConsumptionStats::new();
    stats.record_silence(50);
    stats.record_underrun();
    let json = serde_json::to_string(&stats).unwrap();
    let deserialized: BufferConsumptionStats = serde_json::from_str(&json).unwrap();
    assert_eq!(stats, deserialized);
}

#[test]
fn flush_barrier_stats_serialization_round_trip() {
    let mut stats = FlushBarrierStats::new();
    stats.record_flush();
    let json = serde_json::to_string(&stats).unwrap();
    let deserialized: FlushBarrierStats = serde_json::from_str(&json).unwrap();
    assert_eq!(stats, deserialized);
}

#[test]
fn failure_stats_serialization_round_trip() {
    let mut stats = FailureStats::new();
    stats.record_error(OutputThreadFailureKind::DrainError);
    let json = serde_json::to_string(&stats).unwrap();
    let deserialized: FailureStats = serde_json::from_str(&json).unwrap();
    assert_eq!(stats, deserialized);
}
