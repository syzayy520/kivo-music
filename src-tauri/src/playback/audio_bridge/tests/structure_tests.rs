use std::path::PathBuf;

const PROD_FILES: &[&str] = &[
    "src/playback/audio_bridge/mod.rs",
    "src/playback/audio_bridge/types.rs",
    "src/playback/audio_bridge/source.rs",
    "src/playback/audio_bridge/source_to_ring_buffer.rs",
    "src/playback/audio_bridge/error.rs",
    "src/playback/audio_bridge/report.rs",
];

const TEST_FILES: &[&str] = &[
    "src/playback/audio_bridge/tests/mod.rs",
    "src/playback/audio_bridge/tests/fixtures.rs",
    "src/playback/audio_bridge/tests/format_tests.rs",
    "src/playback/audio_bridge/tests/write_tests.rs",
    "src/playback/audio_bridge/tests/partial_write_tests.rs",
    "src/playback/audio_bridge/tests/backpressure_tests.rs",
    "src/playback/audio_bridge/tests/error_tests.rs",
    "src/playback/audio_bridge/tests/report_tests.rs",
    "src/playback/audio_bridge/tests/structure_tests.rs",
];

#[test]
fn production_bridge_files_do_not_import_forbidden_boundaries() -> Result<(), String> {
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
        concat!("Output", "Sink"),
        concat!("sink", "_submission"),
        concat!("sink", "_drain"),
        concat!("std", "::", "thread", "::", "sleep"),
        concat!("Duration", "::"),
        concat!("Instant", "::"),
    ];

    for file in PROD_FILES {
        let text = read_repo_file(file)?;
        for token in forbidden {
            assert!(!text.contains(token), "{file} contains {token}");
        }
    }
    Ok(())
}

#[test]
fn audio_bridge_family_files_stay_under_line_limit() -> Result<(), String> {
    for file in PROD_FILES.iter().chain(TEST_FILES.iter()) {
        let line_count = read_repo_file(file)?.lines().count();
        assert!(line_count <= 220, "{file} has {line_count} lines");
    }
    Ok(())
}

fn read_repo_file(path: &str) -> Result<String, String> {
    std::fs::read_to_string(repo_file(path)).map_err(|error| format!("{error:?}"))
}

fn repo_file(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
}
