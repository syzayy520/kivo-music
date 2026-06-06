use super::config::{boundary_config, validate_config, RenderRingBufferBoundaryConfig};
use super::error::{map_ring_buffer_boundary_error, RenderRingBufferBoundaryError};
use super::format::render_format_from_wasapi_cache_for_transport_boundary;
use super::outcome::{outcome_from_boundary, RenderRingBufferBoundaryOutcome};
use super::source::build_synthetic_zero_ring_buffer_source;
use crate::playback::output_wasapi::pcm_adapter::PcmRenderFormat;
use crate::playback::output_wasapi::ring_buffer_render_boundary::{
    run_padding_aware_ring_buffer_render_boundary,
    run_padding_aware_ring_buffer_render_boundary_with_callbacks, RingBufferRenderBoundaryError,
};
use crate::playback::output_wasapi::wasapi_context::{
    WasapiContext, WasapiPaddingStateSnapshot, WasapiRenderWriteReport,
};

pub(crate) fn run_render_ring_buffer_boundary(
    context: Option<&WasapiContext>,
    audio_client_started: bool,
    config: RenderRingBufferBoundaryConfig,
) -> Result<RenderRingBufferBoundaryOutcome, RenderRingBufferBoundaryError> {
    if !config.enabled {
        return Ok(RenderRingBufferBoundaryOutcome::disabled(config));
    }

    validate_config(config)?;
    let context = context.ok_or(RenderRingBufferBoundaryError::RequiresContext)?;
    if !audio_client_started {
        return Err(RenderRingBufferBoundaryError::RequiresStartedClient);
    }

    let cache = context
        .format_cache()
        .ok_or(RenderRingBufferBoundaryError::MissingFormatCache)?;
    let render_format = render_format_from_wasapi_cache_for_transport_boundary(cache)?;
    let mut source =
        build_synthetic_zero_ring_buffer_source(render_format, config.synthetic_zero_seed_frames)?;
    let boundary = run_padding_aware_ring_buffer_render_boundary(
        context,
        &mut source,
        boundary_config(config),
    )
    .map_err(map_ring_buffer_boundary_error)?;

    Ok(outcome_from_boundary(
        config,
        boundary,
        source.available_frames(),
    ))
}

pub(crate) fn run_render_ring_buffer_boundary_with_callbacks<P, W>(
    render_format: PcmRenderFormat,
    config: RenderRingBufferBoundaryConfig,
    padding_snapshot: P,
    write_bytes: W,
) -> Result<RenderRingBufferBoundaryOutcome, RenderRingBufferBoundaryError>
where
    P: FnMut() -> Result<WasapiPaddingStateSnapshot, RingBufferRenderBoundaryError>,
    W: FnMut(u32, &[u8]) -> Result<WasapiRenderWriteReport, RingBufferRenderBoundaryError>,
{
    if !config.enabled {
        return Ok(RenderRingBufferBoundaryOutcome::disabled(config));
    }

    validate_config(config)?;
    let mut source =
        build_synthetic_zero_ring_buffer_source(render_format, config.synthetic_zero_seed_frames)?;
    let boundary = run_padding_aware_ring_buffer_render_boundary_with_callbacks(
        &mut source,
        render_format,
        boundary_config(config),
        padding_snapshot,
        write_bytes,
    )
    .map_err(map_ring_buffer_boundary_error)?;

    Ok(outcome_from_boundary(
        config,
        boundary,
        source.available_frames(),
    ))
}
