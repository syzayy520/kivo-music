use super::super::render_loop::{run_bounded_render_silence_loop, RenderSilenceLoopOutcome};
use super::super::render_once::{maybe_write_render_silence_once, RenderSilenceOnceOutcome};
use super::super::render_padding_loop::{
    run_bounded_render_padding_loop, RenderPaddingLoopOutcome,
};
use super::super::render_ring_buffer_boundary::{
    run_render_ring_buffer_boundary, RenderRingBufferBoundaryOutcome,
};
use super::super::thread_error::RealOutputThreadSkeletonError;
use super::config::RealOutputThreadStageConfigs;
use super::error_mapping::map_render_ring_buffer_boundary_error;
use super::outcomes::PostStartStageResults;
use crate::playback::output_wasapi::wasapi_context::WasapiContext;

pub(crate) fn run_render_once_stage(
    context: Option<&WasapiContext>,
    configs: RealOutputThreadStageConfigs,
) -> Result<RenderSilenceOnceOutcome, RealOutputThreadSkeletonError> {
    maybe_write_render_silence_once(context, configs.render_once)
}

pub(crate) fn run_post_start_stages(
    context: Option<&WasapiContext>,
    audio_client_started: bool,
    configs: RealOutputThreadStageConfigs,
) -> PostStartStageResults {
    run_post_start_stages_with(
        configs,
        || run_bounded_render_silence_loop(context, audio_client_started, configs.render_loop),
        || {
            run_bounded_render_padding_loop(
                context,
                audio_client_started,
                configs.render_padding_loop,
            )
        },
        || {
            run_render_ring_buffer_boundary(
                context,
                audio_client_started,
                configs.render_ring_buffer_boundary,
            )
            .map_err(map_render_ring_buffer_boundary_error)
        },
    )
}

pub(crate) fn run_post_start_stages_with<R, P, B>(
    configs: RealOutputThreadStageConfigs,
    render_loop: R,
    render_padding_loop: P,
    render_ring_buffer_boundary: B,
) -> PostStartStageResults
where
    R: FnOnce() -> Result<RenderSilenceLoopOutcome, RealOutputThreadSkeletonError>,
    P: FnOnce() -> Result<RenderPaddingLoopOutcome, RealOutputThreadSkeletonError>,
    B: FnOnce() -> Result<RenderRingBufferBoundaryOutcome, RealOutputThreadSkeletonError>,
{
    let render_loop_result = render_loop();
    let render_padding_loop_result = if render_loop_result.is_ok() {
        render_padding_loop()
    } else {
        Ok(RenderPaddingLoopOutcome::skipped_after_prior_failure(
            configs.render_padding_loop,
        ))
    };
    let render_ring_buffer_boundary_result =
        if render_loop_result.is_ok() && render_padding_loop_result.is_ok() {
            render_ring_buffer_boundary()
        } else {
            Ok(
                RenderRingBufferBoundaryOutcome::skipped_after_prior_failure(
                    configs.render_ring_buffer_boundary,
                ),
            )
        };

    PostStartStageResults {
        render_loop_result,
        render_padding_loop_result,
        render_ring_buffer_boundary_result,
    }
}
