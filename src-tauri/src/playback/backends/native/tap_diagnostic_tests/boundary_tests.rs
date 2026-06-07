use std::path::PathBuf;

const PRODUCTION_FILES: &[&str] = &[
    "src/playback/backends/native/tap_diagnostic/mod.rs",
    "src/playback/backends/native/tap_diagnostic/state.rs",
    "src/playback/backends/native/tap_diagnostic/config.rs",
    "src/playback/backends/native/tap_diagnostic/open.rs",
    "src/playback/backends/native/tap_diagnostic/seek.rs",
    "src/playback/backends/native/tap_diagnostic/close.rs",
    "src/playback/backends/native/tap_diagnostic/query.rs",
    "src/playback/backends/native.rs",
    "src/playback/backends/native/load.rs",
    "src/playback/backends/native/control.rs",
    "src/playback/backends/native/engine.rs",
    "src/playback/backends/native/stop.rs",
];

const TEST_FILES: &[&str] = &[
    "src/playback/backends/native/tap_diagnostic_tests/mod.rs",
    "src/playback/backends/native/tap_diagnostic_tests/boundary_tests.rs",
    "src/playback/backends/native/tap_diagnostic_tests/close_tests.rs",
    "src/playback/backends/native/tap_diagnostic_tests/config_tests.rs",
    "src/playback/backends/native/tap_diagnostic_tests/disabled_tests.rs",
    "src/playback/backends/native/tap_diagnostic_tests/fixtures.rs",
    "src/playback/backends/native/tap_diagnostic_tests/open_tests.rs",
    "src/playback/backends/native/tap_diagnostic_tests/query_tests.rs",
    "src/playback/backends/native/tap_diagnostic_tests/seek_tests.rs",
];

fn source_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn read(relative: &str) -> Result<String, String> {
    std::fs::read_to_string(source_path(relative)).map_err(|error| error.to_string())
}

#[test]
fn module_genealogy_exists_without_flat_mixed_helper() {
    for file in PRODUCTION_FILES.iter().chain(TEST_FILES.iter()) {
        assert!(source_path(file).exists(), "{file}");
    }
    assert!(!source_path("src/playback/backends/native/tap_diagnostic.rs").exists());
}

#[test]
fn production_and_test_files_stay_within_line_budgets() -> Result<(), String> {
    for file in PRODUCTION_FILES {
        assert!(read(file)?.lines().count() <= 220, "{file}");
    }
    for file in TEST_FILES {
        assert!(read(file)?.lines().count() <= 250, "{file}");
    }
    Ok(())
}

#[test]
fn sidecar_state_has_one_optional_policy_and_no_report_cache() -> Result<(), String> {
    let source = read("src/playback/backends/native/tap_diagnostic/state.rs")?;

    assert!(source.contains("Option<NativePipelineRouteTapDiagnosticPolicy>"));
    assert!(!source.contains("AudioRoutePipelineTapReport"));
    assert!(!source.contains("AudioRoutePipelineTap>"));
    assert!(!source.contains("Vec<"));
    Ok(())
}

#[test]
fn diagnostic_family_has_no_forbidden_layer_coupling() -> Result<(), String> {
    let forbidden = [
        concat!("Playback", "State"),
        concat!("Worker", "State"),
        concat!("Output", "Runtime", "Status"),
        concat!("output", "_status"),
        concat!("real", "_transport"),
        concat!("Output", "Sink"),
        concat!("Playback", "Manager"),
        concat!("Playback", "Worker"),
        concat!("Playback", "Queue"),
        "state.error",
    ];

    for file in &PRODUCTION_FILES[..7] {
        let source = read(file)?;
        for token in forbidden {
            assert!(!source.contains(token), "{file}: {token}");
        }
    }
    Ok(())
}

#[test]
fn lifecycle_files_only_call_small_diagnostic_helpers() -> Result<(), String> {
    let load = read("src/playback/backends/native/load.rs")?;
    let control = read("src/playback/backends/native/control.rs")?;
    let engine = read("src/playback/backends/native/engine.rs")?;
    let stop = read("src/playback/backends/native/stop.rs")?;

    assert!(load.contains("tap_diagnostic::open_after_decoder_open"));
    assert!(control.contains("tap_diagnostic::reset_after_seek_success"));
    assert!(stop.contains("tap_diagnostic::close_on_stop"));
    assert!(engine.contains("tap_diagnostic::close_on_shutdown"));
    assert!(!load.contains("NativePipelineRouteTapDiagnosticPolicy"));
    assert!(!control.contains("NativePipelineRouteTapDiagnosticPolicy"));
    assert!(!engine.contains("NativePipelineRouteTapDiagnosticPolicy"));
    assert!(!stop.contains("NativePipelineRouteTapDiagnosticPolicy"));
    Ok(())
}

#[test]
fn lifecycle_diagnostic_failures_are_explicitly_contained() -> Result<(), String> {
    let open = read("src/playback/backends/native/tap_diagnostic/open.rs")?;
    let seek = read("src/playback/backends/native/tap_diagnostic/seek.rs")?;
    let close = read("src/playback/backends/native/tap_diagnostic/close.rs")?;

    assert!(open.contains("let _ = policy.open_new_track"));
    assert!(seek.contains("let _ = policy.reset_on_seek"));
    assert!(close.contains("let _ = policy.close_detach_on_stop"));
    assert!(close.contains("let _ = policy.close_detach_on_shutdown"));
    assert!(!open.contains('?'));
    assert!(!seek.contains('?'));
    assert!(!close.contains('?'));
    Ok(())
}

#[test]
fn forbidden_consumers_have_no_tap_diagnostic_reference() -> Result<(), String> {
    let files = [
        "src/playback/manager.rs",
        "src/playback/manager_queue.rs",
        "src/playback/queue.rs",
        "src/playback/commands.rs",
        "src/playback/events.rs",
        "src/playback/output/mod.rs",
        "src/playback/output_sink.rs",
    ];

    for file in files {
        assert!(!read(file)?.contains("tap_diagnostic"), "{file}");
    }
    Ok(())
}
