use std::path::PathBuf;

const PROD_FILES: &[&str] = &[
    "src/playback/audio_route.rs",
    "src/playback/audio_route/config.rs",
    "src/playback/audio_route/owner.rs",
    "src/playback/audio_route/error.rs",
    "src/playback/audio_route/report.rs",
    "src/playback/audio_route/feeder.rs",
    "src/playback/audio_route/format.rs",
];

const TEST_FILES: &[&str] = &[
    "src/playback/audio_route_tests.rs",
    "src/playback/audio_route_tests/fixtures.rs",
    "src/playback/audio_route_tests/owner_tests.rs",
    "src/playback/audio_route_tests/feed_tests.rs",
    "src/playback/audio_route_tests/format_tests.rs",
    "src/playback/audio_route_tests/backpressure_tests.rs",
    "src/playback/audio_route_tests/lifecycle_tests.rs",
    "src/playback/audio_route_tests/report_tests.rs",
    "src/playback/audio_route_tests/structure_tests.rs",
];

#[test]
fn production_route_files_do_not_import_forbidden_boundaries() -> Result<(), String> {
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
        concat!("native", "_null", "_output"),
        concat!("Output", "Sink"),
        concat!("Wasapi", "Output", "Sink"),
        concat!("sink", "_submission"),
        concat!("sink", "_drain"),
        concat!("submit", "_frame"),
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
fn route_family_files_stay_under_line_limit() -> Result<(), String> {
    for file in PROD_FILES.iter().chain(TEST_FILES.iter()) {
        let line_count = read_repo_file(file)?.lines().count();
        assert!(line_count <= 220, "{file} has {line_count} lines");
    }
    Ok(())
}

#[test]
fn route_root_is_thin_exports_only() -> Result<(), String> {
    let text = read_repo_file("src/playback/audio_route.rs")?;
    assert!(!text.contains("fn "));
    assert!(text.lines().count() <= 20);
    Ok(())
}

fn read_repo_file(path: &str) -> Result<String, String> {
    std::fs::read_to_string(repo_file(path)).map_err(|error| format!("{error:?}"))
}

fn repo_file(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
}
