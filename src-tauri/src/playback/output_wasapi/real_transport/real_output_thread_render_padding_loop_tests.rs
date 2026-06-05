//! Tests for optional padding-aware bounded silent render loop.

use super::render_padding_loop::{
    MAX_RENDER_PADDING_LOOP_FRAMES_PER_WRITE, MAX_RENDER_PADDING_LOOP_ITERATIONS,
};
use super::thread::{
    shutdown_and_join_real_output_thread, spawn_real_output_thread, RealOutputThreadSpawnConfig,
};
use super::thread_error::RealOutputThreadSkeletonError;

fn padding_config(
    open: bool,
    start: bool,
    enabled: bool,
    iterations: u32,
    frames: u32,
) -> RealOutputThreadSpawnConfig {
    RealOutputThreadSpawnConfig {
        max_steps: 10,
        open_wasapi_context_on_start: open,
        start_audio_client_on_start: start,
        render_padding_loop_after_start: enabled,
        render_padding_loop_iterations: iterations,
        render_padding_loop_max_frames_per_write: frames,
        ..Default::default()
    }
}

fn join_error(config: RealOutputThreadSpawnConfig) -> RealOutputThreadSkeletonError {
    let mut handle = spawn_real_output_thread(config).unwrap();
    let join_handle = handle.take_join_handle().unwrap();
    join_handle.join().unwrap().unwrap_err()
}

#[test]
fn render_padding_loop_flags_false_by_default() {
    let handle = spawn_real_output_thread(padding_config(false, false, false, 0, 0)).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();

    assert!(!report.render_padding_loop_requested);
    assert!(!report.render_padding_loop_started);
    assert!(!report.render_padding_loop_completed);
    assert_eq!(report.render_padding_loop_iterations_requested, 0);
    assert_eq!(report.render_padding_loop_iterations_completed, 0);
    assert_eq!(
        report.render_padding_loop_iterations_skipped_no_available,
        0
    );
    assert_eq!(report.render_padding_loop_max_frames_per_write, 0);
    assert_eq!(report.render_padding_loop_frames_written_total, 0);
    assert_eq!(report.render_padding_loop_last_capacity, 0);
    assert_eq!(report.render_padding_loop_last_padding, 0);
    assert_eq!(report.render_padding_loop_last_available, 0);
    assert!(!report.render_padding_loop_used_silent_flag);
}

#[test]
fn render_padding_loop_requires_context_open() {
    assert_eq!(
        join_error(padding_config(false, false, true, 1, 1)),
        RealOutputThreadSkeletonError::RenderPaddingLoopRequiresOpenContext
    );
}

#[test]
fn render_padding_loop_requires_started_client() {
    assert_eq!(
        join_error(padding_config(true, false, true, 1, 1)),
        RealOutputThreadSkeletonError::RenderPaddingLoopRequiresStartedClient
    );
}

#[test]
fn render_padding_loop_rejects_zero_iterations_without_device() {
    assert_eq!(
        join_error(padding_config(true, true, true, 0, 1)),
        RealOutputThreadSkeletonError::RenderPaddingLoopInvalidIterationCount
    );
}

#[test]
fn render_padding_loop_rejects_zero_frames_without_device() {
    assert_eq!(
        join_error(padding_config(true, true, true, 1, 0)),
        RealOutputThreadSkeletonError::RenderPaddingLoopInvalidFrameCount
    );
}

#[test]
fn render_padding_loop_rejects_too_many_iterations_without_device() {
    assert_eq!(
        join_error(padding_config(
            true,
            true,
            true,
            MAX_RENDER_PADDING_LOOP_ITERATIONS + 1,
            1,
        )),
        RealOutputThreadSkeletonError::RenderPaddingLoopIterationCountTooLarge
    );
}

#[test]
fn render_padding_loop_rejects_too_many_frames_without_device() {
    assert_eq!(
        join_error(padding_config(
            true,
            true,
            true,
            1,
            MAX_RENDER_PADDING_LOOP_FRAMES_PER_WRITE + 1,
        )),
        RealOutputThreadSkeletonError::RenderPaddingLoopFrameCountTooLarge
    );
}

#[cfg(not(target_os = "windows"))]
#[test]
fn render_padding_loop_true_non_windows_returns_context_open_error() {
    match join_error(padding_config(true, true, true, 1, 64)) {
        RealOutputThreadSkeletonError::WasapiContextOpen(_) => {}
        other => panic!("expected WasapiContextOpen, got: {:?}", other),
    }
}

#[cfg(target_os = "windows")]
#[ignore = "requires a real Windows audio endpoint"]
#[test]
fn windows_render_padding_loop_smoke_ignored() {
    let handle = spawn_real_output_thread(padding_config(true, true, true, 3, 128)).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();

    assert!(report.wasapi_context_open_requested);
    assert!(report.wasapi_context_opened);
    assert!(report.audio_client_start_requested);
    assert!(report.audio_client_started);
    assert!(report.render_padding_loop_requested);
    assert!(report.render_padding_loop_started);
    assert!(report.render_padding_loop_completed);
    assert_eq!(report.render_padding_loop_iterations_requested, 3);
    assert_eq!(report.render_padding_loop_iterations_completed, 3);
    assert!(report.render_padding_loop_iterations_skipped_no_available <= 3);
    assert!(report.render_padding_loop_frames_written_total <= 384);
    assert!(report.render_padding_loop_last_capacity > 0);
    assert!(report.render_padding_loop_last_padding <= report.render_padding_loop_last_capacity);
    assert!(report.render_padding_loop_last_available <= report.render_padding_loop_last_capacity);
    if report.render_padding_loop_frames_written_total > 0 {
        assert!(report.render_padding_loop_used_silent_flag);
    } else {
        assert!(!report.render_padding_loop_used_silent_flag);
        assert_eq!(
            report.render_padding_loop_iterations_skipped_no_available,
            3
        );
    }
    assert!(report.audio_client_stop_requested);
    assert!(report.audio_client_stopped);
    assert!(report.wasapi_context_closed);
    assert!(report.exited_cleanly);
}
