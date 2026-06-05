//! Tests for real output thread spawn, handle, and shutdown.
//!
//! 8 required tests (A-H):
//! A. spawn_creates_valid_thread
//! B. handle_holds_sender
//! C. send_close_transport_stops_thread
//! D. shutdown_and_join_completes
//! E. thread_runs_bounded_loop
//! F. thread_id_is_stable
//! G. owned_state_validation_passes_on_spawn
//! H. thread_exits_cleanly_after_max_steps

use std::thread;
use std::time::Duration;

use super::command::OutputThreadRealTransportCommand;
use super::thread::{
    shutdown_and_join_real_output_thread, spawn_real_output_thread, RealOutputThreadSpawnConfig,
};

/// A. Thread spawns successfully and handle is returned.
#[test]
fn spawn_creates_valid_thread() {
    let config = RealOutputThreadSpawnConfig { max_steps: 100 };
    let handle = spawn_real_output_thread(config);
    assert!(handle.is_ok(), "spawn should succeed with valid contract");

    let handle = handle.unwrap();
    assert!(handle.has_join_handle(), "handle should have join handle");

    let _ = shutdown_and_join_real_output_thread(handle);
}

/// B. Handle holds a sender that can send commands.
#[test]
fn handle_holds_sender() {
    let config = RealOutputThreadSpawnConfig { max_steps: 100 };
    let handle = spawn_real_output_thread(config).unwrap();

    let result = handle.send_command(OutputThreadRealTransportCommand::close_transport());
    assert!(result.is_ok(), "send should succeed while thread is alive");

    let _ = shutdown_and_join_real_output_thread(handle);
}

/// C. Sending CloseTransport stops the thread.
#[test]
fn send_close_transport_stops_thread() {
    let config = RealOutputThreadSpawnConfig { max_steps: 10_000 };
    let handle = spawn_real_output_thread(config).unwrap();

    let send_result = handle.send_command(OutputThreadRealTransportCommand::close_transport());
    assert!(send_result.is_ok(), "send should succeed");

    let join_result = shutdown_and_join_real_output_thread(handle);
    assert!(join_result.is_ok(), "thread should exit cleanly");
}

/// D. Shutdown and join completes without hanging.
#[test]
fn shutdown_and_join_completes() {
    let config = RealOutputThreadSpawnConfig { max_steps: 100 };
    let handle = spawn_real_output_thread(config).unwrap();

    let start = std::time::Instant::now();
    let result = shutdown_and_join_real_output_thread(handle);
    let elapsed = start.elapsed();

    assert!(result.is_ok(), "shutdown and join should succeed");
    assert!(
        elapsed < Duration::from_secs(5),
        "shutdown should complete within 5 seconds"
    );
}

/// E. Thread runs bounded loop (does not hang on its own).
#[test]
fn thread_runs_bounded_loop() {
    let config = RealOutputThreadSpawnConfig { max_steps: 10 };
    let handle = spawn_real_output_thread(config).unwrap();

    // Without sending CloseTransport, the thread should exit after max_steps.
    let start = std::time::Instant::now();
    let result = shutdown_and_join_real_output_thread(handle);
    let elapsed = start.elapsed();

    assert!(result.is_ok(), "thread should exit after max_steps");
    assert!(
        elapsed < Duration::from_secs(5),
        "bounded loop should complete quickly"
    );
}

/// F. Thread ID is stable and consistent.
#[test]
fn thread_id_is_stable() {
    let config = RealOutputThreadSpawnConfig { max_steps: 100 };
    let handle = spawn_real_output_thread(config).unwrap();

    let id1 = handle.thread_id();
    let id2 = handle.thread_id();
    assert_eq!(id1, id2, "thread id should be stable across calls");

    let _ = shutdown_and_join_real_output_thread(handle);
}

/// G. Owned-state validation passes before spawn.
#[test]
fn owned_state_validation_passes_on_spawn() {
    use super::super::sink_drain::{
        default_wasapi_output_thread_owned_state_contract,
        validate_wasapi_output_thread_owned_state_contract,
    };

    let contract = default_wasapi_output_thread_owned_state_contract();
    let result = validate_wasapi_output_thread_owned_state_contract(contract);
    assert!(result.is_ok(), "default contract should be valid");

    let config = RealOutputThreadSpawnConfig { max_steps: 50 };
    let spawn_result = spawn_real_output_thread(config);
    assert!(
        spawn_result.is_ok(),
        "spawn should succeed with valid contract"
    );

    let _ = shutdown_and_join_real_output_thread(spawn_result.unwrap());
}

/// H. Thread exits cleanly after exhausting max_steps.
#[test]
fn thread_exits_cleanly_after_max_steps() {
    let config = RealOutputThreadSpawnConfig { max_steps: 5 };
    let mut handle = spawn_real_output_thread(config).unwrap();

    // Wait a bit for the thread to finish its bounded loop.
    thread::sleep(Duration::from_millis(50));

    // Join should succeed even without sending CloseTransport.
    let join_handle = handle.take_join_handle().unwrap();
    let result = join_handle.join();
    assert!(result.is_ok(), "thread should exit cleanly after max_steps");
}
