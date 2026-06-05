//! Padding-aware bounded silent render loop helper.

use std::cmp;

use super::super::wasapi_context::WasapiContext;
use super::thread_error::RealOutputThreadSkeletonError;

pub(crate) const MAX_RENDER_PADDING_LOOP_ITERATIONS: u32 = 3;
pub(crate) const MAX_RENDER_PADDING_LOOP_FRAMES_PER_WRITE: u32 = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RenderPaddingLoopConfig {
    pub enabled: bool,
    pub iterations: u32,
    pub max_frames_per_write: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RenderPaddingLoopOutcome {
    pub requested: bool,
    pub started: bool,
    pub completed: bool,
    pub iterations_requested: u32,
    pub iterations_completed: u32,
    pub iterations_skipped_no_available: u32,
    pub max_frames_per_write: u32,
    pub frames_written_total: u32,
    pub last_capacity: u32,
    pub last_padding: u32,
    pub last_available: u32,
    pub used_silent_flag: bool,
}

impl RenderPaddingLoopOutcome {
    pub(crate) fn disabled(config: RenderPaddingLoopConfig) -> Self {
        Self {
            requested: false,
            started: false,
            completed: false,
            iterations_requested: config.iterations,
            iterations_completed: 0,
            iterations_skipped_no_available: 0,
            max_frames_per_write: config.max_frames_per_write,
            frames_written_total: 0,
            last_capacity: 0,
            last_padding: 0,
            last_available: 0,
            used_silent_flag: false,
        }
    }

    pub(crate) fn skipped_after_prior_failure(config: RenderPaddingLoopConfig) -> Self {
        Self {
            requested: config.enabled,
            started: false,
            completed: false,
            iterations_requested: config.iterations,
            iterations_completed: 0,
            iterations_skipped_no_available: 0,
            max_frames_per_write: config.max_frames_per_write,
            frames_written_total: 0,
            last_capacity: 0,
            last_padding: 0,
            last_available: 0,
            used_silent_flag: false,
        }
    }
}

pub(crate) fn validate_render_padding_loop_config(
    config: RenderPaddingLoopConfig,
    open_wasapi_context_on_start: bool,
    start_audio_client_on_start: bool,
) -> Result<(), RealOutputThreadSkeletonError> {
    if !config.enabled {
        return Ok(());
    }
    validate_render_padding_loop_numbers(config)?;
    if !open_wasapi_context_on_start {
        return Err(RealOutputThreadSkeletonError::RenderPaddingLoopRequiresOpenContext);
    }
    if !start_audio_client_on_start {
        return Err(RealOutputThreadSkeletonError::RenderPaddingLoopRequiresStartedClient);
    }
    Ok(())
}

pub(crate) fn run_bounded_render_padding_loop(
    context: Option<&WasapiContext>,
    audio_client_started: bool,
    config: RenderPaddingLoopConfig,
) -> Result<RenderPaddingLoopOutcome, RealOutputThreadSkeletonError> {
    if !config.enabled {
        return Ok(RenderPaddingLoopOutcome::disabled(config));
    }
    validate_render_padding_loop_numbers(config)?;

    let context_ref =
        context.ok_or(RealOutputThreadSkeletonError::RenderPaddingLoopRequiresOpenContext)?;
    if !audio_client_started {
        return Err(RealOutputThreadSkeletonError::RenderPaddingLoopRequiresStartedClient);
    }

    let mut outcome = RenderPaddingLoopOutcome {
        requested: true,
        started: true,
        completed: false,
        iterations_requested: config.iterations,
        iterations_completed: 0,
        iterations_skipped_no_available: 0,
        max_frames_per_write: config.max_frames_per_write,
        frames_written_total: 0,
        last_capacity: 0,
        last_padding: 0,
        last_available: 0,
        used_silent_flag: false,
    };
    let mut writes_performed = false;
    let mut all_writes_used_silent_flag = true;

    for _ in 0..config.iterations {
        let snapshot = context_ref.padding_state_snapshot().map_err(|error| {
            RealOutputThreadSkeletonError::RenderPaddingLoopPaddingStateFailed(format!("{error:?}"))
        })?;
        outcome.last_capacity = snapshot.buffer_frame_capacity;
        outcome.last_padding = snapshot.current_padding_frames;
        outcome.last_available = snapshot.available_frames;

        if snapshot.available_frames == 0 {
            outcome.iterations_skipped_no_available += 1;
            outcome.iterations_completed += 1;
            continue;
        }

        let frames_to_write = cmp::min(snapshot.available_frames, config.max_frames_per_write);
        if frames_to_write == 0 {
            return Err(RealOutputThreadSkeletonError::RenderPaddingLoopWriteFailed(
                "frames_to_write was zero after available check".to_string(),
            ));
        }

        let write = context_ref
            .write_render_buffer_silence(frames_to_write)
            .map_err(|error| {
                RealOutputThreadSkeletonError::RenderPaddingLoopWriteFailed(error.to_string())
            })?;

        if write.frames_written == 0 {
            return Err(RealOutputThreadSkeletonError::RenderPaddingLoopWriteFailed(
                "silent write reported zero frames".to_string(),
            ));
        }
        if write.frames_written > frames_to_write {
            return Err(RealOutputThreadSkeletonError::RenderPaddingLoopWriteFailed(
                "silent write reported more frames than requested".to_string(),
            ));
        }
        if write.frames_written != frames_to_write {
            return Err(RealOutputThreadSkeletonError::RenderPaddingLoopWriteFailed(
                "silent write reported unexpected frame count".to_string(),
            ));
        }
        if !write.used_silent_flag {
            return Err(RealOutputThreadSkeletonError::RenderPaddingLoopWriteFailed(
                "silent render flag was not used".to_string(),
            ));
        }

        outcome.frames_written_total += write.frames_written;
        writes_performed = true;
        all_writes_used_silent_flag &= write.used_silent_flag;
        outcome.iterations_completed += 1;
    }

    outcome.completed = true;
    outcome.used_silent_flag = writes_performed && all_writes_used_silent_flag;
    Ok(outcome)
}

fn validate_render_padding_loop_numbers(
    config: RenderPaddingLoopConfig,
) -> Result<(), RealOutputThreadSkeletonError> {
    if config.iterations == 0 {
        return Err(RealOutputThreadSkeletonError::RenderPaddingLoopInvalidIterationCount);
    }
    if config.max_frames_per_write == 0 {
        return Err(RealOutputThreadSkeletonError::RenderPaddingLoopInvalidFrameCount);
    }
    if config.iterations > MAX_RENDER_PADDING_LOOP_ITERATIONS {
        return Err(RealOutputThreadSkeletonError::RenderPaddingLoopIterationCountTooLarge);
    }
    if config.max_frames_per_write > MAX_RENDER_PADDING_LOOP_FRAMES_PER_WRITE {
        return Err(RealOutputThreadSkeletonError::RenderPaddingLoopFrameCountTooLarge);
    }
    Ok(())
}
