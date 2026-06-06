use super::super::render_loop::RenderSilenceLoopOutcome;
use super::super::render_once::RenderSilenceOnceOutcome;
use super::super::render_padding_loop::{RenderPaddingLoopConfig, RenderPaddingLoopOutcome};
use super::super::render_ring_buffer_boundary::{
    RenderRingBufferBoundaryConfig, RenderRingBufferBoundaryOutcome,
};
use super::super::thread::RealOutputThreadSpawnConfig;
use super::super::thread_stages::{
    PostStartStageResults, RealOutputThreadLifecycleReportFields, RealOutputThreadStageConfigs,
    ResolvedThreadStageResults,
};
use crate::playback::output_wasapi::worker_loop::report::OutputThreadWorkerLoopReport;
use crate::playback::output_wasapi::worker_loop::runner::OutputThreadWorkerLoopRunResult;
use crate::playback::output_wasapi::worker_loop::state::OutputThreadWorkerLoopState;

pub(super) fn boundary_spawn_config(
    enabled: bool,
    iterations: u32,
    max_frames: u32,
    seed_frames: u32,
) -> RealOutputThreadSpawnConfig {
    RealOutputThreadSpawnConfig {
        max_steps: 1,
        open_wasapi_context_on_start: true,
        start_audio_client_on_start: true,
        render_ring_buffer_boundary_after_padding_loop: enabled,
        render_ring_buffer_boundary_iterations: iterations,
        render_ring_buffer_boundary_max_frames_per_write: max_frames,
        render_ring_buffer_boundary_synthetic_zero_seed_frames: seed_frames,
        ..Default::default()
    }
}

pub(super) fn stage_configs(
    enabled: bool,
    iterations: u32,
    max_frames: u32,
    seed_frames: u32,
) -> RealOutputThreadStageConfigs {
    RealOutputThreadStageConfigs::from_spawn_config(boundary_spawn_config(
        enabled,
        iterations,
        max_frames,
        seed_frames,
    ))
}

pub(super) fn render_loop_ok() -> RenderSilenceLoopOutcome {
    RenderSilenceLoopOutcome::not_requested()
}

pub(super) fn padding_loop_ok() -> RenderPaddingLoopOutcome {
    RenderPaddingLoopOutcome::disabled(RenderPaddingLoopConfig {
        enabled: false,
        iterations: 0,
        max_frames_per_write: 0,
    })
}

pub(super) fn boundary_ok() -> RenderRingBufferBoundaryOutcome {
    RenderRingBufferBoundaryOutcome::disabled(RenderRingBufferBoundaryConfig {
        enabled: false,
        iterations: 0,
        max_frames_per_write: 0,
        synthetic_zero_seed_frames: 0,
    })
}

pub(super) fn worker_loop_ok() -> OutputThreadWorkerLoopRunResult {
    OutputThreadWorkerLoopRunResult {
        report: OutputThreadWorkerLoopReport::empty(OutputThreadWorkerLoopState::NotStarted),
        stopped_early: false,
    }
}

pub(super) fn successful_stage_results() -> PostStartStageResults {
    PostStartStageResults {
        render_loop_result: Ok(render_loop_ok()),
        render_padding_loop_result: Ok(padding_loop_ok()),
        render_ring_buffer_boundary_result: Ok(boundary_ok()),
    }
}

pub(super) fn resolved_with_boundary(
    boundary: RenderRingBufferBoundaryOutcome,
) -> ResolvedThreadStageResults {
    ResolvedThreadStageResults {
        render_loop_outcome: render_loop_ok(),
        render_padding_loop_outcome: padding_loop_ok(),
        render_ring_buffer_boundary_outcome: boundary,
        loop_result: worker_loop_ok(),
    }
}

pub(super) fn lifecycle_fields() -> RealOutputThreadLifecycleReportFields {
    RealOutputThreadLifecycleReportFields {
        thread_started: true,
        owned_state_validated: true,
        panicked: false,
        com_initialized: true,
        com_uninitialized: true,
        wasapi_context_open_requested: true,
        wasapi_context_opened: true,
        wasapi_context_closed: true,
        audio_client_start_requested: true,
        audio_client_started: true,
        audio_client_stop_requested: true,
        audio_client_stopped: true,
    }
}

pub(super) fn render_once_disabled() -> RenderSilenceOnceOutcome {
    RenderSilenceOnceOutcome::not_requested()
}
