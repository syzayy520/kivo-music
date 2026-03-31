use crate::audio::errors::{AudioError, AudioResult};

use super::{ffi, types::PcmChunkF32};

/// Convert an `AVFrame` with planar/packed float to interleaved f32.
///
/// P0 assumptions:
/// - We only accept `f32` sample format.
/// - Channel count is inferred from non-null `frame.data[...]` pointers (up to 8).
pub unsafe fn frame_to_f32(
    frame: *const ffi::AVFrame,
    bytes_per_sample: ffi::c_int,
    is_planar: bool,
) -> AudioResult<PcmChunkF32> {
    if frame.is_null() {
        return Err(AudioError::InvalidInput("frame is null".to_string()));
    }
    if bytes_per_sample != 4 {
        return Err(AudioError::unsupported(format!(
            "only f32 is supported (bytes_per_sample={bytes_per_sample})"
        )));
    }

    let nb_samples = (*frame).nb_samples.max(0) as usize;
    let channels = infer_channels(frame, nb_samples, bytes_per_sample, is_planar);
    if nb_samples == 0 || channels == 0 {
        return Ok(PcmChunkF32 {
            data: Vec::new(),
            channels: channels.max(1) as u16,
        });
    }

    if is_planar {
        let mut out = vec![0f32; nb_samples * channels];
        for ch in 0..channels {
            let src = (*frame).data[ch];
            if src.is_null() {
                continue;
            }
            let plane = std::slice::from_raw_parts(src as *const f32, nb_samples);
            for i in 0..nb_samples {
                out[i * channels + ch] = plane[i];
            }
        }
        Ok(PcmChunkF32 {
            data: out,
            channels: channels as u16,
        })
    } else {
        let src = (*frame).data[0];
        if src.is_null() {
            return Ok(PcmChunkF32 {
                data: Vec::new(),
                channels: channels as u16,
            });
        }
        let total = nb_samples * channels;
        let slice = std::slice::from_raw_parts(src as *const f32, total);
        Ok(PcmChunkF32 {
            data: slice.to_vec(),
            channels: channels as u16,
        })
    }
}

fn infer_channels(
    frame: *const ffi::AVFrame,
    nb_samples: usize,
    bps: ffi::c_int,
    planar: bool,
) -> usize {
    if planar {
        // For planar audio, FFmpeg typically populates data[0..channels-1].
        let mut n = 0usize;
        unsafe {
            for p in (*frame).data.iter() {
                if p.is_null() {
                    break;
                }
                n += 1;
            }
        }
        return n.clamp(1, 8);
    }

    // Packed/interleaved audio: infer channels from line size.
    // linesize[0] is bytes for the whole interleaved buffer.
    if nb_samples == 0 || bps <= 0 {
        return 1;
    }
    let bytes = unsafe { (*frame).linesize[0] } as usize;
    let denom = nb_samples * (bps as usize);
    let ch = if denom == 0 { 1 } else { bytes / denom };
    ch.clamp(1, 8)
}
