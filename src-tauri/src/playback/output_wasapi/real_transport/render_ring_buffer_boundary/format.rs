use super::error::RenderRingBufferBoundaryError;
use crate::playback::output_wasapi::pcm_adapter::{
    validate_render_format, PcmRenderFormat, PcmSampleFormat,
};
use crate::playback::output_wasapi::ring_buffer::RingBufferFormat;
use crate::playback::output_wasapi::wasapi_context::WasapiFormatCache;

pub(crate) fn render_format_from_wasapi_cache_for_transport_boundary(
    cache: &WasapiFormatCache,
) -> Result<PcmRenderFormat, RenderRingBufferBoundaryError> {
    if !cache.is_float32() {
        return Err(RenderRingBufferBoundaryError::UnsupportedFormat);
    }

    validate_render_format(PcmRenderFormat {
        sample_rate_hz: cache.sample_rate_hz,
        channels: cache.channels,
        bits_per_sample: cache.bits_per_sample,
        block_align: cache.block_align,
        sample_format: PcmSampleFormat::Float32Interleaved,
    })
    .map_err(RenderRingBufferBoundaryError::PcmAdapter)
}

pub(super) fn ring_format(render_format: PcmRenderFormat) -> RingBufferFormat {
    RingBufferFormat {
        sample_rate_hz: render_format.sample_rate_hz,
        channels: render_format.channels,
        bits_per_sample: render_format.bits_per_sample,
        block_align: render_format.block_align,
    }
}
