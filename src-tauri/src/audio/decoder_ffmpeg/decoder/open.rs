use std::{ffi::CString, path::Path, rc::Rc};

use crate::audio::errors::{AudioError, AudioResult};

use super::super::{
    api::{FfmpegApi, FfmpegLoadOptions},
    ffi, header_probe,
};

use super::FfmpegDecoder;

pub fn open(path: &Path, opt: &FfmpegLoadOptions) -> AudioResult<FfmpegDecoder> {
    let api = Rc::new(FfmpegApi::load(opt)?);

    let c_path = CString::new(path.to_string_lossy().as_bytes())
        .map_err(|_| AudioError::InvalidInput("input path contains NUL byte".to_string()))?;

    let mut fmt: *mut ffi::AVFormatContext = std::ptr::null_mut();
    let r = unsafe {
        (api.avformat_open_input)(
            &mut fmt,
            c_path.as_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    if r < 0 {
        return Err(AudioError::FfmpegApiError {
            context: "avformat_open_input",
            code: r,
            message: api.error_string(r),
        });
    }

    let r = unsafe { (api.avformat_find_stream_info)(fmt, std::ptr::null_mut()) };
    if r < 0 {
        unsafe { (api.avformat_close_input)(&mut fmt) };
        return Err(AudioError::FfmpegApiError {
            context: "avformat_find_stream_info",
            code: r,
            message: api.error_string(r),
        });
    }

    let mut codec: *const ffi::AVCodec = std::ptr::null();
    let idx =
        unsafe { (api.av_find_best_stream)(fmt, ffi::AVMEDIA_TYPE_AUDIO, -1, -1, &mut codec, 0) };
    if idx < 0 || codec.is_null() {
        unsafe { (api.avformat_close_input)(&mut fmt) };
        return Err(AudioError::unsupported("no audio stream found"));
    }

    let mut codec_ctx = unsafe { (api.avcodec_alloc_context3)(codec) };
    if codec_ctx.is_null() {
        unsafe { (api.avformat_close_input)(&mut fmt) };
        return Err(AudioError::unsupported(
            "avcodec_alloc_context3 returned null",
        ));
    }

    let r = unsafe { (api.avcodec_open2)(codec_ctx, codec, std::ptr::null_mut()) };
    if r < 0 {
        unsafe {
            (api.avcodec_free_context)(&mut codec_ctx);
            (api.avformat_close_input)(&mut fmt);
        }
        return Err(AudioError::FfmpegApiError {
            context: "avcodec_open2",
            code: r,
            message: api.error_string(r),
        });
    }

    let mut pkt = unsafe { (api.av_packet_alloc)() };
    if pkt.is_null() {
        unsafe {
            (api.avcodec_free_context)(&mut codec_ctx);
            (api.avformat_close_input)(&mut fmt);
        }
        return Err(AudioError::unsupported("av_packet_alloc returned null"));
    }

    let frame = unsafe { (api.av_frame_alloc)() };
    if frame.is_null() {
        unsafe {
            (api.av_packet_free)(&mut pkt);
            (api.avcodec_free_context)(&mut codec_ctx);
            (api.avformat_close_input)(&mut fmt);
        }
        return Err(AudioError::unsupported("av_frame_alloc returned null"));
    }

    Ok(FfmpegDecoder {
        api,
        fmt,
        codec_ctx,
        pkt,
        frame,
        audio_stream_index: idx,
        sent_eof: false,
        frames_emitted: 0,

        sample_rate_hint: header_probe::probe_sample_rate(path).unwrap_or(44_100) as i32,

        resampler: None,
    })
}
