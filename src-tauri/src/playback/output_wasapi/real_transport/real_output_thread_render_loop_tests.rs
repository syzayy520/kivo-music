//! Tests for optional bounded silent render loop in real output thread.

use super::render_loop::{
    MAX_RENDER_SILENCE_LOOP_FRAMES_PER_WRITE, MAX_RENDER_SILENCE_LOOP_ITERATIONS,
};
use super::thread::{
    shutdown_and_join_real_output_thread, spawn_real_output_thread, RealOutputThreadSpawnConfig,
};
use super::thread_error::RealOutputThreadSkeletonError;

fn loop_config(open: bool, start: bool, enabled: bool, iterations: u32, frames: u32) -> RealOutputThreadSpawnConfig {
    RealOutputThreadSpawnConfig {
        max_steps: 10,
        open_wasapi_context_on_start: open,
        start_audio_client_on_start: start,
        render_silence_loop_after_start: enabled,
        render_silence_loop_iterations: iterations,
        render_silence_loop_frames_per_write: frames,
        ..Default::default()
    }
}

fn join_error(config: RealOutputThreadSpawnConfig) -> RealOutputThreadSkeletonError {
    let mut handle = spawn_real_output_thread(config).unwrap();
    let join_handle = handle.take_join_handle().unwrap();
    join_handle.join().unwrap().unwrap_err()
}

#[test]
fn render_loop_flags_false_by_default() {
    let handle = spawn_real_output_thread(loop_config(false, false, false, 0, 0)).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();

    assert!(!report.render_silence_loop_requested);
    assert!(!report.render_silence_loop_started);
    assert!(!report.render_silence_loop_completed);
    assert_eq!(report.render_silence_loop_iterations_requested, 0);
    assert_eq!(report.render_silence_loop_iterations_completed, 0);
    assert_eq!(report.render_silence_loop_frames_per_write, 0);
    assert_eq!(report.render_silence_loop_frames_written_total, 0);
    assert!(!report.render_silence_loop_used_silent_flag);
    assert!(!report.render_silence_once_requested);
    assert!(!report.audio_client_start_requested);
    assert!(!report.audio_client_started);
}

#[test]
fn render_loop_requires_context_open() {
    assert_eq!(
        join_error(loop_config(false, false, true, 1, 1)),
        RealOutputThreadSkeletonError::RenderSilenceLoopRequiresOpenContext
    );
}

#[test]
fn render_loop_requires_started_client() {
    assert_eq!(
        join_error(loop_config(true, false, true, 1, 1)),
        RealOutputThreadSkeletonError::RenderSilenceLoopRequiresStartedClient
    );
}

#[test]
fn render_loop_rejects_zero_iterations_without_device() {
    assert_eq!(
        join_error(loop_config(true, true, true, 0, 1)),
        RealOutputThreadSkeletonError::RenderSilenceLoopInvalidIterationCount
    );
}

#[test]
fn render_loop_rejects_zero_frames_without_device() {
    assert_eq!(
        join_error(loop_config(true, true, true, 1, 0)),
        RealOutputThreadSkeletonError::RenderSilenceLoopInvalidFrameCount
    );
}

#[test]
fn render_loop_rejects_too_many_iterations_without_device() {
    assert_eq!(
        join_error(loop_config(
            true,
            true,
            true,
            MAX_RENDER_SILENCE_LOOP_ITERATIONS + 1,
            1,
        )),
        RealOutputThreadSkeletonError::RenderSilenceLoopIterationCountTooLarge
    );
}

#[test]
fn render_loop_rejects_too_many_frames_without_device() {
    assert_eq!(
        join_error(loop_config(
            true,
            true,
            true,
            1,
            MAX_RENDER_SILENCE_LOOP_FRAMES_PER_WRITE + 1,
        )),
        RealOutputThreadSkeletonError::RenderSilenceLoopFrameCountTooLarge
    );
}

#[cfg(not(target_os = "windows"))]
#[test]
fn render_loop_true_non_windows_returns_context_open_error() {
    match join_error(loop_config(true, true, true, 1, 64)) {
        RealOutputThreadSkeletonError::WasapiContextOpen(_) => {}
        other => panic!("expected WasapiContextOpen, got: {:?}", other),
    }
}

#[cfg(target_os = "windows")]
#[ignore = "requires a real Windows audio endpoint"]
#[test]
fn windows_render_silence_loop_smoke_ignored() {
    let handle = spawn_real_output_thread(loop_config(true, true, true, 3, 128)).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();

    assert!(report.wasapi_context_opened);
    assert!(!report.render_silence_once_requested);
    assert_eq!(report.render_silence_once_frames_written, 0);
    assert!(report.audio_client_start_requested);
    assert!(report.audio_client_started);
    assert!(report.render_silence_loop_requested);
    assert!(report.render_silence_loop_started);
    assert!(report.render_silence_loop_completed);
    assert_eq!(report.render_silence_loop_iterations_requested, 3);
    assert_eq!(report.render_silence_loop_iterations_completed, 3);
    assert_eq!(report.render_silence_loop_frames_per_write, 128);
    assert_eq!(report.render_silence_loop_frames_written_total, 384);
    assert!(report.render_silence_loop_used_silent_flag);
    assert!(report.audio_client_stop_requested);
    assert!(report.audio_client_stopped);
    assert!(report.wasapi_context_closed);
    assert!(report.exited_cleanly);
}
