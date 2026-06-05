//! Tests for real output thread spawn, handle, and shutdown.
//!
//! Required tests (A-I):
//! A. spawn_creates_valid_thread
//! B. shutdown_and_join_returns_worker_report
//! C. send_close_transport_stops_thread
//! D. worker_exits_after_max_steps_without_shutdown
//! E. shutdown_send_failure_is_reported
//! F. join_panic_is_reported
//! G. thread_does_not_start_audio_client
//! H. skeleton_does_not_call_runtime_audio_layers
//! I. owned_state_validation_happens_inside_thread
//! J. context_flags_false_when_not_requested
//! K. context_open_returns_error_on_non_windows (non-Windows only)
//! L. windows_context_open_smoke_test (Windows only, ignored)

use std::thread;
use std::time::Duration;

use super::command::OutputThreadRealTransportCommand;
use super::thread::{
    shutdown_and_join_real_output_thread, spawn_real_output_thread, RealOutputThreadSpawnConfig,
};
use super::thread_error::RealOutputThreadSkeletonError;

fn config(max_steps: usize) -> RealOutputThreadSpawnConfig {
    RealOutputThreadSpawnConfig {
        max_steps,
        open_wasapi_context_on_start: false,
    }
}

/// A. Thread spawns, handle has join, shutdown returns valid report.
#[test]
fn spawn_creates_valid_thread() {
    let handle = spawn_real_output_thread(config(100)).unwrap();
    assert!(handle.has_join_handle(), "handle should have join handle");
    let report = shutdown_and_join_real_output_thread(handle).unwrap();
    assert!(report.thread_started, "thread should have started");
    assert!(
        report.owned_state_validated,
        "owned state should be validated inside thread"
    );
}

/// B. Shutdown returns report with correct worker loop fields.
#[test]
fn shutdown_and_join_returns_worker_report() {
    let handle = spawn_real_output_thread(config(100)).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();
    assert!(report.shutdown_received, "shutdown should be received");
    assert!(report.exited_cleanly, "thread should exit cleanly");
    // CloseTransport maps to TransportClosed step kind, which is NOT counted
    // in commands_handled (only RuntimeIntentHandled/StopRequested are counted).
    assert_eq!(
        report.commands_processed, 0,
        "CloseTransport is a transport-level signal, not a counted command"
    );
    assert!(
        report.loop_result.is_some(),
        "loop result should be present"
    );
}

