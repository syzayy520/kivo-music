use std::ffi::CStr;

use crate::audio::errors::{AudioError, AudioResult};

use super::{api::FfmpegApi, ffi};

pub(super) fn diag(api: &FfmpegApi, fmt: i32, ch: u16, sr: i32) -> String {
    let layout = unsafe { (api.av_get_default_channel_layout)(ch as i32) };
    format!(
        "in_fmt={} out_fmt=f32_packed ch={ch} sr={sr} layout=0x{layout:x}",
        sample_fmt_name(api, fmt)
    )
}

pub(super) fn sample_fmt_name(api: &FfmpegApi, fmt: i32) -> String {
    let p = unsafe { (api.av_get_sample_fmt_name)(fmt) };
    if p.is_null() {
        return format!("fmt#{fmt}");
    }
    unsafe { CStr::from_ptr(p) }
        .to_str()
        .unwrap_or("(invalid utf8)")
        .to_string()
}

pub(super) fn infer_channels(
    api: &FfmpegApi,
    frame: *const ffi::AVFrame,
    fmt: i32,
) -> AudioResult<usize> {
    let bps = unsafe { (api.av_get_bytes_per_sample)(fmt) };
    if bps <= 0 {
        return Err(AudioError::unsupported(format!(
            "unsupported fmt={} (bps={bps})",
            sample_fmt_name(api, fmt)
        )));
    }
    let planar = unsafe { (api.av_sample_fmt_is_planar)(fmt) } != 0;
    if planar {
        unsafe {
            if !(*frame).extended_data.is_null()
                && ((*frame).extended_data as *const *const u8)
                    != ((*frame).data.as_ptr() as *const *const u8)
            {
                return Err(AudioError::unsupported("planar >8ch frame not supported"));
            }
            let mut ch = 0usize;
            for i in 0..8 {
                if (*frame).data[i].is_null() {
                    break;
                }
                ch += 1;
            }
            if ch == 0 {
                return Err(AudioError::unsupported("planar frame has no planes"));
            }
            Ok(ch)
        }
    } else {
        let nb = unsafe { (*frame).nb_samples.max(0) as usize };
        if nb == 0 {
            return Ok(1);
        }
        let bytes = unsafe { (*frame).linesize[0].max(0) as usize };
        let denom = nb.saturating_mul(bps as usize);
        if bytes == 0 || denom == 0 {
            return Err(AudioError::unsupported(format!(
                "packed frame missing size: nb={nb} bytes={bytes} bps={bps}"
            )));
        }
        if bytes % denom != 0 {
            return Err(AudioError::unsupported(format!(
                "cannot infer channels (packed): bytes={bytes} nb={nb} bps={bps}"
            )));
        }
        let ch = bytes / denom;
        if ch == 0 || ch > 8 {
            return Err(AudioError::unsupported(format!(
                "unsupported channels inferred={ch}"
            )));
        }
        Ok(ch)
    }
}

pub(super) fn frame_input_ptrs(
    api: &FfmpegApi,
    frame: *const ffi::AVFrame,
    fmt: i32,
    channels: usize,
) -> AudioResult<Vec<*const u8>> {
    let planar = unsafe { (api.av_sample_fmt_is_planar)(fmt) } != 0;
    unsafe {
        if planar {
            if channels > 8 {
                return Err(AudioError::unsupported("planar >8ch frame not supported"));
            }
            let mut v = Vec::with_capacity(channels);
            for i in 0..channels {
                let p = (*frame).data[i] as *const u8;
                if p.is_null() {
                    return Err(AudioError::unsupported(format!(
                        "planar plane missing: ch={i} total={channels}"
                    )));
                }
                v.push(p);
            }
            Ok(v)
        } else {
            let p = (*frame).data[0] as *const u8;
            if p.is_null() {
                return Err(AudioError::unsupported("packed frame data[0] is null"));
            }
            Ok(vec![p])
        }
    }
}
