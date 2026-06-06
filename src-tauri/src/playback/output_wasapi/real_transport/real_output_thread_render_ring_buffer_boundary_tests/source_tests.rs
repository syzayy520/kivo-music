use super::super::render_ring_buffer_boundary::{
    build_synthetic_zero_ring_buffer_source,
    render_format_from_wasapi_cache_for_transport_boundary, RenderRingBufferBoundaryError,
    MAX_SYNTHETIC_ZERO_SEED_FRAMES,
};
use super::fixtures::{cache, fmt};

#[test]
fn format_cache_conversion_accepts_float32() {
    assert_eq!(
        render_format_from_wasapi_cache_for_transport_boundary(&cache()),
        Ok(fmt())
    );
}

#[test]
fn format_cache_conversion_rejects_other_format() {
    let mut cache = cache();
    cache.format_tag = 1;
    assert_eq!(
        render_format_from_wasapi_cache_for_transport_boundary(&cache),
        Err(RenderRingBufferBoundaryError::UnsupportedFormat)
    );
}

#[test]
fn build_synthetic_source_rejects_seed_above_limit() {
    assert_eq!(
        build_synthetic_zero_ring_buffer_source(fmt(), MAX_SYNTHETIC_ZERO_SEED_FRAMES + 1)
            .unwrap_err(),
        RenderRingBufferBoundaryError::SyntheticSeedFrameCountTooLarge
    );
}
