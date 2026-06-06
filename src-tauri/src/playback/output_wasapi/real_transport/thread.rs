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
use super::thread_error::RealOutputThreadSkeletonError;
use super::thread_report::RealOutputThreadReport;
use super::thread_stages::{
    build_real_output_thread_report, resolve_thread_stage_results, run_post_start_stages,
    run_render_once_stage, validate_thread_stage_configs, RealOutputThreadLifecycleReportFields,
    RealOutputThreadStageConfigs,
};

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
    pub render_padding_loop_after_start: bool,
    pub render_padding_loop_iterations: u32,
    pub render_padding_loop_max_frames_per_write: u32,
    pub render_ring_buffer_boundary_after_padding_loop: bool,
    pub render_ring_buffer_boundary_iterations: u32,
    pub render_ring_buffer_boundary_max_frames_per_write: u32,
    pub render_ring_buffer_boundary_synthetic_zero_seed_frames: u32,
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
    let contract = default_wasapi_output_thread_owned_state_contract();
    validate_wasapi_output_thread_owned_state_contract(contract)
        .map_err(RealOutputThreadSkeletonError::OwnedStateValidation)?;

    let stage_configs = RealOutputThreadStageConfigs::from_spawn_config(config);
    validate_thread_stage_configs(
        stage_configs,
        config.open_wasapi_context_on_start,
        config.start_audio_client_on_start,
    )?;

    let mut context: Option<WasapiContext> = None;
    let mut com_initialized = false;
    let mut wasapi_context_opened = false;
    let mut audio_client_started = false;
    let mut audio_client_stop_requested = false;
    let mut audio_client_stopped = false;
    let mut started_guard = None;

    if config.open_wasapi_context_on_start {
        let mut opened_context = WasapiContext::new();
        opened_context
            .open()
            .map_err(RealOutputThreadSkeletonError::WasapiContextOpen)?;
        com_initialized = opened_context.is_open();
        wasapi_context_opened = opened_context.is_open();
        context = Some(opened_context);
    }

    let render_once_outcome = run_render_once_stage(context.as_ref(), stage_configs)?;

    if config.start_audio_client_on_start {
        let context_ref = context
            .as_ref()
            .ok_or(RealOutputThreadSkeletonError::AudioClientStartRequiresOpenContext)?;
        let guard = context_ref
            .start_audio_client()
            .map_err(RealOutputThreadSkeletonError::AudioClientStartFailed)?;

        audio_client_started = true;
        started_guard = Some(guard);
    }

    let stage_results =
        run_post_start_stages(context.as_ref(), audio_client_started, stage_configs);
    let worker_loop_result = if stage_results.can_run_worker_loop() {
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
    if let Some(mut opened_context) = context {
        opened_context.close();
        wasapi_context_closed = true;
    }

    let resolved = resolve_thread_stage_results(stop_result, stage_results, worker_loop_result)?;
    Ok(build_real_output_thread_report(
        RealOutputThreadLifecycleReportFields {
            thread_started: true,
            owned_state_validated: true,
            panicked: false,
            com_initialized,
            com_uninitialized: wasapi_context_closed,
            wasapi_context_open_requested: config.open_wasapi_context_on_start,
            wasapi_context_opened,
            wasapi_context_closed,
            audio_client_start_requested: config.start_audio_client_on_start,
            audio_client_started,
            audio_client_stop_requested,
            audio_client_stopped,
        },
        render_once_outcome,
        resolved,
    ))
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
