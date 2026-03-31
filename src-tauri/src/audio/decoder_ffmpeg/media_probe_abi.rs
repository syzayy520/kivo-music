use std::{ffi::CStr, ptr};

use crate::audio::errors::AudioResult;

use super::{dll_set::FfmpegDllSet, ffi};

pub(super) struct CloseInputGuard<'a> {
    pub(super) api: &'a ProbeApi,
    pub(super) fmt: *mut ffi::AVFormatContext,
}

impl Drop for CloseInputGuard<'_> {
    fn drop(&mut self) {
        unsafe {
            let mut p = self.fmt;
            (self.api.avformat_close_input)(&mut p);
        }
    }
}

#[repr(C)]
pub(super) struct AVDictionaryEntry {
    #[allow(dead_code)]
    pub(super) key: *const i8,
    pub(super) value: *const i8,
}

pub(super) type AvDictGetFn = unsafe extern "C" fn(
    m: *mut ffi::AVDictionary,
    key: *const i8,
    prev: *const AVDictionaryEntry,
    flags: i32,
) -> *mut AVDictionaryEntry;

#[repr(C)]
pub(super) struct AVRational {
    pub(super) num: i32,
    pub(super) den: i32,
}

#[repr(C)]
pub(super) struct AVFormatContextView {
    pub(super) av_class: *const core::ffi::c_void,
    pub(super) iformat: *const core::ffi::c_void,
    pub(super) oformat: *const core::ffi::c_void,
    pub(super) priv_data: *mut core::ffi::c_void,
    pub(super) pb: *mut core::ffi::c_void,
    pub(super) ctx_flags: i32,
    pub(super) nb_streams: u32,
    pub(super) streams: *mut *mut core::ffi::c_void,
    pub(super) url: *mut i8,
    pub(super) start_time: i64,
    pub(super) duration: i64,
    pub(super) bit_rate: i64,
    pub(super) packet_size: u32,
    pub(super) max_delay: i32,
    pub(super) flags: i32,
    pub(super) probesize: i64,
    pub(super) max_analyze_duration: i64,
    pub(super) key: *const u8,
    pub(super) keylen: i32,
    pub(super) nb_programs: u32,
    pub(super) programs: *mut core::ffi::c_void,
    pub(super) video_codec_id: i32,
    pub(super) audio_codec_id: i32,
    pub(super) subtitle_codec_id: i32,
    pub(super) max_index_size: u32,
    pub(super) max_picture_buffer: u32,
    pub(super) nb_chapters: u32,
    pub(super) chapters: *mut core::ffi::c_void,
    pub(super) metadata: *mut ffi::AVDictionary,
}

#[repr(C)]
pub(super) struct AVStreamView {
    pub(super) index: i32,
    pub(super) id: i32,
    pub(super) codecpar: *mut core::ffi::c_void,
    pub(super) priv_data: *mut core::ffi::c_void,
    pub(super) time_base: AVRational,
    pub(super) start_time: i64,
    pub(super) duration: i64,
    pub(super) nb_frames: i64,
    pub(super) disposition: i32,
    pub(super) discard: i32,
    pub(super) sample_aspect_ratio: AVRational,
    pub(super) metadata: *mut ffi::AVDictionary,
}

pub(super) struct ProbeApi {
    pub(super) avformat_open_input: ffi::avformat_open_input_fn,
    pub(super) avformat_find_stream_info: ffi::avformat_find_stream_info_fn,
    pub(super) avformat_close_input: ffi::avformat_close_input_fn,
    pub(super) av_find_best_stream: ffi::av_find_best_stream_fn,
    pub(super) av_strerror: ffi::av_strerror_fn,
    pub(super) av_dict_get: AvDictGetFn,
}

impl ProbeApi {
    pub(super) unsafe fn load(dlls: &FfmpegDllSet) -> AudioResult<Self> {
        Ok(Self {
            avformat_open_input: dlls.avformat.sym("avformat_open_input")?,
            avformat_find_stream_info: dlls.avformat.sym("avformat_find_stream_info")?,
            avformat_close_input: dlls.avformat.sym("avformat_close_input")?,
            av_find_best_stream: dlls.avformat.sym("av_find_best_stream")?,
            av_strerror: dlls.avutil.sym("av_strerror")?,
            av_dict_get: dlls.avutil.sym("av_dict_get")?,
        })
    }

    pub(super) fn api_error(&self, ctx: &'static str, code: i32) -> String {
        let mut buf = [0i8; 256];
        let r = unsafe { (self.av_strerror)(code, buf.as_mut_ptr(), buf.len()) };
        let msg = if r < 0 {
            format!("ffmpeg_error({code})")
        } else {
            unsafe { CStr::from_ptr(buf.as_ptr()) }
                .to_string_lossy()
                .to_string()
        };
        format!("{ctx}: {msg} (code={code})")
    }
}

pub(super) unsafe fn open_and_probe_format(
    api: &ProbeApi,
    c_path: *const i8,
) -> Result<*mut ffi::AVFormatContext, String> {
    let mut fmt: *mut ffi::AVFormatContext = ptr::null_mut();
    let r = (api.avformat_open_input)(&mut fmt, c_path, ptr::null_mut(), ptr::null_mut());
    if r < 0 {
        return Err(api.api_error("avformat_open_input", r));
    }

    let r = (api.avformat_find_stream_info)(fmt, ptr::null_mut());
    if r < 0 {
        return Err(api.api_error("avformat_find_stream_info", r));
    }

    Ok(fmt)
}
