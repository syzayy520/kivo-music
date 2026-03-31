use super::{
    api::FfmpegApi,
    decoder::FfmpegDecoder,
    ffi,
    resample_frame::{diag, frame_input_ptrs, infer_channels},
    types::PcmChunkF32,
};
use crate::audio::errors::{AudioError, AudioResult};
use std::rc::Rc;

const OUT_SAMPLE_FMT: i32 = ffi::AV_SAMPLE_FMT_FLT; // f32 packed (interleaved)

pub(super) struct FfmpegResampler {
    api: Rc<FfmpegApi>,
    ctx: *mut ffi::SwrContext,
    in_fmt: i32,
    channels: u16,
    sample_rate: i32,
}

impl FfmpegResampler {
    pub fn new(
        api: Rc<FfmpegApi>,
        in_fmt: i32,
        channels: u16,
        sample_rate: i32,
    ) -> AudioResult<Self> {
        if channels == 0 || sample_rate <= 0 {
            return Err(AudioError::unsupported(format!(
                "invalid audio params: ch={channels} sr={sample_rate}"
            )));
        }
        let ch_layout = unsafe { (api.av_get_default_channel_layout)(channels as i32) };
        if ch_layout == 0 {
            return Err(AudioError::unsupported(format!(
                "unsupported channel layout: ch={channels}"
            )));
        }

        let mut ctx = unsafe {
            (api.swr_alloc_set_opts)(
                std::ptr::null_mut(),
                ch_layout,
                OUT_SAMPLE_FMT,
                sample_rate,
                ch_layout,
                in_fmt,
                sample_rate,
                0,
                std::ptr::null_mut(),
            )
        };
        if ctx.is_null() {
            return Err(AudioError::unsupported(format!(
                "swr_alloc_set_opts returned null ({})",
                diag(api.as_ref(), in_fmt, channels, sample_rate)
            )));
        }

        let r = unsafe { (api.swr_init)(ctx) };
        if r < 0 {
            unsafe { (api.swr_free)(&mut ctx) };
            return Err(AudioError::FfmpegApiError {
                context: "swr_init",
                code: r,
                message: format!(
                    "{} ({})",
                    api.error_string(r),
                    diag(api.as_ref(), in_fmt, channels, sample_rate)
                ),
            });
        }

        Ok(Self {
            api,
            ctx,
            in_fmt,
            channels,
            sample_rate,
        })
    }

    pub fn matches(&self, in_fmt: i32, channels: u16, sample_rate: i32) -> bool {
        self.in_fmt == in_fmt && self.channels == channels && self.sample_rate == sample_rate
    }

    pub fn convert_frame(&mut self, frame: *const ffi::AVFrame) -> AudioResult<PcmChunkF32> {
        let nb = unsafe { (*frame).nb_samples.max(0) };
        if nb == 0 {
            return Ok(PcmChunkF32 {
                channels: self.channels,
                data: Vec::new(),
            });
        }

        let out_cap = unsafe { (self.api.swr_get_out_samples)(self.ctx, nb) };
        if out_cap <= 0 {
            return Err(AudioError::unsupported(format!(
                "swr_get_out_samples={out_cap} (in={nb}) ({})",
                diag(
                    self.api.as_ref(),
                    self.in_fmt,
                    self.channels,
                    self.sample_rate
                )
            )));
        }

        let ch = self.channels as usize;
        let mut out: Vec<f32> = vec![0.0; (out_cap as usize) * ch];
        let mut out_ptrs: [*mut u8; 1] = [out.as_mut_ptr() as *mut u8];
        let in_ptrs = frame_input_ptrs(&self.api, frame, self.in_fmt, ch)?;

        let r = unsafe {
            (self.api.swr_convert)(
                self.ctx,
                out_ptrs.as_mut_ptr(),
                out_cap,
                in_ptrs.as_ptr(),
                nb,
            )
        };
        if r < 0 {
            return Err(AudioError::FfmpegApiError {
                context: "swr_convert",
                code: r,
                message: format!(
                    "{} ({})",
                    self.api.error_string(r),
                    diag(
                        self.api.as_ref(),
                        self.in_fmt,
                        self.channels,
                        self.sample_rate
                    )
                ),
            });
        }

        out.truncate((r as usize) * ch);
        Ok(PcmChunkF32 {
            channels: self.channels,
            data: out,
        })
    }
}

impl Drop for FfmpegResampler {
    fn drop(&mut self) {
        unsafe {
            if !self.ctx.is_null() {
                (self.api.swr_free)(&mut self.ctx);
            }
        }
    }
}

pub(super) fn frame_to_pcm_f32(dec: &mut FfmpegDecoder) -> AudioResult<PcmChunkF32> {
    let frame = dec.frame as *const ffi::AVFrame;
    let in_fmt = unsafe { (*frame).format };
    let channels = infer_channels(dec.api.as_ref(), frame, in_fmt)? as u16;
    let sr = dec.sample_rate_hint.max(1);

    if dec
        .resampler
        .as_ref()
        .is_none_or(|r| !r.matches(in_fmt, channels, sr))
    {
        dec.resampler = Some(FfmpegResampler::new(dec.api.clone(), in_fmt, channels, sr)?);
    }

    let r = dec
        .resampler
        .as_mut()
        .ok_or_else(|| AudioError::unsupported("resampler not initialized"))?;
    r.convert_frame(frame).map_err(|e| match e {
        AudioError::FfmpegApiError { .. } | AudioError::FfmpegSymbolMissing { .. } => e,
        _ => AudioError::unsupported(format!(
            "{e} ({})",
            diag(dec.api.as_ref(), in_fmt, channels, sr)
        )),
    })
}
