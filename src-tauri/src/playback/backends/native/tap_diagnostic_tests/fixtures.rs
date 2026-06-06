use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::native_pipeline_route_tap_diagnostic_policy::NativeTapDiagnosticConfig;
use crate::playback::state::PlaybackState;
use crate::playback::types::{PlaybackTrack, TrackId};

use super::super::KivoNativeEngine;

pub(super) fn enable_diagnostic(engine: &mut KivoNativeEngine, capacity_frames: u32) {
    let config =
        NativeTapDiagnosticConfig::new(capacity_frames).expect("valid diagnostic capacity");
    *engine = KivoNativeEngine::new_with_tap_diagnostic_policy(config);
}

pub(super) fn wav_track(id: &str, sample_rate: u32) -> (PlaybackTrack, String) {
    let (track, path) = wav_track_path(id);
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(&path, spec).expect("create diagnostic wav");
    for sample in [0.0_f32, 0.25, -0.25, 0.5] {
        writer
            .write_sample(sample)
            .expect("write diagnostic sample");
    }
    writer.finalize().expect("finalize diagnostic wav");
    (track, path)
}

pub(super) fn signed16_wav_track(id: &str) -> (PlaybackTrack, String) {
    let (track, path) = wav_track_path(id);
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44_100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&path, spec).expect("create signed16 wav");
    for sample in [0_i16, 1000, -1000, 2000] {
        writer.write_sample(sample).expect("write signed16 sample");
    }
    writer.finalize().expect("finalize signed16 wav");
    (track, path)
}

fn wav_track_path(id: &str) -> (PlaybackTrack, String) {
    let path = std::env::temp_dir().join(format!(
        "kivo-native-tap-diagnostic-{}-{}-{id}.wav",
        std::process::id(),
        unique_suffix()
    ));
    let source_path = path.to_string_lossy().into_owned();
    (
        PlaybackTrack {
            id: TrackId(id.to_string()),
            title: id.to_string(),
            artist: "Diagnostic".to_string(),
            source_path: source_path.clone(),
        },
        source_path,
    )
}

pub(super) fn unsupported_track() -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId("tap-diagnostic-unsupported".to_string()),
        title: "Unsupported".to_string(),
        artist: "Diagnostic".to_string(),
        source_path: "C:/Music/tap-diagnostic.flac".to_string(),
    }
}

pub(super) fn assert_unsupported(result: PlaybackResult<PlaybackState>, expected_message: &str) {
    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(message, expected_message);
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }
}

pub(super) fn remove_wav(path: String) {
    std::fs::remove_file(path).expect("remove diagnostic wav");
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos()
}
