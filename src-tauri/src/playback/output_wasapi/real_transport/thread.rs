use std::sync::mpsc;
use std::thread;

use super::super::sink_drain::{
    default_wasapi_output_thread_owned_state_contract,
    validate_wasapi_output_thread_owned_state_contract,
};
use super::super::wasapi_context::WasapiContext;
use super::super::worker_loop::runner::{
    run_worker_loop_skeleton, OutputThreadWorkerLoopRunConfig,
};
use super::super::worker_loop::state::OutputThreadWorkerLoopState;
use super::channel::OutputThreadRealTransportChannel;
use super::command::OutputThreadRealTransportCommand;
use super::handle::OutputThreadRealTransportHandle;
use super::render_loop::{
    run_bounded_render_silence_loop, validate_render_silence_loop_config,
    RenderSilenceLoopConfig,
};
use super::render_once::{
    maybe_write_render_silence_once, validate_render_silence_once_config, RenderSilenceOnceConfig,
};
use super::thread_error::RealOutputThreadSkeletonError;
use super::thread_report::RealOutputThreadReport;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct RealOutputThreadSpawnConfig {
    pub max_steps: usize,
    pub open_wasapi_context_on_start: bool,
    pub start_audio_client_on_start: bool,
    pub render_silence_once_after_open: bool,
    pub render_silence_once_frames: u32,
    pub render_silence_loop_after_start: bool,
    pub render_silence_loop_iterations: u32,
    pub render_silence_loop_frames_per_write: u32,
}

#[allow(dead_code)]
pub(crate) fn spawn_real_output_thread(
    config: RealOutputThreadSpawnConfig,
) -> Result<OutputThreadRealTransportHandle, RealOutputThreadSkeletonError> {
    let (sender, receiver) = mpsc::channel();
    let join_handle = thread::spawn(move || run_real_output_thread_entry(receiver, config));

    Ok(OutputThreadRealTransportHandle::new(sender, join_handle))
}

fn run_real_output_thread_entry(
    receiver: std::sync::mpsc::Receiver<OutputThreadRealTransportCommand>,
    config: RealOutputThreadSpawnConfig,
) -> Result<RealOutputThreadReport, RealOutputThreadSkeletonError> {
    let thread_started = true;

    // Validate owned-state contract inside the thread.
    let contract = default_wasapi_output_thread_owned_state_contract();
    validate_wasapi_output_thread_owned_state_contract(contract)
        .map_err(RealOutputThreadSkeletonError::OwnedStateValidation)?;

    let render_once_config = RenderSilenceOnceConfig {
        enabled: config.render_silence_once_after_open,
        frames: config.render_silence_once_frames,
    };
    validate_render_silence_once_config(render_once_config)?;
    let render_loop_config = RenderSilenceLoopConfig {
        enabled: config.render_silence_loop_after_start,
        iterations: config.render_silence_loop_iterations,
        frames_per_write: config.render_silence_loop_frames_per_write,
    };
    validate_render_silence_loop_config(
        render_loop_config,
        config.open_wasapi_context_on_start,
        config.start_audio_client_on_start,
    )?;

    // Optionally open WasapiContext inside the thread.
    // Context is owned locally — never stored in handle or returned to caller.
    let mut context: Option<WasapiContext> = None;
    let mut com_initialized = false;
    let mut wasapi_context_opened = false;
    let audio_client_start_requested = config.start_audio_client_on_start;
    let mut audio_client_started = false;
    let mut audio_client_stop_requested = false;
    let mut audio_client_stopped = false;
    let mut started_guard = None;

    if config.open_wasapi_context_on_start {
        let mut ctx = WasapiContext::new();
        ctx.open()
            .map_err(RealOutputThreadSkeletonError::WasapiContextOpen)?;
        // If open() succeeded on Windows, COM is initialized and context is open.
        com_initialized = ctx.is_open();
        wasapi_context_opened = ctx.is_open();
        context = Some(ctx);
    }

    let render_once_outcome =
        maybe_write_render_silence_once(context.as_ref(), render_once_config)?;

    if config.start_audio_client_on_start {
        if !config.open_wasapi_context_on_start {
            return Err(RealOutputThreadSkeletonError::AudioClientStartRequiresOpenContext);
        }

        let context_ref = context
            .as_ref()
            .ok_or(RealOutputThreadSkeletonError::AudioClientStartRequiresOpenContext)?;
        let guard = context_ref
            .start_audio_client()
            .map_err(RealOutputThreadSkeletonError::AudioClientStartFailed)?;

        audio_client_started = true;
        started_guard = Some(guard);
    }

    let render_loop_result = run_bounded_render_silence_loop(
        context.as_ref(),
        audio_client_started,
        render_loop_config,
    );
    let worker_loop_result = if render_loop_result.is_ok() {
        let (dummy_sender, _) = mpsc::channel();
        let channel = OutputThreadRealTransportChannel::from_parts(dummy_sender, receiver);
        let loop_config = OutputThreadWorkerLoopRunConfig {
            max_steps: config.max_steps,
            initial_state: OutputThreadWorkerLoopState::NotStarted,
        };
        Some(run_worker_loop_skeleton(&channel, loop_config))
    } else {
        None
    };

    // Close context (drop) before returning — happens inside the thread.
    let stop_result = if let Some(mut guard) = started_guard.take() {
        audio_client_stop_requested = true;
        let result = guard.stop();
        if result.is_ok() {
            audio_client_stopped = true;
        }
        result
    } else {
        Ok(())
    };

    let mut wasapi_context_closed = false;
    if let Some(mut ctx) = context {
        ctx.close();
        wasapi_context_closed = true;
    }
    let com_uninitialized = wasapi_context_closed;

    stop_result.map_err(RealOutputThreadSkeletonError::AudioClientStopFailed)?;
    let render_loop_outcome = render_loop_result?;
    let loop_result = match worker_loop_result {
        Some(loop_result) => loop_result,
        None => return Err(RealOutputThreadSkeletonError::WorkerDidNotReport),
    };
    let report = loop_result.report;

    Ok(RealOutputThreadReport {
        thread_started,
        owned_state_validated: true,
        shutdown_received: report.stopped_by_close_transport,
        exited_cleanly: report.final_state.is_terminal() || !loop_result.stopped_early,
        commands_processed: report.commands_handled,
        loop_result: Some(loop_result),
        panicked: false,
        com_initialized,
        com_uninitialized,
        wasapi_context_open_requested: config.open_wasapi_context_on_start,
        wasapi_context_opened,
        wasapi_context_closed,
        audio_client_start_requested,
        audio_client_started,
        audio_client_stop_requested,
        audio_client_stopped,
        render_silence_once_requested: render_once_outcome.requested,
        render_silence_once_written: render_once_outcome.written,
        render_silence_once_frames_requested: render_once_outcome.frames_requested,
        render_silence_once_frames_written: render_once_outcome.frames_written,
        render_silence_once_used_silent_flag: render_once_outcome.used_silent_flag,
        render_silence_loop_requested: render_loop_outcome.requested,
        render_silence_loop_started: render_loop_outcome.started,
        render_silence_loop_completed: render_loop_outcome.completed,
        render_silence_loop_iterations_requested: render_loop_outcome.iterations_requested,
        render_silence_loop_iterations_completed: render_loop_outcome.iterations_completed,
        render_silence_loop_frames_per_write: render_loop_outcome.frames_per_write,
        render_silence_loop_frames_written_total: render_loop_outcome.frames_written_total,
        render_silence_loop_used_silent_flag: render_loop_outcome.used_silent_flag,
    })
}

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
