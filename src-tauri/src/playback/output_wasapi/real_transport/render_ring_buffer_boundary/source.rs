use super::constants::MAX_SYNTHETIC_ZERO_SEED_FRAMES;
use super::error::{map_seed_error, RenderRingBufferBoundaryError};
use super::format::ring_format;
use crate::playback::output_wasapi::pcm_adapter::{validate_render_format, PcmRenderFormat};
use crate::playback::output_wasapi::ring_buffer::RingBuffer;

pub(crate) fn build_synthetic_zero_ring_buffer_source(
    render_format: PcmRenderFormat,
    seed_frames: u32,
) -> Result<RingBuffer, RenderRingBufferBoundaryError> {
    if seed_frames > MAX_SYNTHETIC_ZERO_SEED_FRAMES {
        return Err(RenderRingBufferBoundaryError::SyntheticSeedFrameCountTooLarge);
    }

    let render_format =
        validate_render_format(render_format).map_err(RenderRingBufferBoundaryError::PcmAdapter)?;
    let mut source =
        RingBuffer::new(ring_format(render_format), seed_frames.max(1)).map_err(map_seed_error)?;

    if seed_frames > 0 {
        let bytes = vec![0_u8; checked_byte_len(seed_frames, render_format.block_align)?];
        let written = source.write_frames(&bytes).map_err(map_seed_error)?;
        if written != seed_frames {
            return Err(RenderRingBufferBoundaryError::SyntheticSourceSeedFailed(
                format!("requested={seed_frames:?}; written={written:?}"),
            ));
        }
    }

    Ok(source)
}

fn checked_byte_len(frames: u32, block_align: u16) -> Result<usize, RenderRingBufferBoundaryError> {
    usize::try_from(frames)
        .ok()
        .and_then(|frames| frames.checked_mul(usize::from(block_align)))
        .ok_or(RenderRingBufferBoundaryError::ByteLengthOverflow)
}
