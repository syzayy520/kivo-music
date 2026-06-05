use super::pcm_adapter::{validate_render_bytes, PcmRenderFormat, PcmSampleFormat};
use super::wasapi_context::{WasapiContext, WasapiFormatCache, WasapiRenderWriteReport};
use super::zero_pcm_render::{
    pcm_render_format_from_wasapi_cache, prepare_zero_pcm_render_bytes,
    validate_zero_pcm_write_report, write_zero_pcm_bytes_once, ZeroPcmRenderConfig,
    ZeroPcmRenderError, MAX_ZERO_PCM_RENDER_FRAMES_PER_WRITE,
};

fn format() -> PcmRenderFormat {
    PcmRenderFormat {
        sample_rate_hz: 44_100,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
        sample_format: PcmSampleFormat::Float32Interleaved,
    }
}

fn cache() -> WasapiFormatCache {
    WasapiFormatCache {
        sample_rate_hz: 44_100,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
        avg_bytes_per_sec: 352_800,
        format_tag: 3,
        cb_size: 0,
    }
}

fn report() -> WasapiRenderWriteReport {
    WasapiRenderWriteReport {
        frames_written: 2,
        bytes_written: 16,
        used_silent_flag: false,
        sample_rate_hz: 44_100,
        channels: 2,
    }
}

fn cfg(frames: u32, max_frames_per_write: u32) -> ZeroPcmRenderConfig {
    ZeroPcmRenderConfig {
        frames,
        max_frames_per_write,
    }
}

#[test]
fn zero_pcm_default_config_is_bounded() {
    assert_eq!(
        ZeroPcmRenderConfig::default().max_frames_per_write,
        MAX_ZERO_PCM_RENDER_FRAMES_PER_WRITE
    );
}

#[test]
fn zero_pcm_prepare_rejects_invalid_limits() {
    let cases = [
        (cfg(0, 128), ZeroPcmRenderError::InvalidFrameCount),
        (cfg(1, 0), ZeroPcmRenderError::InvalidMaxFrames),
        (
            cfg(3, 2),
            ZeroPcmRenderError::FrameLimitExceeded { frames: 3, max: 2 },
        ),
        (
            cfg(1, 129),
            ZeroPcmRenderError::FrameLimitExceeded {
                frames: 129,
                max: 128,
            },
        ),
        (
            cfg(129, 129),
            ZeroPcmRenderError::FrameLimitExceeded {
                frames: 129,
                max: 128,
            },
        ),
    ];

    for (config, expected) in cases {
        assert_eq!(
            prepare_zero_pcm_render_bytes(format(), config),
            Err(expected)
        );
    }
}

#[test]
fn zero_pcm_prepare_accepts_zero_bytes_and_pcm_contract() {
    let prepared =
        prepare_zero_pcm_render_bytes(format(), ZeroPcmRenderConfig::new(2)).expect("prepare");

    assert_eq!(prepared.frames, 2);
    assert_eq!(prepared.render_format, format());
    assert_eq!(prepared.bytes.len(), 16);
    assert!(prepared.bytes.iter().all(|byte| *byte == 0));
    assert!(validate_render_bytes(format(), prepared.frames, &prepared.bytes).is_ok());
}

#[test]
fn zero_pcm_prepare_rejects_invalid_format() {
    let mut invalid_block = format();
    invalid_block.block_align = 4;
    let mut unsupported = format();
    unsupported.sample_format = PcmSampleFormat::Signed16Interleaved;

    assert!(matches!(
        prepare_zero_pcm_render_bytes(invalid_block, ZeroPcmRenderConfig::new(1)),
        Err(ZeroPcmRenderError::PcmAdapter(_))
    ));
    assert!(matches!(
        prepare_zero_pcm_render_bytes(unsupported, ZeroPcmRenderConfig::new(1)),
        Err(ZeroPcmRenderError::PcmAdapter(_))
    ));
}

#[test]
fn zero_pcm_cache_to_pcm_format_accepts_ieee_float32() {
    assert_eq!(pcm_render_format_from_wasapi_cache(&cache()), Ok(format()));
}

#[test]
fn zero_pcm_cache_to_pcm_format_rejects_invalid_cache() {
    let mut non_float = cache();
    non_float.format_tag = 1;
    let mut invalid_bits = cache();
    invalid_bits.bits_per_sample = 16;
    let mut invalid_align = cache();
    invalid_align.block_align = 4;

    assert_eq!(
        pcm_render_format_from_wasapi_cache(&non_float),
        Err(ZeroPcmRenderError::UnsupportedFormat)
    );
    assert!(matches!(
        pcm_render_format_from_wasapi_cache(&invalid_bits),
        Err(ZeroPcmRenderError::PcmAdapter(_))
    ));
    assert!(matches!(
        pcm_render_format_from_wasapi_cache(&invalid_align),
        Err(ZeroPcmRenderError::PcmAdapter(_))
    ));
}

#[test]
fn zero_pcm_report_validator_rejects_invalid_write_result() {
    let mut silent = report();
    silent.used_silent_flag = true;
    let mut frame_mismatch = report();
    frame_mismatch.frames_written = 1;
    let mut byte_mismatch = report();
    byte_mismatch.bytes_written = 8;

    assert_eq!(
        validate_zero_pcm_write_report(&silent, 2, 16),
        Err(ZeroPcmRenderError::UnexpectedSilentFlag)
    );
    assert!(matches!(
        validate_zero_pcm_write_report(&frame_mismatch, 2, 16),
        Err(ZeroPcmRenderError::WrittenFrameCountMismatch { .. })
    ));
    assert!(matches!(
        validate_zero_pcm_write_report(&byte_mismatch, 2, 16),
        Err(ZeroPcmRenderError::WrittenByteCountMismatch { .. })
    ));
}

#[test]
#[cfg(not(target_os = "windows"))]
fn zero_pcm_not_open_context_returns_missing_format_cache() {
    assert_eq!(
        write_zero_pcm_bytes_once(&WasapiContext::new(), ZeroPcmRenderConfig::new(1)),
        Err(ZeroPcmRenderError::MissingFormatCache)
    );
}

#[test]
#[cfg(target_os = "windows")]
#[ignore]
fn windows_ignored_zero_pcm_bytes_write_smoke() {
    let mut context = WasapiContext::new();
    if context.open().is_err() {
        return;
    }

    let result = write_zero_pcm_bytes_once(&context, ZeroPcmRenderConfig::new(1));
    context.close();

    if let Ok(report) = result {
        assert_eq!(report.requested_frames, 1);
        assert!(!report.used_silent_flag);
    }
}