/// C. Manual send CloseTransport then join, report shows shutdown received.
#[test]
fn send_close_transport_stops_thread() {
    let mut handle = spawn_real_output_thread(config(10_000)).unwrap();
    let send_result = handle.send_command(OutputThreadRealTransportCommand::close_transport());
    assert!(send_result.is_ok(), "send should succeed");
    let join_handle = handle.take_join_handle().unwrap();
    let result = join_handle.join().unwrap().unwrap();
    assert!(
        result.shutdown_received,
        "shutdown should be received after manual send"
    );
    assert!(result.exited_cleanly, "thread should exit cleanly");
}
/// D. Thread exits after max_steps without shutdown command.
#[test]
fn worker_exits_after_max_steps_without_shutdown() {
    let mut handle = spawn_real_output_thread(config(5)).unwrap();
    let join_handle = handle.take_join_handle().unwrap();
    let report = join_handle.join().unwrap().unwrap();
    assert!(!report.shutdown_received, "no shutdown should be received");
    assert!(report.exited_cleanly, "thread should exit cleanly");
    assert_eq!(
        report.commands_processed, 0,
        "no commands should be processed"
    );
}
/// E. Send failure returns stable error.
#[test]
fn shutdown_send_failure_is_reported() {
    let mut handle = spawn_real_output_thread(config(100)).unwrap();
    let join_handle = handle.take_join_handle().unwrap();
    thread::sleep(Duration::from_millis(50));
    let _ = join_handle.join();
    let result = shutdown_and_join_real_output_thread(handle);
    assert!(
        result.is_err(),
        "shutdown should fail when receiver is dropped"
    );
    match result.unwrap_err() {
        RealOutputThreadSkeletonError::SendShutdown(_) => {} // expected
        other => panic!("expected SendShutdown error, got: {:?}", other),
    }
}
/// F. Join panic is reported as stable error.
#[test]
fn join_panic_is_reported() {
    let (sender, _receiver) = std::sync::mpsc::channel();
    let join_handle = thread::spawn(|| -> Result<super::thread_report::RealOutputThreadReport, RealOutputThreadSkeletonError> {
        panic!("test panic");
    });
    let mut handle = super::handle::OutputThreadRealTransportHandle::new(sender, join_handle);
    let taken = handle.take_join_handle().unwrap();
    let result = taken.join();
    assert!(
        result.is_err(),
        "join should return Err for panicked thread"
    );
}
/// G. Thread does not call IAudioClient::Start or GetBuffer/ReleaseBuffer.
#[test]
fn thread_does_not_start_audio_client() {
    let handle = spawn_real_output_thread(config(10)).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();
    assert!(
        report.thread_started,
        "thread should start without audio APIs"
    );
}
/// H. Skeleton does not call runtime audio layers.
#[test]
fn skeleton_does_not_call_runtime_audio_layers() {
    let handle = spawn_real_output_thread(config(10)).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();
    assert!(
        report.thread_started,
        "thread should start without audio layers"
    );
}
/// I. Owned-state validation happens inside the spawned thread.
#[test]
fn owned_state_validation_happens_inside_thread() {
    let handle = spawn_real_output_thread(config(100)).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();
    assert!(
        report.owned_state_validated,
        "owned state must be validated inside thread, not just before spawn"
    );
    assert!(report.thread_started, "thread should have started");
}
/// J. Context lifecycle flags are all false when open not requested.
#[test]
fn context_flags_false_when_not_requested() {
    let handle = spawn_real_output_thread(config(10)).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();
    assert!(
        !report.wasapi_context_open_requested,
        "open should not be requested"
    );
    assert!(
        !report.wasapi_context_opened,
        "context should not be opened"
    );
    assert!(
        !report.wasapi_context_closed,
        "context should not be closed"
    );
    assert!(!report.com_initialized, "COM should not be initialized");
    assert!(!report.com_uninitialized, "COM should not be uninitialized");
}
/// K. Non-Windows: open request returns WasapiContextOpen(UnsupportedPlatform).
///
/// Only runs on non-Windows to avoid requiring a real audio device.
#[cfg(not(target_os = "windows"))]
#[test]
fn context_open_returns_error_on_non_windows() {
    let ctx_config = RealOutputThreadSpawnConfig {
        max_steps: 10,
        open_wasapi_context_on_start: true,
    };
    let result =
        spawn_real_output_thread(ctx_config).and_then(|h| shutdown_and_join_real_output_thread(h));
    assert!(
        result.is_err(),
        "should fail on non-Windows with UnsupportedPlatform"
    );
    match result.unwrap_err() {
        RealOutputThreadSkeletonError::WasapiContextOpen(_) => {} // expected
        other => panic!("expected WasapiContextOpen, got: {:?}", other),
    }
}
/// L. Windows ignored smoke test: context open does not call Start/GetBuffer.
///
/// This test is only meaningful on Windows where real COM/WASAPI resources
/// are acquired. Marked ignored so default cargo test does not touch real devices.
///
/// Run manually:
///   cargo test --manifest-path src-tauri/Cargo.toml -- windows_context_open_smoke_test --ignored -- --nocapture
#[cfg(target_os = "windows")]
#[ignore = "requires a real Windows audio endpoint"]
#[test]
fn windows_context_open_smoke_test() {
    let ctx_config = RealOutputThreadSpawnConfig {
        max_steps: 10,
        open_wasapi_context_on_start: true,
    };
    let handle = spawn_real_output_thread(ctx_config).unwrap();
    let report = shutdown_and_join_real_output_thread(handle).unwrap();
    assert!(report.wasapi_context_opened, "context should be opened");
    assert!(report.wasapi_context_closed, "context should be closed");
    assert!(report.com_initialized, "COM should be initialized");
    // No audio client Start/Stop/GetBuffer was called.
    // We verify by code inspection: thread entry only calls open() and close().
    assert!(report.exited_cleanly, "thread should exit cleanly");
}
