use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::engine::PlaybackEngine;

use super::super::KivoNativeEngine;
use super::fixtures::{enable_diagnostic, remove_wav, wav_track};

#[test]
fn internal_queries_delegate_to_policy_owned_state() {
    let mut engine = KivoNativeEngine::new();
    enable_diagnostic(&mut engine, 4);
    let (track, path) = wav_track("query", 44_100);
    let _ = engine.load(track);
    let expected = AudioStreamInfo {
        sample_rate_hz: 44_100,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    };

    assert!(engine.tap_diagnostic_current_report().is_some());
    assert_eq!(
        engine
            .tap_diagnostic_current_stream()
            .map(|stream| (stream.sample_rate_hz, stream.channels)),
        Some((44_100, 2))
    );
    assert!(engine.tap_diagnostic_stream_is_compatible(&expected));
    remove_wav(path);
}

#[test]
fn last_report_query_reads_policy_after_close() {
    let mut engine = KivoNativeEngine::new();
    enable_diagnostic(&mut engine, 4);
    let (track, path) = wav_track("query-close", 44_100);
    let _ = engine.load(track);
    let _ = engine.stop();

    assert!(engine.tap_diagnostic_current_report().is_none());
    assert!(engine
        .tap_diagnostic_last_detached_report()
        .is_some_and(|report| report.closed));
    remove_wav(path);
}
