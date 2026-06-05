//! Real output thread spawn and entry point.
//!
//! Creates a real std thread with a bounded worker loop skeleton.
//! Does not initialize COM, WASAPI, audio devices, or render clients.

use std::sync::mpsc;
use std::thread;

use super::super::sink_drain::{
    default_wasapi_output_thread_owned_state_contract,
    validate_wasapi_output_thread_owned_state_contract,
};
use super::super::worker_loop::runner::{
    run_worker_loop_skeleton, OutputThreadWorkerLoopRunConfig,
};
use super::super::worker_loop::state::OutputThreadWorkerLoopState;
use super::channel::OutputThreadRealTransportChannel;
use super::command::OutputThreadRealTransportCommand;
use super::handle::OutputThreadRealTransportHandle;
use super::thread_error::RealOutputThreadSkeletonError;
use super::thread_report::RealOutputThreadReport;

/// Configuration for spawning a real output thread.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct RealOutputThreadSpawnConfig {
    /// Maximum number of bounded loop iterations.
    pub max_steps: usize,
}

/// Spawn a real output thread with a bounded worker loop skeleton.
///
/// Creates a command channel pair and spawns a thread that runs
/// `run_worker_loop_skeleton` with the specified `max_steps`.
/// Owned-state validation happens inside the thread.
///
/// Returns a handle containing the sender and join handle.
#[allow(dead_code)]
pub(crate) fn spawn_real_output_thread(
    config: RealOutputThreadSpawnConfig,
) -> Result<OutputThreadRealTransportHandle, RealOutputThreadSkeletonError> {
    let (sender, receiver) = mpsc::channel();
    let join_handle = thread::spawn(move || run_real_output_thread_entry(receiver, config));

    Ok(OutputThreadRealTransportHandle::new(sender, join_handle))
}

/// Thread entry point. Runs owned-state validation and bounded worker loop.
///
/// Returns a report on success or an error if validation fails.
fn run_real_output_thread_entry(
    receiver: std::sync::mpsc::Receiver<OutputThreadRealTransportCommand>,
    config: RealOutputThreadSpawnConfig,
) -> Result<RealOutputThreadReport, RealOutputThreadSkeletonError> {
    // Validate owned-state contract inside the thread.
    let contract = default_wasapi_output_thread_owned_state_contract();
    validate_wasapi_output_thread_owned_state_contract(contract)
        .map_err(RealOutputThreadSkeletonError::OwnedStateValidation)?;

    // Create a dummy sender for the channel struct.
    // The real sender is in the handle; the worker only uses try_recv.
    let (dummy_sender, _) = mpsc::channel();
    let channel = OutputThreadRealTransportChannel::from_parts(dummy_sender, receiver);

    let loop_config = OutputThreadWorkerLoopRunConfig {
        max_steps: config.max_steps,
        initial_state: OutputThreadWorkerLoopState::NotStarted,
    };

    let loop_result = run_worker_loop_skeleton(&channel, loop_config);

    let report = loop_result.report;

    Ok(RealOutputThreadReport {
        thread_started: true,
        owned_state_validated: true,
        shutdown_received: report.stopped_by_close_transport,
        exited_cleanly: report.final_state.is_terminal() || !loop_result.stopped_early,
        commands_processed: report.commands_handled,
        loop_result: Some(loop_result),
        panicked: false,
    })
}

/// Send a shutdown command and join the thread.
///
/// Sends `CloseTransport` command, then joins the thread.
/// Returns the thread report or a stable error.
#[allow(dead_code)]
pub(crate) fn shutdown_and_join_real_output_thread(
    mut handle: OutputThreadRealTransportHandle,
) -> Result<RealOutputThreadReport, RealOutputThreadSkeletonError> {
    handle
        .send_command(OutputThreadRealTransportCommand::close_transport())
        .map_err(RealOutputThreadSkeletonError::SendShutdown)?;

    match handle.take_join_handle() {
        Some(join_handle) => match join_handle.join() {
            Ok(thread_result) => thread_result,
            Err(_) => Err(RealOutputThreadSkeletonError::JoinPanic),
        },
        None => Err(RealOutputThreadSkeletonError::WorkerDidNotReport),
    }
}
