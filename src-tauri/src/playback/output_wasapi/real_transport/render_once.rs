//! One-shot silent render buffer write helper for the real output thread.

use super::super::wasapi_context::WasapiContext;
use super::thread_error::RealOutputThreadSkeletonError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RenderSilenceOnceConfig {
    pub enabled: bool,
    pub frames: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RenderSilenceOnceOutcome {
    pub requested: bool,
    pub written: bool,
    pub frames_requested: u32,
    pub frames_written: u32,
    pub used_silent_flag: bool,
}

impl RenderSilenceOnceOutcome {
    pub(crate) fn not_requested() -> Self {
        Self {
            requested: false,
            written: false,
            frames_requested: 0,
            frames_written: 0,
            used_silent_flag: false,
        }
    }
}

pub(crate) fn validate_render_silence_once_config(
    config: RenderSilenceOnceConfig,
) -> Result<(), RealOutputThreadSkeletonError> {
    if config.enabled && config.frames == 0 {
        Err(RealOutputThreadSkeletonError::RenderSilenceOnceInvalidFrameCount)
    } else {
        Ok(())
    }
}

pub(crate) fn maybe_write_render_silence_once(
    context: Option<&WasapiContext>,
    config: RenderSilenceOnceConfig,
) -> Result<RenderSilenceOnceOutcome, RealOutputThreadSkeletonError> {
    if !config.enabled {
        return Ok(RenderSilenceOnceOutcome::not_requested());
    }
    if config.frames == 0 {
        return Err(RealOutputThreadSkeletonError::RenderSilenceOnceInvalidFrameCount);
    }

    let context_ref =
        context.ok_or(RealOutputThreadSkeletonError::RenderSilenceOnceRequiresOpenContext)?;
    let report = context_ref
        .write_render_buffer_silence(config.frames)
        .map_err(|error| {
            RealOutputThreadSkeletonError::RenderSilenceOnceFailed(error.to_string())
        })?;

    Ok(RenderSilenceOnceOutcome {
        requested: true,
        written: true,
        frames_requested: config.frames,
        frames_written: report.frames_written,
        used_silent_flag: report.used_silent_flag,
    })
}
