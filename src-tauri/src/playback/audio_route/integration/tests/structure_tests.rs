use std::path::PathBuf;

const PRODUCTION_FILES: &[&str] = &[
    "src/playback/audio_route/integration/mod.rs",
    "src/playback/audio_route/integration/config.rs",
    "src/playback/audio_route/integration/facade.rs",
    "src/playback/audio_route/integration/error.rs",
    "src/playback/audio_route/integration/report.rs",
    "src/playback/audio_route/integration/mapping.rs",
    "src/playback/audio_route/integration/frame_input.rs",
    "src/playback/audio_route/integration/state.rs",
];

const TEST_FILES: &[&str] = &[
    "src/playback/audio_route/integration/tests/mod.rs",
    "src/playback/audio_route/integration/tests/fixtures.rs",
    "src/playback/audio_route/integration/tests/init_tests.rs",
    "src/playback/audio_route/integration/tests/feed_tests.rs",
    "src/playback/audio_route/integration/tests/lifecycle_tests.rs",
    "src/playback/audio_route/integration/tests/report_tests.rs",
    "src/playback/audio_route/integration/tests/error_tests.rs",
    "src/playback/audio_route/integration/tests/structure_tests.rs",
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
        concat!("Native", "Pipeline"),
        concat!("native", "_pipeline"),
        concat!("native", "_output"),
        concat!("native", "_null_output"),
        concat!("Playback", "Manager"),
        concat!("Playback", "Worker"),
        concat!("playback", "_worker"),
        concat!("manager", "_", "que", "ue"),
        concat!("Playback", "State"),
        concat!("Playback", "Que", "ue"),
        concat!("Output", "Sink"),
        concat!("Wasapi", "Output", "Sink"),
        concat!("sink", "_submission"),
        concat!("sink", "_drain"),
        concat!("submit", "_frame"),
        concat!("Audio", "Decoder"),
        concat!("next", "_frame"),
        concat!("std::thread", "::sleep"),
        concat!("Duration", "::"),
        concat!("Instant", "::"),
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
    let text = read_repo_file("src/playback/audio_route/integration/mod.rs")?;
    assert!(text.lines().count() <= 20);
    assert!(!text.contains("struct "));
    assert!(!text.contains("fn "));
    Ok(())
}
