//! Bounded silent render loop helper for the real output thread.

use super::super::wasapi_context::WasapiContext;
use super::thread_error::RealOutputThreadSkeletonError;

pub(crate) const MAX_RENDER_SILENCE_LOOP_ITERATIONS: u32 = 3;
pub(crate) const MAX_RENDER_SILENCE_LOOP_FRAMES_PER_WRITE: u32 = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RenderSilenceLoopConfig {
    pub enabled: bool,
    pub iterations: u32,
    pub frames_per_write: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RenderSilenceLoopOutcome {
    pub requested: bool,
    pub started: bool,
    pub completed: bool,
    pub iterations_requested: u32,
    pub iterations_completed: u32,
    pub frames_per_write: u32,
    pub frames_written_total: u32,
    pub used_silent_flag: bool,
}

impl RenderSilenceLoopOutcome {
    pub(crate) fn not_requested() -> Self {
        Self {
            requested: false,
            started: false,
            completed: false,
            iterations_requested: 0,
            iterations_completed: 0,
            frames_per_write: 0,
            frames_written_total: 0,
            used_silent_flag: false,
        }
    }
}

pub(crate) fn validate_render_silence_loop_config(
    config: RenderSilenceLoopConfig,
    open_wasapi_context_on_start: bool,
    start_audio_client_on_start: bool,
) -> Result<(), RealOutputThreadSkeletonError> {
    validate_render_silence_loop_numbers(config)?;
    if !config.enabled {
        return Ok(());
    }
    if !open_wasapi_context_on_start {
        return Err(RealOutputThreadSkeletonError::RenderSilenceLoopRequiresOpenContext);
    }
    if !start_audio_client_on_start {
        return Err(RealOutputThreadSkeletonError::RenderSilenceLoopRequiresStartedClient);
    }
    Ok(())
}

pub(crate) fn run_bounded_render_silence_loop(
    context: Option<&WasapiContext>,
    audio_client_started: bool,
    config: RenderSilenceLoopConfig,
) -> Result<RenderSilenceLoopOutcome, RealOutputThreadSkeletonError> {
    validate_render_silence_loop_numbers(config)?;
    if !config.enabled {
        return Ok(RenderSilenceLoopOutcome::not_requested());
    }

    let context_ref =
        context.ok_or(RealOutputThreadSkeletonError::RenderSilenceLoopRequiresOpenContext)?;
    if !audio_client_started {
        return Err(RealOutputThreadSkeletonError::RenderSilenceLoopRequiresStartedClient);
    }

    let mut outcome = RenderSilenceLoopOutcome {
        requested: true,
        started: true,
        completed: false,
        iterations_requested: config.iterations,
        iterations_completed: 0,
        frames_per_write: config.frames_per_write,
        frames_written_total: 0,
        used_silent_flag: true,
    };

    for _ in 0..config.iterations {
        let report = context_ref
            .write_render_buffer_silence(config.frames_per_write)
            .map_err(|error| RealOutputThreadSkeletonError::RenderSilenceLoopFailed(error.to_string()))?;

        outcome.iterations_completed += 1;
        outcome.frames_written_total += report.frames_written;
        outcome.used_silent_flag &= report.used_silent_flag;
        if !outcome.used_silent_flag {
            return Err(RealOutputThreadSkeletonError::RenderSilenceLoopFailed(
                "silent render flag was not used".to_string(),
            ));
        }
    }

    outcome.completed = true;
    Ok(outcome)
}

fn validate_render_silence_loop_numbers(
    config: RenderSilenceLoopConfig,
) -> Result<(), RealOutputThreadSkeletonError> {
    if !config.enabled {
        return Ok(());
    }
    if config.iterations == 0 {
        return Err(RealOutputThreadSkeletonError::RenderSilenceLoopInvalidIterationCount);
    }
    if config.frames_per_write == 0 {
        return Err(RealOutputThreadSkeletonError::RenderSilenceLoopInvalidFrameCount);
    }
    if config.iterations > MAX_RENDER_SILENCE_LOOP_ITERATIONS {
        return Err(RealOutputThreadSkeletonError::RenderSilenceLoopIterationCountTooLarge);
    }
    if config.frames_per_write > MAX_RENDER_SILENCE_LOOP_FRAMES_PER_WRITE {
        return Err(RealOutputThreadSkeletonError::RenderSilenceLoopFrameCountTooLarge);
    }
    Ok(())
}
