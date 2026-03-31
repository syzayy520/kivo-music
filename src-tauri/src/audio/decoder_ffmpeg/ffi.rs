#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::{c_char, c_void};

pub type c_int = i32;
pub type int64_t = i64;
pub type uint8_t = u8;

pub const AV_TIME_BASE: i64 = 1_000_000;

// FFmpeg enums (subset)
pub const AVMEDIA_TYPE_AUDIO: c_int = 1;

// These constant values are stable in FFmpeg releases.
pub const AVERROR_EOF: c_int = -541478725;
pub const AVERROR_EAGAIN: c_int = -11;

pub const AVSEEK_FLAG_BACKWARD: c_int = 1;

// Opaque types
#[repr(C)]
pub struct AVFormatContext {
    _private: [u8; 0],
}

#[repr(C)]
pub struct AVCodecContext {
    _private: [u8; 0],
}

#[repr(C)]
pub struct AVCodec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct AVPacket {
    pub buf: *mut c_void,
    pub pts: int64_t,
    pub dts: int64_t,
    pub data: *mut uint8_t,
    pub size: c_int,
    pub stream_index: c_int,
}

pub const AV_NUM_DATA_POINTERS: usize = 8;

/// We only access fields up to `format`.
/// This intentionally avoids relying on FFmpeg's full `AVFrame` layout.
#[repr(C)]
pub struct AVFrame {
    pub data: [*mut uint8_t; AV_NUM_DATA_POINTERS],
    pub linesize: [c_int; AV_NUM_DATA_POINTERS],
    pub extended_data: *mut *mut uint8_t,
    pub width: c_int,
    pub height: c_int,
    pub nb_samples: c_int,
    pub format: c_int,
}

pub type AVDictionary = c_void;

// Function pointer types (subset)
pub type avformat_open_input_fn = unsafe extern "C" fn(
    ps: *mut *mut AVFormatContext,
    url: *const i8,
    fmt: *mut c_void,
    options: *mut *mut AVDictionary,
) -> c_int;

pub type avformat_find_stream_info_fn =
    unsafe extern "C" fn(ic: *mut AVFormatContext, options: *mut *mut c_void) -> c_int;

pub type avformat_close_input_fn = unsafe extern "C" fn(s: *mut *mut AVFormatContext);

pub type av_read_frame_fn =
    unsafe extern "C" fn(s: *mut AVFormatContext, pkt: *mut AVPacket) -> c_int;

pub type av_seek_frame_fn = unsafe extern "C" fn(
    s: *mut AVFormatContext,
    stream_index: c_int,
    timestamp: int64_t,
    flags: c_int,
) -> c_int;

pub type av_find_best_stream_fn = unsafe extern "C" fn(
    ic: *mut AVFormatContext,
    type_: c_int,
    wanted_stream_nb: c_int,
    related_stream: c_int,
    decoder_ret: *mut *const AVCodec,
    flags: c_int,
) -> c_int;

pub type avcodec_alloc_context3_fn =
    unsafe extern "C" fn(codec: *const AVCodec) -> *mut AVCodecContext;

pub type avcodec_open2_fn = unsafe extern "C" fn(
    avctx: *mut AVCodecContext,
    codec: *const AVCodec,
    options: *mut *mut AVDictionary,
) -> c_int;

pub type avcodec_free_context_fn = unsafe extern "C" fn(avctx: *mut *mut AVCodecContext);

pub type avcodec_send_packet_fn =
    unsafe extern "C" fn(avctx: *mut AVCodecContext, avpkt: *const AVPacket) -> c_int;
pub type avcodec_receive_frame_fn =
    unsafe extern "C" fn(avctx: *mut AVCodecContext, frame: *mut AVFrame) -> c_int;
pub type avcodec_flush_buffers_fn = unsafe extern "C" fn(avctx: *mut AVCodecContext);

pub type av_packet_alloc_fn = unsafe extern "C" fn() -> *mut AVPacket;
pub type av_packet_unref_fn = unsafe extern "C" fn(pkt: *mut AVPacket);
pub type av_packet_free_fn = unsafe extern "C" fn(pkt: *mut *mut AVPacket);

pub type av_frame_alloc_fn = unsafe extern "C" fn() -> *mut AVFrame;
pub type av_frame_unref_fn = unsafe extern "C" fn(frame: *mut AVFrame);
pub type av_frame_free_fn = unsafe extern "C" fn(frame: *mut *mut AVFrame);

pub type av_get_bytes_per_sample_fn = unsafe extern "C" fn(sample_fmt: c_int) -> c_int;
pub type av_sample_fmt_is_planar_fn = unsafe extern "C" fn(sample_fmt: c_int) -> c_int;

pub type av_strerror_fn =
    unsafe extern "C" fn(errnum: c_int, errbuf: *mut i8, errbuf_size: usize) -> c_int;

pub type av_log_set_level_fn = unsafe extern "C" fn(level: c_int);
pub type av_get_sample_fmt_name_fn = unsafe extern "C" fn(sample_fmt: c_int) -> *const c_char;

// -------- swresample (audio sample format conversion) --------

// Matches FFmpeg's AVSampleFormat enum value for packed float (FLT).
// FFmpeg keeps these values stable across major versions.
pub const AV_SAMPLE_FMT_FLT: c_int = 3;

#[repr(C)]
pub struct SwrContext {
    _private: [u8; 0],
}

pub type av_get_default_channel_layout_fn = unsafe extern "C" fn(nb_channels: c_int) -> int64_t;

pub type swr_alloc_set_opts_fn = unsafe extern "C" fn(
    s: *mut SwrContext,
    out_ch_layout: int64_t,
    out_sample_fmt: c_int,
    out_sample_rate: c_int,
    in_ch_layout: int64_t,
    in_sample_fmt: c_int,
    in_sample_rate: c_int,
    log_offset: c_int,
    log_ctx: *mut c_void,
) -> *mut SwrContext;

pub type swr_init_fn = unsafe extern "C" fn(s: *mut SwrContext) -> c_int;

pub type swr_free_fn = unsafe extern "C" fn(s: *mut *mut SwrContext);

pub type swr_get_out_samples_fn =
    unsafe extern "C" fn(s: *mut SwrContext, in_samples: c_int) -> c_int;

pub type swr_convert_fn = unsafe extern "C" fn(
    s: *mut SwrContext,
    out: *mut *mut uint8_t,
    out_count: c_int,
    in_: *const *const uint8_t,
    in_count: c_int,
) -> c_int;
