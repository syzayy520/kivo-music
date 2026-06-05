#![allow(dead_code)]

use super::pcm_adapter::{
    validate_render_bytes, validate_render_format, PcmAdapterError, PcmRenderFormat,
    PcmSampleFormat,
};
use super::wasapi_context::{
    WasapiContext, WasapiFormatCache, WasapiRenderWriteError, WasapiRenderWriteReport,
};

pub(crate) const MAX_ZERO_PCM_RENDER_FRAMES_PER_WRITE: u32 = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ZeroPcmRenderConfig {
    pub frames: u32,
    pub max_frames_per_write: u32,
}

impl ZeroPcmRenderConfig {
    pub(crate) fn new(frames: u32) -> Self {
        Self {
            frames,
            max_frames_per_write: MAX_ZERO_PCM_RENDER_FRAMES_PER_WRITE,
        }
    }
}

impl Default for ZeroPcmRenderConfig {
    fn default() -> Self {
        Self::new(1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ZeroPcmRenderPrepared {
    pub frames: u32,
    pub render_format: PcmRenderFormat,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ZeroPcmRenderReport {
    pub requested_frames: u32,
    pub frames_written: u32,
    pub bytes_written: u32,
    pub used_silent_flag: bool,
    pub sample_rate_hz: u32,
    pub channels: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ZeroPcmRenderError {
    InvalidFrameCount,
    InvalidMaxFrames,
    FrameLimitExceeded { frames: u32, max: u32 },
    MissingFormatCache,
    UnsupportedFormat,
    PcmAdapter(PcmAdapterError),
    WasapiWrite(WasapiRenderWriteError),
    WrittenFrameCountMismatch { expected: u32, actual: u32 },
    WrittenByteCountMismatch { expected: u32, actual: u32 },
    UnexpectedSilentFlag,
    ByteLengthOverflow,
}

pub(crate) fn pcm_render_format_from_wasapi_cache(
    cache: &WasapiFormatCache,
) -> Result<PcmRenderFormat, ZeroPcmRenderError> {
    if !cache.is_float32() {
        return Err(ZeroPcmRenderError::UnsupportedFormat);
    }

    let format = PcmRenderFormat {
        sample_rate_hz: cache.sample_rate_hz,
        channels: cache.channels,
        bits_per_sample: cache.bits_per_sample,
        block_align: cache.block_align,
        sample_format: PcmSampleFormat::Float32Interleaved,
    };

    validate_render_format(format).map_err(ZeroPcmRenderError::PcmAdapter)
}

pub(crate) fn prepare_zero_pcm_render_bytes(
    format: PcmRenderFormat,
    config: ZeroPcmRenderConfig,
) -> Result<ZeroPcmRenderPrepared, ZeroPcmRenderError> {
    validate_config(config)?;
    let format = validate_render_format(format).map_err(ZeroPcmRenderError::PcmAdapter)?;
    let byte_len = checked_byte_len(config.frames, format.block_align)?;
    let bytes = vec![0_u8; byte_len];

    validate_render_bytes(format, config.frames, &bytes).map_err(ZeroPcmRenderError::PcmAdapter)?;

    Ok(ZeroPcmRenderPrepared {
        frames: config.frames,
        render_format: format,
        bytes,
    })
}

pub(crate) fn write_zero_pcm_bytes_once(
    context: &WasapiContext,
    config: ZeroPcmRenderConfig,
) -> Result<ZeroPcmRenderReport, ZeroPcmRenderError> {
    let cache = context
        .format_cache()
        .ok_or(ZeroPcmRenderError::MissingFormatCache)?;
    let format = pcm_render_format_from_wasapi_cache(cache)?;
    let prepared = prepare_zero_pcm_render_bytes(format, config)?;
    let write_report = context
        .write_render_buffer_bytes(prepared.frames, &prepared.bytes)
        .map_err(ZeroPcmRenderError::WasapiWrite)?;
    let expected_bytes = checked_u32_len(prepared.bytes.len())?;

    validate_zero_pcm_write_report(&write_report, prepared.frames, expected_bytes)
}

pub(crate) fn validate_zero_pcm_write_report(
    report: &WasapiRenderWriteReport,
    requested_frames: u32,
    expected_bytes: u32,
) -> Result<ZeroPcmRenderReport, ZeroPcmRenderError> {
    if report.frames_written != requested_frames {
        return Err(ZeroPcmRenderError::WrittenFrameCountMismatch {
            expected: requested_frames,
            actual: report.frames_written,
        });
    }

    if report.bytes_written != expected_bytes {
        return Err(ZeroPcmRenderError::WrittenByteCountMismatch {
            expected: expected_bytes,
            actual: report.bytes_written,
        });
    }

    if report.used_silent_flag {
        return Err(ZeroPcmRenderError::UnexpectedSilentFlag);
    }

    Ok(ZeroPcmRenderReport {
        requested_frames,
        frames_written: report.frames_written,
        bytes_written: report.bytes_written,
        used_silent_flag: report.used_silent_flag,
        sample_rate_hz: report.sample_rate_hz,
        channels: report.channels,
    })
}

fn validate_config(config: ZeroPcmRenderConfig) -> Result<(), ZeroPcmRenderError> {
    if config.frames == 0 {
        return Err(ZeroPcmRenderError::InvalidFrameCount);
    }

    if config.max_frames_per_write == 0 {
        return Err(ZeroPcmRenderError::InvalidMaxFrames);
    }

    if config.max_frames_per_write > MAX_ZERO_PCM_RENDER_FRAMES_PER_WRITE {
        return Err(ZeroPcmRenderError::FrameLimitExceeded {
            frames: config.max_frames_per_write,
            max: MAX_ZERO_PCM_RENDER_FRAMES_PER_WRITE,
        });
    }

    if config.frames > config.max_frames_per_write {
        return Err(ZeroPcmRenderError::FrameLimitExceeded {
            frames: config.frames,
            max: config.max_frames_per_write,
        });
    }

    if config.frames > MAX_ZERO_PCM_RENDER_FRAMES_PER_WRITE {
        return Err(ZeroPcmRenderError::FrameLimitExceeded {
            frames: config.frames,
            max: MAX_ZERO_PCM_RENDER_FRAMES_PER_WRITE,
        });
    }

    Ok(())
}

fn checked_byte_len(frames: u32, block_align: u16) -> Result<usize, ZeroPcmRenderError> {
    let frames = usize::try_from(frames).map_err(|_| ZeroPcmRenderError::ByteLengthOverflow)?;

    frames
        .checked_mul(usize::from(block_align))
        .ok_or(ZeroPcmRenderError::ByteLengthOverflow)
}

fn checked_u32_len(len: usize) -> Result<u32, ZeroPcmRenderError> {
    u32::try_from(len).map_err(|_| ZeroPcmRenderError::ByteLengthOverflow)
}
