use super::super::render_ring_buffer_boundary::{
    RenderRingBufferBoundaryConfig, RenderRingBufferBoundaryError,
};
use crate::playback::output_wasapi::pcm_adapter::{PcmRenderFormat, PcmSampleFormat};
use crate::playback::output_wasapi::wasapi_context::{
    WasapiFormatCache, WasapiPaddingStateSnapshot, WasapiRenderWriteReport,
};

pub(super) fn fmt() -> PcmRenderFormat {
    PcmRenderFormat {
        sample_rate_hz: 44_100,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
        sample_format: PcmSampleFormat::Float32Interleaved,
    }
}

pub(super) fn cache() -> WasapiFormatCache {
    WasapiFormatCache {
        sample_rate_hz: 44_100,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
        avg_bytes_per_sec: 352_800,
        format_tag: 3,
        cb_size: 0,
    }
}

pub(super) fn snap(available: u32) -> WasapiPaddingStateSnapshot {
    WasapiPaddingStateSnapshot {
        buffer_frame_capacity: 8,
        current_padding_frames: 8 - available,
        available_frames: available,
    }
}

pub(super) fn report(frames: u32, bytes: u32) -> WasapiRenderWriteReport {
    WasapiRenderWriteReport {
        frames_written: frames,
        bytes_written: bytes,
        used_silent_flag: false,
        sample_rate_hz: 44_100,
        channels: 2,
    }
}

pub(super) fn cfg(
    enabled: bool,
    iterations: u32,
    max_frames: u32,
    seed: u32,
) -> RenderRingBufferBoundaryConfig {
    RenderRingBufferBoundaryConfig {
        enabled,
        iterations,
        max_frames_per_write: max_frames,
        synthetic_zero_seed_frames: seed,
    }
}

pub(super) fn assert_err<T: std::fmt::Debug>(
    actual: Result<T, RenderRingBufferBoundaryError>,
    expected: RenderRingBufferBoundaryError,
) {
    assert_eq!(actual.unwrap_err(), expected);
}
