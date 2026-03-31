use std::path::PathBuf;

use crate::audio::errors::AudioResult;

use super::{dll_set::FfmpegDllSet, dynlib::DynLib, ffi};

#[derive(Debug, Clone, Default)]
pub struct FfmpegLoadOptions {
    /// Optional directory containing FFmpeg DLLs.
    /// If not provided, env KIVO_FFMPEG_DIR is checked, then OS search.
    pub dll_dir: Option<PathBuf>,
}

pub struct FfmpegApi {
    // Keep DLLs alive for the lifetime of all function pointers.
    _dlls: FfmpegDllSet,

    pub avformat_open_input: ffi::avformat_open_input_fn,
    pub avformat_find_stream_info: ffi::avformat_find_stream_info_fn,
    pub avformat_close_input: ffi::avformat_close_input_fn,
    pub av_read_frame: ffi::av_read_frame_fn,
    pub av_seek_frame: ffi::av_seek_frame_fn,
    pub av_find_best_stream: ffi::av_find_best_stream_fn,

    pub avcodec_alloc_context3: ffi::avcodec_alloc_context3_fn,
    pub avcodec_open2: ffi::avcodec_open2_fn,
    pub avcodec_free_context: ffi::avcodec_free_context_fn,
    pub avcodec_send_packet: ffi::avcodec_send_packet_fn,
    pub avcodec_receive_frame: ffi::avcodec_receive_frame_fn,
    pub avcodec_flush_buffers: ffi::avcodec_flush_buffers_fn,

    pub av_packet_alloc: ffi::av_packet_alloc_fn,
    pub av_packet_unref: ffi::av_packet_unref_fn,
    pub av_packet_free: ffi::av_packet_free_fn,

    pub av_frame_alloc: ffi::av_frame_alloc_fn,
    pub av_frame_unref: ffi::av_frame_unref_fn,
    pub av_frame_free: ffi::av_frame_free_fn,

    pub av_get_bytes_per_sample: ffi::av_get_bytes_per_sample_fn,
    pub av_sample_fmt_is_planar: ffi::av_sample_fmt_is_planar_fn,
    pub av_get_default_channel_layout: ffi::av_get_default_channel_layout_fn,
    pub av_get_sample_fmt_name: ffi::av_get_sample_fmt_name_fn,

    pub swr_alloc_set_opts: ffi::swr_alloc_set_opts_fn,
    pub swr_init: ffi::swr_init_fn,
    pub swr_free: ffi::swr_free_fn,
    pub swr_get_out_samples: ffi::swr_get_out_samples_fn,
    pub swr_convert: ffi::swr_convert_fn,

    pub av_strerror: ffi::av_strerror_fn,

    pub ffmpeg_version: String,
}

impl FfmpegApi {
    pub fn load(opts: &FfmpegLoadOptions) -> AudioResult<Self> {
        let (dlls, report) = FfmpegDllSet::load(opts.dll_dir.clone())?;

        unsafe fn sym<T: Copy>(lib: &DynLib, name: &str) -> AudioResult<T> {
            lib.sym(name)
        }
        unsafe fn sym_any<T: Copy>(dlls: &FfmpegDllSet, name: &str) -> AudioResult<T> {
            dlls.avformat
                .sym(name)
                .or_else(|_| dlls.avcodec.sym(name))
                .or_else(|_| dlls.avutil.sym(name))
                .or_else(|_| dlls.swresample.sym(name))
        }

        let ffmpeg_version = unsafe {
            let ver_fn: unsafe extern "C" fn() -> *const i8 = sym(&dlls.avutil, "av_version_info")?;
            let p = ver_fn();
            if p.is_null() {
                "unknown".to_string()
            } else {
                std::ffi::CStr::from_ptr(p).to_string_lossy().to_string()
            }
        };

        maybe_print_smoke_probe(&ffmpeg_version, report.resolved_dir.as_deref());

        let api = unsafe {
            Self {
                avformat_open_input: sym(&dlls.avformat, "avformat_open_input")?,
                avformat_find_stream_info: sym(&dlls.avformat, "avformat_find_stream_info")?,
                avformat_close_input: sym(&dlls.avformat, "avformat_close_input")?,
                av_read_frame: sym(&dlls.avformat, "av_read_frame")?,
                av_seek_frame: sym(&dlls.avformat, "av_seek_frame")?,
                av_find_best_stream: sym(&dlls.avformat, "av_find_best_stream")?,

                avcodec_alloc_context3: sym(&dlls.avcodec, "avcodec_alloc_context3")?,
                avcodec_open2: sym(&dlls.avcodec, "avcodec_open2")?,
                avcodec_free_context: sym(&dlls.avcodec, "avcodec_free_context")?,
                avcodec_send_packet: sym(&dlls.avcodec, "avcodec_send_packet")?,
                avcodec_receive_frame: sym(&dlls.avcodec, "avcodec_receive_frame")?,
                avcodec_flush_buffers: sym(&dlls.avcodec, "avcodec_flush_buffers")?,

                av_packet_alloc: sym_any(&dlls, "av_packet_alloc")?,
                av_packet_unref: sym_any(&dlls, "av_packet_unref")?,
                av_packet_free: sym_any(&dlls, "av_packet_free")?,

                av_frame_alloc: sym_any(&dlls, "av_frame_alloc")?,
                av_frame_unref: sym_any(&dlls, "av_frame_unref")?,
                av_frame_free: sym_any(&dlls, "av_frame_free")?,

                av_get_bytes_per_sample: sym_any(&dlls, "av_get_bytes_per_sample")?,
                av_sample_fmt_is_planar: sym_any(&dlls, "av_sample_fmt_is_planar")?,
                av_get_default_channel_layout: sym_any(&dlls, "av_get_default_channel_layout")?,
                av_get_sample_fmt_name: sym_any(&dlls, "av_get_sample_fmt_name")?,

                swr_alloc_set_opts: sym(&dlls.swresample, "swr_alloc_set_opts")?,
                swr_init: sym(&dlls.swresample, "swr_init")?,
                swr_free: sym(&dlls.swresample, "swr_free")?,
                swr_get_out_samples: sym(&dlls.swresample, "swr_get_out_samples")?,
                swr_convert: sym(&dlls.swresample, "swr_convert")?,

                av_strerror: sym(&dlls.avutil, "av_strerror")?,

                ffmpeg_version,
                _dlls: dlls,
            }
        };

        Ok(api)
    }

    pub fn error_string(&self, code: i32) -> String {
        let mut buf = [0i8; 256];
        let r = unsafe { (self.av_strerror)(code, buf.as_mut_ptr(), buf.len()) };
        if r < 0 {
            return format!("ffmpeg_error({code})");
        }
        unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) }
            .to_string_lossy()
            .to_string()
    }

    pub fn version_info(&self) -> &str {
        &self.ffmpeg_version
    }
}

fn maybe_print_smoke_probe(version_info: &str, resolved_dir: Option<&std::path::Path>) {
    if !is_ffmpeg_smoke_process() {
        return;
    }

    println!("ffmpeg_version_info={version_info}");

    let dir = resolved_dir
        .map(|p| p.display().to_string())
        .unwrap_or_default();
    println!("ffmpeg_resolved_dir={dir}");
}

fn is_ffmpeg_smoke_process() -> bool {
    let arg0 = match std::env::args().next() {
        Some(v) => v,
        None => return false,
    };

    let name = std::path::Path::new(&arg0)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(&arg0);

    name.to_ascii_lowercase().contains("ffmpeg_smoke")
}
