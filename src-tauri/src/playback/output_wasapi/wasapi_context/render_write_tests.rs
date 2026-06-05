//! Pure tests for render write primitives and format cache.
//!
//! These tests verify cross-platform behavior without requiring real audio hardware.

use super::format_cache::WasapiFormatCache;
use super::render_error::{WasapiRenderWriteError, WasapiRenderWriteReport};
use super::WasapiContext;

// --- Test A: WasapiFormatCache field creation ---

#[test]
fn format_cache_from_fields() {
    let cache = WasapiFormatCache {
        sample_rate_hz: 48000,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
        avg_bytes_per_sec: 384000,
        format_tag: 3,
        cb_size: 0,
    };
    assert_eq!(cache.sample_rate_hz, 48000);
    assert_eq!(cache.channels, 2);
    assert_eq!(cache.bits_per_sample, 32);
    assert_eq!(cache.block_align, 8);
    assert_eq!(cache.avg_bytes_per_sec, 384000);
    assert_eq!(cache.format_tag, 3);
    assert_eq!(cache.cb_size, 0);
}

// --- Test B: is_float32 ---

#[test]
fn is_float32_returns_true_for_format_tag_3() {
    let cache = WasapiFormatCache {
        sample_rate_hz: 44100,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
        avg_bytes_per_sec: 352800,
        format_tag: 3,
        cb_size: 0,
    };
    assert!(cache.is_float32());
}

#[test]
fn is_float32_returns_false_for_format_tag_1() {
    let cache = WasapiFormatCache {
        sample_rate_hz: 44100,
        channels: 2,
        bits_per_sample: 16,
        block_align: 4,
        avg_bytes_per_sec: 176400,
        format_tag: 1,
        cb_size: 0,
    };
    assert!(!cache.is_float32());
}

// --- Test C/D: write methods return NotOpen when not open ---

#[test]
fn write_silence_returns_not_open_when_not_open() {
    let ctx = WasapiContext::new();
    let result = ctx.write_render_buffer_silence(1);
    assert_eq!(result.unwrap_err(), WasapiRenderWriteError::NotOpen);
}

#[test]
fn write_bytes_returns_not_open_when_not_open() {
    let ctx = WasapiContext::new();
    let data = vec![0u8; 8];
    let result = ctx.write_render_buffer_bytes(1, &data);
    assert_eq!(result.unwrap_err(), WasapiRenderWriteError::NotOpen);
}

// --- Error Display tests ---

#[test]
fn render_write_error_display_variants() {
    let cases = [
        (WasapiRenderWriteError::NotOpen, "context is not open"),
        (
            WasapiRenderWriteError::MissingRenderClient,
            "render client is missing",
        ),
        (
            WasapiRenderWriteError::MissingAudioClient,
            "audio client is missing",
        ),
        (
            WasapiRenderWriteError::MissingFormatCache,
            "format cache is missing",
        ),
        (
            WasapiRenderWriteError::UnsupportedFormat,
            "format is not IEEE Float32",
        ),
        (
            WasapiRenderWriteError::InvalidFrameCount,
            "frame count must be > 0",
        ),
    ];
    for (err, expected) in &cases {
        assert_eq!(&format!("{err}"), expected);
    }
    let mismatch = WasapiRenderWriteError::ByteLengthMismatch {
        expected: 8,
        actual: 4,
    };
    assert_eq!(
        format!("{mismatch}"),
        "byte length mismatch: expected 8, got 4"
    );
    let get_err = WasapiRenderWriteError::GetBufferFailed("E_INVALIDARG".into());
    assert_eq!(format!("{get_err}"), "GetBuffer failed: E_INVALIDARG");
    let rel_err = WasapiRenderWriteError::ReleaseBufferFailed("E_FAIL".into());
    assert_eq!(format!("{rel_err}"), "ReleaseBuffer failed: E_FAIL");
}

// --- Report basics ---

#[test]
fn render_write_report_silence_basics() {
    let report = WasapiRenderWriteReport {
        frames_written: 100,
        bytes_written: 0,
        used_silent_flag: true,
        sample_rate_hz: 44100,
        channels: 2,
    };
    assert_eq!(report.frames_written, 100);
    assert_eq!(report.bytes_written, 0);
    assert!(report.used_silent_flag);
}

#[test]
fn render_write_report_bytes_basics() {
    let report = WasapiRenderWriteReport {
        frames_written: 480,
        bytes_written: 1920,
        used_silent_flag: false,
        sample_rate_hz: 48000,
        channels: 2,
    };
    assert_eq!(report.bytes_written, 1920);
    assert!(!report.used_silent_flag);
}

// --- frames_to_bytes ---

#[test]
fn frames_to_bytes_calculations() {
    let stereo = WasapiFormatCache {
        sample_rate_hz: 48000,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
        avg_bytes_per_sec: 384000,
        format_tag: 3,
        cb_size: 0,
    };
    assert_eq!(stereo.frames_to_bytes(1), 8);
    assert_eq!(stereo.frames_to_bytes(100), 800);
    assert_eq!(stereo.frames_to_bytes(0), 0);

    let mono = WasapiFormatCache {
        channels: 1,
        block_align: 4,
        ..stereo
    };
    assert_eq!(mono.frames_to_bytes(1), 4);
    assert_eq!(mono.frames_to_bytes(480), 1920);
}

// --- format_cache accessor ---

#[test]
fn format_cache_returns_none_when_not_open() {
    let ctx = WasapiContext::new();
    assert!(ctx.format_cache().is_none());
}

// --- Clone/PartialEq traits ---

#[test]
fn format_cache_clone_and_eq() {
    let cache = WasapiFormatCache {
        sample_rate_hz: 44100,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
        avg_bytes_per_sec: 352800,
        format_tag: 3,
        cb_size: 0,
    };
    let cloned = cache.clone();
    assert_eq!(cache, cloned);
    let debug = format!("{cache:?}");
    assert!(debug.contains("WasapiFormatCache"));
}

#[test]
fn render_write_report_clone_and_eq() {
    let report = WasapiRenderWriteReport {
        frames_written: 100,
        bytes_written: 0,
        used_silent_flag: true,
        sample_rate_hz: 44100,
        channels: 2,
    };
    assert_eq!(report, report.clone());
}

#[test]
fn render_write_error_clone_and_eq() {
    let err = WasapiRenderWriteError::NotOpen;
    assert_eq!(err, err.clone());
}
