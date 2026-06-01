use std::path::PathBuf;

use hound::{SampleFormat, WavSpec, WavWriter};

use crate::playback::decoder::{AudioDecoder, AudioSampleFormat};
use crate::playback::decoders::factory::create_decoder_for_path;
use crate::playback::decoders::wav_decoder::WavDecoder;
use crate::playback::decoders::wav_format::{map_sample_kind, WavSampleKind};
use crate::playback::errors::PlaybackError;

fn create_test_wav() -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join(format!(
        "kivo-decoder-test-{}.wav",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos()
    ));

    let spec = WavSpec {
        channels: 2,
        sample_rate: 48_000,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create(&path, spec).expect("create wav writer");
    for _ in 0..(48_000 / 10) {
        writer.write_sample(600_i16).expect("write left sample");
        writer.write_sample(-600_i16).expect("write right sample");
    }
    writer.finalize().expect("finalize wav");

    path
}

fn assert_backend_error<T>(result: Result<T, PlaybackError>, expected_message: &str) {
    match result {
        Err(PlaybackError::Backend(message)) => assert_eq!(message, expected_message),
        Err(other) => panic!("expected backend error, got {other}"),
        Ok(_) => panic!("expected backend error, got ok"),
    }
}

#[test]
fn wav_decoder_open_reads_stream_info() {
    let path = create_test_wav();
    let mut decoder = WavDecoder::default();

    let stream = decoder
        .open(path.to_str().expect("wav path should be utf8"))
        .expect("open wav decoder");

    assert_eq!(stream.sample_rate_hz, 48_000);
    assert_eq!(stream.channels, 2);
}

#[test]
fn wav_decoder_next_frame_reads_pcm_samples() {
    let path = create_test_wav();
    let mut decoder = WavDecoder::default();
    decoder
        .open(path.to_str().expect("wav path should be utf8"))
        .expect("open wav decoder");

    let frame = decoder
        .next_frame()
        .expect("read frame")
        .expect("frame should exist");

    assert!(!frame.samples.is_empty());
    assert!(frame.samples[0] > 0.0);
    assert!(frame.samples[1] < 0.0);
}

#[test]
fn wav_decoder_next_frame_before_open_returns_backend_error() {
    let mut decoder = WavDecoder::default();

    let result = decoder.next_frame();

    assert_backend_error(result, "decoder is not open");
}

#[test]
fn wav_decoder_next_frame_after_close_returns_backend_error() {
    let mut decoder = WavDecoder::default();

    decoder.close().expect("close unopened decoder");
    let result = decoder.next_frame();

    assert_backend_error(result, "decoder is not open");
}

#[test]
fn decoder_factory_accepts_uppercase_wav_extension() {
    let result = create_decoder_for_path("C:/Music/song.WAV");

    assert!(result.is_ok());
}

#[test]
fn decoder_factory_rejects_missing_extension() {
    let result = create_decoder_for_path("C:/Music/song");

    match result {
        Err(PlaybackError::UnsupportedFormat(format)) => {
            assert_eq!(format, "missing file extension")
        }
        Err(other) => panic!("expected unsupported format, got {other}"),
        Ok(_) => panic!("expected unsupported format, got decoder"),
    }
}

#[test]
fn decoder_factory_rejects_unsupported_extension() {
    let result = create_decoder_for_path("C:/Music/song.flac");

    match result {
        Err(PlaybackError::UnsupportedFormat(format)) => assert_eq!(format, "flac"),
        Err(other) => panic!("expected unsupported format, got {other}"),
        Ok(_) => panic!("expected unsupported format, got decoder"),
    }
}

#[test]
fn wav_sample_kind_maps_supported_formats() {
    let (int16_kind, int16_format) = map_sample_kind(SampleFormat::Int, 16).expect("map int16");
    let (int24_kind, int24_format) = map_sample_kind(SampleFormat::Int, 24).expect("map int24");
    let (int32_kind, int32_format) = map_sample_kind(SampleFormat::Int, 32).expect("map int32");
    let (float32_kind, float32_format) =
        map_sample_kind(SampleFormat::Float, 32).expect("map float32");

    assert!(matches!(int16_kind, WavSampleKind::Int16));
    assert!(matches!(int16_format, AudioSampleFormat::Signed16));
    assert!(matches!(int24_kind, WavSampleKind::Int24));
    assert!(matches!(int24_format, AudioSampleFormat::Signed24));
    assert!(matches!(int32_kind, WavSampleKind::Int32));
    assert!(matches!(int32_format, AudioSampleFormat::Signed32));
    assert!(matches!(float32_kind, WavSampleKind::Float32));
    assert!(matches!(float32_format, AudioSampleFormat::Float32));
}

#[test]
fn wav_sample_kind_rejects_unsupported_bit_depth() {
    let result = map_sample_kind(SampleFormat::Int, 20);

    match result {
        Err(PlaybackError::UnsupportedFormat(format)) => assert_eq!(format, "wav Int 20-bit"),
        Err(other) => panic!("expected unsupported format, got {other}"),
        Ok(_) => panic!("expected unsupported format, got sample kind"),
    }
}
