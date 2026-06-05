//! Tests for optional IAudioClient Start/Stop lifecycle in real output thread.

use super::thread::{
    shutdown_and_join_real_output_thread, spawn_real_output_thread, RealOutputThreadSpawnConfig,
};
use super::thread_error::RealOutputThreadSkeletonError;

fn config(
    open_wasapi_context_on_start: bool,
    start_audio_client_on_start: bool,
) -> RealOutputThreadSpawnConfig {
    RealOutputThreadSpawnConfig {
        max_steps: 10,
        open_wasapi_context_on_start,
        start_audio_client_on_start,
    }
}

#[test]
fn start_flags_false_by_default() {
    let handle = spawn_real_output_thread(config(false, false)).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();

    assert!(!report.audio_client_start_requested);
    assert!(!report.audio_client_started);
    assert!(!report.audio_client_stop_requested);
    assert!(!report.audio_client_stopped);
}

#[test]
fn start_requires_context_open() {
    let mut handle = spawn_real_output_thread(config(false, true)).unwrap();
    let join_handle = handle.take_join_handle().unwrap();
    let result = join_handle.join().unwrap();

    assert_eq!(
        result.unwrap_err(),
        RealOutputThreadSkeletonError::AudioClientStartRequiresOpenContext
    );
}

#[cfg(not(target_os = "windows"))]
#[test]
fn start_true_non_windows_returns_context_open_or_start_error() {
    let mut handle = spawn_real_output_thread(config(true, true)).unwrap();
    let join_handle = handle.take_join_handle().unwrap();
    let result = join_handle.join().unwrap();

    match result.unwrap_err() {
        RealOutputThreadSkeletonError::WasapiContextOpen(_)
        | RealOutputThreadSkeletonError::AudioClientStartFailed(_) => {}
        other => panic!("expected context open or start error, got: {:?}", other),
    }
}

#[cfg(target_os = "windows")]
#[ignore = "requires a real Windows audio endpoint"]
#[test]
fn windows_start_stop_smoke_ignored() {
    let handle = spawn_real_output_thread(config(true, true)).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();

    assert!(report.wasapi_context_opened);
    assert!(report.audio_client_start_requested);
    assert!(report.audio_client_started);
    assert!(report.audio_client_stop_requested);
    assert!(report.audio_client_stopped);
    assert!(report.wasapi_context_closed);
    assert!(report.exited_cleanly);
}
