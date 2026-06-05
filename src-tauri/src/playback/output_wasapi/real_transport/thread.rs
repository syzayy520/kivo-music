//! Real output thread spawn and entry point.
//!
//! Creates a real std thread with a bounded worker loop skeleton.
//! Does not initialize COM, WASAPI, audio devices, or render clients.

use std::sync::mpsc;
use std::thread;

use super::super::sink_drain::{
    default_wasapi_output_thread_owned_state_contract,
    validate_wasapi_output_thread_owned_state_contract, WasapiOutputThreadOwnedStateError,
};
use super::super::worker_loop::runner::{
    run_worker_loop_skeleton, OutputThreadWorkerLoopRunConfig, OutputThreadWorkerLoopRunResult,
};
use super::super::worker_loop::state::OutputThreadWorkerLoopState;
use super::channel::OutputThreadRealTransportChannel;
use super::command::OutputThreadRealTransportCommand;
use super::handle::OutputThreadRealTransportHandle;

/// Configuration for spawning a real output thread.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct RealOutputThreadSpawnConfig {
    /// Maximum number of bounded loop iterations.
    pub max_steps: usize,
}

/// Result report from a completed output thread.
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct RealOutputThreadReport {
    /// The worker loop run result.
    pub loop_result: OutputThreadWorkerLoopRunResult,
    /// Whether the thread panicked.
    pub panicked: bool,
}

/// Spawn a real output thread with a bounded worker loop skeleton.
///
/// Validates the owned-state contract, creates a command channel pair,
/// and spawns a thread that runs `run_worker_loop_skeleton` with the
/// specified `max_steps`.
///
/// Returns a handle containing the sender and join handle.
#[allow(dead_code)]
pub(crate) fn spawn_real_output_thread(
    config: RealOutputThreadSpawnConfig,
) -> Result<OutputThreadRealTransportHandle, WasapiOutputThreadOwnedStateError> {
    let contract = default_wasapi_output_thread_owned_state_contract();
    validate_wasapi_output_thread_owned_state_contract(contract)?;

    let (sender, receiver) = mpsc::channel();
    let join_handle = thread::spawn(move || {
        run_real_output_thread_entry(receiver, config);
    });

    Ok(OutputThreadRealTransportHandle::new(sender, join_handle))
}

/// Thread entry point. Runs the bounded worker loop skeleton.
fn run_real_output_thread_entry(
    receiver: std::sync::mpsc::Receiver<OutputThreadRealTransportCommand>,
    config: RealOutputThreadSpawnConfig,
) {
    // Create a dummy sender for the channel struct.
    // The real sender is in the handle; the worker only uses try_recv.
    let (dummy_sender, _) = mpsc::channel();
    let channel = OutputThreadRealTransportChannel::from_parts(dummy_sender, receiver);

    let loop_config = OutputThreadWorkerLoopRunConfig {
        max_steps: config.max_steps,
        initial_state: OutputThreadWorkerLoopState::NotStarted,
    };

    let _loop_result = run_worker_loop_skeleton(&channel, loop_config);
}

/// Send a shutdown command and join the thread.
///
/// Sends `CloseTransport` command, then joins the thread with no timeout.
/// Returns the join result.
#[allow(dead_code)]
pub(crate) fn shutdown_and_join_real_output_thread(
    mut handle: OutputThreadRealTransportHandle,
) -> thread::Result<()> {
    let _ = handle.send_command(OutputThreadRealTransportCommand::close_transport());

    if let Some(join_handle) = handle.take_join_handle() {
        join_handle.join()
    } else {
        Ok(())
    }
}
