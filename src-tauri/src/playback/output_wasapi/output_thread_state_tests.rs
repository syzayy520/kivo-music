use super::output_thread_state::{OutputThreadReport, OutputThreadState, OutputThreadStats};

// ---------------------------------------------------------------------------
// OutputThreadState default and equality
// ---------------------------------------------------------------------------

#[test]
fn default_state_is_created() {
    let state = OutputThreadState::default();
    assert_eq!(state, OutputThreadState::Created);
}

#[test]
fn state_equality_works() {
    assert_eq!(OutputThreadState::Created, OutputThreadState::Created);
    assert_ne!(OutputThreadState::Created, OutputThreadState::Running);
}

#[test]
fn state_copy_and_clone() {
    let a = OutputThreadState::Running;
    let b = a;
    let c = a.clone();
    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn all_seven_variants_exist() {
    let variants = [
        OutputThreadState::Created,
        OutputThreadState::Running,
        OutputThreadState::Stopping,
        OutputThreadState::Stopped,
        OutputThreadState::Closed,
        OutputThreadState::Joined,
        OutputThreadState::Failed,
    ];
    assert_eq!(variants.len(), 7);
}

// ---------------------------------------------------------------------------
// OutputThreadStats default values
// ---------------------------------------------------------------------------

#[test]
fn stats_default_all_counters_zero() {
    let stats = OutputThreadStats::default();
    assert_eq!(stats.consumed_frames, 0);
    assert_eq!(stats.rendered_frames, 0);
    assert_eq!(stats.silence_filled_frames, 0);
    assert_eq!(stats.dropped_frames, 0);
    assert_eq!(stats.render_error_count, 0);
    assert_eq!(stats.device_lost_count, 0);
}

#[test]
fn stats_default_errors_none() {
    let stats = OutputThreadStats::default();
    assert!(stats.last_output_thread_error.is_none());
    assert!(stats.last_render_error.is_none());
}

#[test]
fn stats_default_state_is_created() {
    let stats = OutputThreadStats::default();
    assert_eq!(stats.output_thread_state, OutputThreadState::Created);
}

#[test]
fn stats_mutation() {
    let mut stats = OutputThreadStats::default();
    stats.consumed_frames = 1024;
    stats.rendered_frames = 960;
    stats.silence_filled_frames = 64;
    stats.dropped_frames = 2;
    stats.render_error_count = 1;
    stats.device_lost_count = 0;
    stats.last_output_thread_error = Some("mock error".to_string());
    stats.output_thread_state = OutputThreadState::Running;

    assert_eq!(stats.consumed_frames, 1024);
    assert_eq!(stats.rendered_frames, 960);
    assert_eq!(stats.silence_filled_frames, 64);
    assert_eq!(stats.dropped_frames, 2);
    assert_eq!(stats.render_error_count, 1);
    assert_eq!(stats.device_lost_count, 0);
    assert_eq!(
        stats.last_output_thread_error.as_deref(),
        Some("mock error")
    );
    assert_eq!(stats.output_thread_state, OutputThreadState::Running);
}

// ---------------------------------------------------------------------------
// OutputThreadReport default values
// ---------------------------------------------------------------------------

#[test]
fn report_default_no_error() {
    let report = OutputThreadReport::default();
    assert!(report.error.is_none());
}

#[test]
fn report_default_not_panicked() {
    let report = OutputThreadReport::default();
    assert!(!report.panicked);
}

#[test]
fn report_default_stats_are_default() {
    let report = OutputThreadReport::default();
    assert_eq!(report.stats.consumed_frames, 0);
    assert_eq!(report.stats.rendered_frames, 0);
    assert_eq!(report.stats.output_thread_state, OutputThreadState::Created);
}

#[test]
fn report_with_error() {
    let mut stats = OutputThreadStats::default();
    stats.output_thread_state = OutputThreadState::Failed;
    let report = OutputThreadReport {
        stats,
        error: Some("device lost".to_string()),
        panicked: false,
    };
    assert_eq!(report.stats.output_thread_state, OutputThreadState::Failed);
    assert_eq!(report.error.as_deref(), Some("device lost"));
    assert!(!report.panicked);
}

#[test]
fn report_with_panic() {
    let mut stats = OutputThreadStats::default();
    stats.output_thread_state = OutputThreadState::Failed;
    let report = OutputThreadReport {
        stats,
        error: Some("thread panicked".to_string()),
        panicked: true,
    };
    assert!(report.panicked);
    assert_eq!(report.stats.output_thread_state, OutputThreadState::Failed);
}
