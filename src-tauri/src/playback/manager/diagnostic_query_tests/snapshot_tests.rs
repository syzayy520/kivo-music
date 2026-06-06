use super::super::diagnostic_query::TapDiagnosticSampleFormatSnapshot;
use super::super::PlaybackManager;
use crate::playback::native_pipeline_route_tap_diagnostic_policy::NativeTapDiagnosticConfig;
use crate::playback::types::{PlaybackTrack, TrackId};

#[test]
fn manager_diagnostic_query_returns_current_compact_snapshot_after_diagnostic_open(
) -> Result<(), String> {
    let mut manager = diagnostic_manager(4)?;
    let (track, path) = wav_track("manager-diagnostic-current", 48_000)?;

    let _ = manager.load(track);
    let snapshot = manager.tap_diagnostic_report_snapshot();
    remove_wav(path);

    let current = snapshot
        .current
        .ok_or_else(|| "current diagnostic report missing".to_string())?;
    let stream = snapshot
        .current_stream
        .ok_or_else(|| "current diagnostic stream missing".to_string())?;

    assert!(snapshot.enabled);
    assert!(snapshot.last_detached.is_none());
    assert!(current.initialized);
    assert!(!current.closed);
    assert_eq!(current.capacity_frames, 4);
    assert_eq!(current.pending_frames, 2);
    assert_eq!(current.input_count, 1);
    assert_eq!(current.total_accepted_frames, 2);
    assert!(current.last_error.is_none());
    assert_eq!(stream.sample_rate_hz, 48_000);
    assert_eq!(stream.channels, 2);
    assert_eq!(
        stream.sample_format,
        TapDiagnosticSampleFormatSnapshot::Float32
    );
    Ok(())
}

#[test]
fn manager_diagnostic_query_returns_last_detached_snapshot_after_stop() -> Result<(), String> {
    let mut manager = diagnostic_manager(4)?;
    let (track, path) = wav_track("manager-diagnostic-last", 44_100)?;

    let _ = manager.load(track);
    let _ = manager.stop();
    let snapshot = manager.tap_diagnostic_report_snapshot();
    remove_wav(path);

    let last = snapshot
        .last_detached
        .ok_or_else(|| "last detached diagnostic report missing".to_string())?;

    assert!(snapshot.enabled);
    assert!(snapshot.current.is_none());
    assert!(snapshot.current_stream.is_none());
    assert!(last.closed);
    assert_eq!(last.capacity_frames, 4);
    assert_eq!(last.input_count, 1);
    Ok(())
}

#[test]
fn manager_diagnostic_repeated_queries_are_fresh_owned_values_without_mutation(
) -> Result<(), String> {
    let mut manager = diagnostic_manager(2)?;
    let (track, path) = wav_track("manager-diagnostic-fresh", 48_000)?;

    let _ = manager.load(track);
    let first = manager.tap_diagnostic_report_snapshot();
    let second = manager.tap_diagnostic_report_snapshot();
    remove_wav(path);

    assert_eq!(first, second);
    assert!(first.enabled);
    assert!(first.current.is_some());
    Ok(())
}

fn diagnostic_manager(capacity_frames: u32) -> Result<PlaybackManager, String> {
    let config =
        NativeTapDiagnosticConfig::new(capacity_frames).map_err(|error| format!("{error:?}"))?;
    let mut manager = PlaybackManager::new();
    manager.primary_engine =
        crate::playback::backends::native::KivoNativeEngine::new_with_tap_diagnostic_policy(config);
    Ok(manager)
}

fn wav_track(id: &str, sample_rate: u32) -> Result<(PlaybackTrack, String), String> {
    let path = std::env::temp_dir().join(format!(
        "kivo-manager-diagnostic-{}-{}-{id}.wav",
        std::process::id(),
        unique_suffix()
    ));
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(&path, spec).map_err(|error| format!("{error}"))?;
    for sample in [0.0_f32, 0.25, -0.25, 0.5] {
        writer
            .write_sample(sample)
            .map_err(|error| format!("{error}"))?;
    }
    writer.finalize().map_err(|error| format!("{error}"))?;
    let source_path = path.to_string_lossy().into_owned();

    Ok((
        PlaybackTrack {
            id: TrackId(id.to_string()),
            title: id.to_string(),
            artist: "Diagnostic".to_string(),
            source_path: source_path.clone(),
        },
        source_path,
    ))
}

fn remove_wav(path: String) {
    let _ = std::fs::remove_file(path);
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default()
}
