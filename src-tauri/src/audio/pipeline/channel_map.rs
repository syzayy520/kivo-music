//! Simple channel mapping helpers.
//!
//! These are intentionally small and allocation-free: caller provides the buffers.

pub(crate) fn map_mono_to_stereo(src: &[f32], dst: &mut [f32], frames: usize) {
    debug_assert!(dst.len() >= frames * 2);

    for (i, &x) in src.iter().take(frames).enumerate() {
        let base = i * 2;
        dst[base] = x;
        dst[base + 1] = x;
    }
}

pub(crate) fn map_stereo_to_mono(src: &[f32], dst: &mut [f32], frames: usize) {
    debug_assert!(src.len() >= frames * 2);
    debug_assert!(dst.len() >= frames);

    for (i, chunk) in src.chunks_exact(2).take(frames).enumerate() {
        let l = chunk[0];
        let r = chunk[1];
        dst[i] = (l + r) * 0.5;
    }
}

/// Map interleaved samples between channel counts, writing into `dst`.
///
/// - `src` is interleaved f32 samples.
/// - `src_ch`/`dst_ch` are channel counts.
/// - `dst` is resized as needed and overwritten.
///
/// This is intentionally pragmatic: it handles the common 1<->2 cases with the
/// dedicated helpers, and falls back to simple average/replicate mapping for
/// other channel layouts.
pub(crate) fn map_channels_interleaved(
    src: &[f32],
    src_ch: usize,
    dst_ch: usize,
    dst: &mut Vec<f32>,
) {
    if src_ch == 0 || dst_ch == 0 {
        dst.clear();
        return;
    }

    let frames = src.len() / src_ch;
    let src = &src[..frames.saturating_mul(src_ch)];

    dst.resize(frames.saturating_mul(dst_ch), 0.0);
    let out = &mut dst[..];

    match (src_ch, dst_ch) {
        (1, 1) => {
            out[..frames].copy_from_slice(&src[..frames]);
        }
        (1, 2) => {
            map_mono_to_stereo(src, out, frames);
        }
        (2, 1) => {
            map_stereo_to_mono(src, out, frames);
        }
        (2, 2) => {
            out[..frames * 2].copy_from_slice(&src[..frames * 2]);
        }
        (s, d) if s == d => {
            out[..frames * s].copy_from_slice(&src[..frames * s]);
        }
        (s, 1) => {
            // Downmix N -> mono by simple average.
            for (frame_idx, out_s) in out.iter_mut().take(frames).enumerate() {
                let base = frame_idx * s;
                let mut acc = 0.0f32;
                for ch in 0..s {
                    acc += src.get(base + ch).copied().unwrap_or(0.0);
                }
                *out_s = acc / (s as f32);
            }
        }
        (1, d) => {
            // Upmix mono -> N by replication.
            for (frame_idx, mono) in src.iter().take(frames).enumerate() {
                let base = frame_idx * d;
                for ch in 0..d {
                    out[base + ch] = *mono;
                }
            }
        }
        (s, d) => {
            // Generic mapping: copy min channels, fill extra with 0.
            let min_ch = s.min(d);
            for frame_idx in 0..frames {
                let src_base = frame_idx * s;
                let dst_base = frame_idx * d;
                for ch in 0..min_ch {
                    out[dst_base + ch] = src.get(src_base + ch).copied().unwrap_or(0.0);
                }
                for ch in min_ch..d {
                    out[dst_base + ch] = 0.0;
                }
            }
        }
    }
}
