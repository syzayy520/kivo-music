use super::super::render_ring_buffer_boundary::RenderRingBufferBoundaryOutcome;
use super::super::thread::{
    shutdown_and_join_real_output_thread, spawn_real_output_thread, RealOutputThreadSpawnConfig,
};
use super::super::thread_stages::build_real_output_thread_report;
use super::fixtures::{lifecycle_fields, render_once_disabled, resolved_with_boundary};

#[test]
fn default_disabled_boundary_report_fields_are_false_or_zero() {
    let handle = spawn_real_output_thread(RealOutputThreadSpawnConfig::default()).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();

    assert!(!report.render_ring_buffer_boundary_requested);
    assert!(!report.render_ring_buffer_boundary_started);
    assert!(!report.render_ring_buffer_boundary_completed);
    assert!(!report.render_ring_buffer_boundary_skipped_after_prior_failure);
    assert_eq!(report.render_ring_buffer_boundary_iterations_requested, 0);
    assert_eq!(report.render_ring_buffer_boundary_iterations_completed, 0);
    assert_eq!(report.render_ring_buffer_boundary_write_attempts, 0);
    assert_eq!(report.render_ring_buffer_boundary_frames_written_total, 0);
    assert_eq!(report.render_ring_buffer_boundary_frames_committed_total, 0);
    assert_eq!(report.render_ring_buffer_boundary_source_seeded_frames, 0);
    assert_eq!(
        report.render_ring_buffer_boundary_source_remaining_frames,
        0
    );
    assert!(!report.render_ring_buffer_boundary_used_silent_flag);
}

#[test]
fn compact_boundary_fields_are_mapped_from_success_outcome() {
    let outcome = RenderRingBufferBoundaryOutcome {
        requested: true,
        started: true,
        completed: true,
        skipped_after_prior_failure: false,
        iterations_requested: 2,
        iterations_completed: 2,
        synthetic_zero_seed_frames: 6,
        write_attempts: 2,
        frames_written_total: 5,
        frames_committed_total: 5,
        source_remaining_frames: 1,
        used_silent_flag: false,
        ..Default::default()
    };
    let report = build_real_output_thread_report(
        lifecycle_fields(),
        render_once_disabled(),
        resolved_with_boundary(outcome),
    );

    assert!(report.render_ring_buffer_boundary_requested);
    assert!(report.render_ring_buffer_boundary_started);
    assert!(report.render_ring_buffer_boundary_completed);
    assert_eq!(report.render_ring_buffer_boundary_iterations_requested, 2);
    assert_eq!(report.render_ring_buffer_boundary_iterations_completed, 2);
    assert_eq!(report.render_ring_buffer_boundary_write_attempts, 2);
    assert_eq!(report.render_ring_buffer_boundary_frames_written_total, 5);
    assert_eq!(report.render_ring_buffer_boundary_frames_committed_total, 5);
    assert_eq!(report.render_ring_buffer_boundary_source_seeded_frames, 6);
    assert_eq!(
        report.render_ring_buffer_boundary_source_remaining_frames,
        1
    );
}
