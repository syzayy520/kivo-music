use std::path::PathBuf;

use hound::{SampleFormat, WavSpec, WavWriter};

use crate::playback::decoder::AudioDecoder;
use crate::playback::decoders::factory::create_decoder_for_path;
use crate::playback::decoders::wav_decoder::WavDecoder;
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
fn wav_decoder_open_reports_duration_ms() {
    let path = create_test_wav();
    let mut decoder = WavDecoder::default();

    decoder
        .open(path.to_str().expect("wav path should be utf8"))
        .expect("open wav decoder");

    assert_eq!(decoder.duration_ms(), Some(100));
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
fn decoder_factory_rejects_unsupported_extension() {
    let result = create_decoder_for_path("C:/Music/song.flac");

    match result {
        Err(PlaybackError::UnsupportedFormat(format)) => assert_eq!(format, "flac"),
        Err(other) => panic!("expected unsupported format, got {other}"),
        Ok(_) => panic!("expected unsupported format, got decoder"),
    }
}
