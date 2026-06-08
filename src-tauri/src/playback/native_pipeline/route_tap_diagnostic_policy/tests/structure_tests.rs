use std::path::PathBuf;

const PRODUCTION_FILES: &[&str] = &[
    "src/playback/native_pipeline/route_tap_diagnostic_policy/mod.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/close_detach.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/config.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/error.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/open_new_track.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/query.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/seek.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/state.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/stream_compat.rs",
    "src/playback/native_pipeline/route_tap.rs",
];

const TEST_FILES: &[&str] = &[
    "src/playback/native_pipeline/route_tap_diagnostic_policy/tests/mod.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/tests/close_tests.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/tests/config_tests.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/tests/fixtures.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/tests/open_tests.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/tests/query_tests.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/tests/reset_contract_tests.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/tests/seek_tests.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/tests/stream_compat_tests.rs",
    "src/playback/native_pipeline/route_tap_diagnostic_policy/tests/structure_tests.rs",
];

fn source_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn read_repo_file(relative: &str) -> Result<String, String> {
    std::fs::read_to_string(source_path(relative)).map_err(|error| format!("{error}"))
}

#[test]
fn diagnostic_policy_family_exists_and_stays_small() -> Result<(), String> {
    for file in PRODUCTION_FILES {
        let source = read_repo_file(file)?;
        assert!(source.lines().count() <= 220, "{file}");
    }
    for file in TEST_FILES {
        let source = read_repo_file(file)?;
        assert!(source.lines().count() <= 150, "{file}");
    }
    Ok(())
}

#[test]
fn policy_root_is_thin_tree_registration_only() -> Result<(), String> {
    let source = read_repo_file("src/playback/native_pipeline/route_tap_diagnostic_policy/mod.rs")?;

    assert!(!source.contains("struct "));
    assert!(!source.contains("fn "));
    assert!(source.lines().count() <= 30);
    Ok(())
}

#[test]
fn policy_keeps_one_bounded_detached_report_without_history() -> Result<(), String> {
    let source =
        read_repo_file("src/playback/native_pipeline/route_tap_diagnostic_policy/state.rs")?;

    assert!(source.contains("last_detached_report: Option<AudioRoutePipelineTapReport>"));
    assert!(!source.contains("Vec<"));
    assert!(!source.contains("history"));
    Ok(())
}

#[test]
fn policy_has_no_forbidden_runtime_coupling() -> Result<(), String> {
    let forbidden = [
        concat!("Playback", "State"),
        concat!("Worker", "State"),
        concat!("Output", "Runtime", "Status"),
        concat!("output", "_status"),
        concat!("real", "_transport"),
        concat!("Output", "Sink"),
        concat!("Wasapi", "Output"),
        concat!("Playback", "Manager"),
        concat!("Playback", "Worker"),
        concat!("Playback", "Queue"),
        concat!("Arc", "<Mutex"),
    ];

    for file in PRODUCTION_FILES {
        let source = read_repo_file(file)?;
        for token in forbidden {
            assert!(!source.contains(token), "{file}: {token}");
        }
    }
    Ok(())
}

#[test]
fn production_lifecycle_paths_do_not_call_diagnostic_policy() -> Result<(), String> {
    let production_paths = [
        "src/playback/native_pipeline/decoder.rs",
        "src/playback/native_pipeline/runtime.rs",
        "src/playback/native_pipeline/worker.rs",
        "src/playback/native_pipeline/drain/mod.rs",
    ];
    let policy_type = "NativePipelineRouteTapDiagnosticPolicy";

    for file in production_paths {
        assert!(!read_repo_file(file)?.contains(policy_type), "{file}");
    }
    Ok(())
}
