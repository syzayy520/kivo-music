use std::{path::Path, rc::Rc};

use crate::audio::errors::{AudioError, AudioResult};

use super::{
    api::{FfmpegApi, FfmpegLoadOptions},
    ffi,
    resample::FfmpegResampler,
    types::PcmChunkF32,
};

mod open;
mod read;

pub struct FfmpegDecoder {
    pub(super) api: Rc<FfmpegApi>,
    pub(super) fmt: *mut ffi::AVFormatContext,
    pub(super) codec_ctx: *mut ffi::AVCodecContext,
    pub(super) pkt: *mut ffi::AVPacket,
    pub(super) frame: *mut ffi::AVFrame,
    pub(super) audio_stream_index: i32,
    pub(super) sent_eof: bool,
    pub(super) frames_emitted: u64,

    /// Best-effort hint for sample-rate; used for configuring the format-conversion resampler.
    ///
    /// Note: the WAV writer currently uses `header_probe` separately. In later iterations,
    /// sample-rate should come from decoded stream metadata.
    pub(super) sample_rate_hint: i32,

    /// Lazily created when we see the first decoded frame.
    pub(super) resampler: Option<FfmpegResampler>,
}

impl FfmpegDecoder {
    pub fn open(path: &Path, opt: &FfmpegLoadOptions) -> AudioResult<Self> {
        open::open(path, opt)
    }

    pub fn ffmpeg_version(&self) -> String {
        self.api.version_info().to_string()
    }

    pub fn frames_emitted(&self) -> u64 {
        self.frames_emitted
    }

    pub fn seek_seconds(&mut self, seconds: f64) -> AudioResult<()> {
        if seconds <= 0.0 {
            return Ok(());
        }

        let ts = (seconds * ffi::AV_TIME_BASE as f64) as i64;
        let r = unsafe { (self.api.av_seek_frame)(self.fmt, -1, ts, ffi::AVSEEK_FLAG_BACKWARD) };
        if r < 0 {
            return Err(AudioError::FfmpegApiError {
                context: "av_seek_frame",
                code: r,
                message: self.api.error_string(r),
            });
        }

        unsafe { (self.api.avcodec_flush_buffers)(self.codec_ctx) };
        self.sent_eof = false;

        // SwrContext may keep delay; reset on seek to avoid producing tail samples.
        self.resampler = None;
        Ok(())
    }

    pub fn next_pcm_f32(&mut self) -> AudioResult<Option<PcmChunkF32>> {
        read::next_pcm_f32(self)
    }
}

impl Drop for FfmpegDecoder {
    fn drop(&mut self) {
        // Ensure resampler is freed while FFmpeg DLLs are still alive.
        self.resampler.take();

        unsafe {
            if !self.frame.is_null() {
                (self.api.av_frame_free)(&mut self.frame);
            }
            if !self.pkt.is_null() {
                (self.api.av_packet_free)(&mut self.pkt);
            }
            if !self.codec_ctx.is_null() {
                (self.api.avcodec_free_context)(&mut self.codec_ctx);
            }
            if !self.fmt.is_null() {
                (self.api.avformat_close_input)(&mut self.fmt);
            }
        }
    }
}
