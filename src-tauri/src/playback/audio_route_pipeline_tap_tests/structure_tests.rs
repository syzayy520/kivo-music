use std::path::PathBuf;

const PRODUCTION_FILES: &[&str] = &[
    "src/playback/audio_route_pipeline_tap.rs",
    "src/playback/audio_route_pipeline_tap/config.rs",
    "src/playback/audio_route_pipeline_tap/tap.rs",
    "src/playback/audio_route_pipeline_tap/error.rs",
    "src/playback/audio_route_pipeline_tap/report.rs",
    "src/playback/audio_route_pipeline_tap/mapping.rs",
    "src/playback/audio_route_pipeline_tap/frame_kind.rs",
    "src/playback/native_pipeline_route_tap.rs",
];

const TEST_FILES: &[&str] = &[
    "src/playback/audio_route_pipeline_tap_tests.rs",
    "src/playback/audio_route_pipeline_tap_tests/fixtures.rs",
    "src/playback/audio_route_pipeline_tap_tests/tap_tests.rs",
    "src/playback/audio_route_pipeline_tap_tests/backpressure_tests.rs",
    "src/playback/audio_route_pipeline_tap_tests/failure_isolation_tests.rs",
    "src/playback/audio_route_pipeline_tap_tests/report_tests.rs",
    "src/playback/audio_route_pipeline_tap_tests/structure_tests.rs",
    "src/playback/native_pipeline_route_tap_tests.rs",
];

fn source_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn read_repo_file(relative: &str) -> Result<String, String> {
    std::fs::read_to_string(source_path(relative)).map_err(|error| format!("{error}"))
}

#[test]
fn expected_family_files_exist() {
    for file in PRODUCTION_FILES.iter().chain(TEST_FILES.iter()) {
        assert!(source_path(file).exists(), "{file}");
    }
}

#[test]
fn production_files_avoid_forbidden_boundaries() -> Result<(), String> {
    let forbidden = [
        concat!("Wasapi", "Context"),
        concat!("IAudio", "Client"),
        concat!("Get", "Buffer"),
        concat!("Release", "Buffer"),
        concat!("real", "_transport"),
        concat!("thread", "_stages"),
        concat!("native", "_pipeline_drain"),
        concat!("Output", "Sink"),
        concat!("Wasapi", "Output", "Sink"),
        concat!("sink", "_submission"),
        concat!("sink", "_drain"),
        concat!("submit", "_frame"),
        concat!("Playback", "Manager"),
        concat!("Playback", "Worker"),
        concat!("Playback", "State"),
        concat!("Playback", "Queue"),
        concat!("Audio", "Decoder"),
        concat!("next", "_frame"),
        concat!("std::thread", "::sleep"),
        concat!("Duration", "::"),
        concat!("Instant", "::"),
        concat!("Arc", "<Mutex"),
    ];

    for file in PRODUCTION_FILES {
        let text = read_repo_file(file)?;
        for token in forbidden {
            assert!(!text.contains(token), "{file}: {token}");
        }
    }
    Ok(())
}

#[test]
fn production_files_stay_under_line_budget() -> Result<(), String> {
    for file in PRODUCTION_FILES {
        let text = read_repo_file(file)?;
        assert!(text.lines().count() <= 220, "{file}");
    }
    Ok(())
}

#[test]
fn root_file_is_thin_exports_only() -> Result<(), String> {
    let text = read_repo_file("src/playback/audio_route_pipeline_tap.rs")?;
    assert!(text.lines().count() <= 20);
    assert!(!text.contains("struct "));
    assert!(!text.contains("fn "));
    Ok(())
}

#[test]
fn new_tap_files_do_not_clone_frames_or_samples() -> Result<(), String> {
    let frame_clone = concat!("clone", "(");
    let sample_clone = concat!("samples", ".", "clone");
    for file in PRODUCTION_FILES {
        let text = read_repo_file(file)?;
        assert!(!text.contains(frame_clone), "{file}");
        assert!(!text.contains(sample_clone), "{file}");
    }
    Ok(())
}
