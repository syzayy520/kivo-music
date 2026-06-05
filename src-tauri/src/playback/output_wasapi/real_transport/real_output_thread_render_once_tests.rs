//! Tests for optional one-shot silent render write in real output thread.

use super::thread::{
    shutdown_and_join_real_output_thread, spawn_real_output_thread, RealOutputThreadSpawnConfig,
};
use super::thread_error::RealOutputThreadSkeletonError;

fn config(
    open_wasapi_context_on_start: bool,
    start_audio_client_on_start: bool,
    render_silence_once_after_open: bool,
    render_silence_once_frames: u32,
) -> RealOutputThreadSpawnConfig {
    RealOutputThreadSpawnConfig {
        max_steps: 10,
        open_wasapi_context_on_start,
        start_audio_client_on_start,
        render_silence_once_after_open,
        render_silence_once_frames,
    }
}

#[test]
fn render_once_flags_false_by_default() {
    let handle = spawn_real_output_thread(config(false, false, false, 0)).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();

    assert!(!report.render_silence_once_requested);
    assert!(!report.render_silence_once_written);
    assert_eq!(report.render_silence_once_frames_requested, 0);
    assert_eq!(report.render_silence_once_frames_written, 0);
    assert!(!report.render_silence_once_used_silent_flag);
    assert!(!report.audio_client_start_requested);
    assert!(!report.audio_client_started);
}

#[test]
fn render_once_requires_context_open() {
    let mut handle = spawn_real_output_thread(config(false, false, true, 1)).unwrap();
    let join_handle = handle.take_join_handle().unwrap();
    let result = join_handle.join().unwrap();

    assert_eq!(
        result.unwrap_err(),
        RealOutputThreadSkeletonError::RenderSilenceOnceRequiresOpenContext
    );
}

#[test]
fn render_once_rejects_zero_frames_without_device() {
    let mut handle = spawn_real_output_thread(config(true, false, true, 0)).unwrap();
    let join_handle = handle.take_join_handle().unwrap();
    let result = join_handle.join().unwrap();

    assert_eq!(
        result.unwrap_err(),
        RealOutputThreadSkeletonError::RenderSilenceOnceInvalidFrameCount
    );
}

#[cfg(not(target_os = "windows"))]
#[test]
fn render_once_true_non_windows_returns_context_open_error() {
    let mut handle = spawn_real_output_thread(config(true, false, true, 64)).unwrap();
    let join_handle = handle.take_join_handle().unwrap();
    let result = join_handle.join().unwrap();

    match result.unwrap_err() {
        RealOutputThreadSkeletonError::WasapiContextOpen(_) => {}
        other => panic!("expected WasapiContextOpen, got: {:?}", other),
    }
}

#[cfg(target_os = "windows")]
#[ignore = "requires a real Windows audio endpoint"]
#[test]
fn windows_render_silence_once_smoke_ignored() {
    let handle = spawn_real_output_thread(config(true, true, true, 128)).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();

    assert!(report.wasapi_context_opened);
    assert!(report.render_silence_once_requested);
    assert!(report.render_silence_once_written);
    assert_eq!(report.render_silence_once_frames_requested, 128);
    assert_eq!(report.render_silence_once_frames_written, 128);
    assert!(report.render_silence_once_used_silent_flag);
    assert!(report.audio_client_start_requested);
    assert!(report.audio_client_started);
    assert!(report.audio_client_stop_requested);
    assert!(report.audio_client_stopped);
    assert!(report.wasapi_context_closed);
    assert!(report.exited_cleanly);
}
