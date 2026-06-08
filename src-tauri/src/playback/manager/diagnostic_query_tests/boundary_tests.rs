use std::path::PathBuf;

const PRODUCTION_FILES: &[&str] = &[
    "src/playback/manager/diagnostic_query/mod.rs",
    "src/playback/manager/diagnostic_query/query.rs",
    "src/playback/manager/diagnostic_query/snapshot.rs",
];

const TEST_FILES: &[&str] = &[
    "src/playback/manager/diagnostic_query_tests/mod.rs",
    "src/playback/manager/diagnostic_query_tests/disabled_tests.rs",
    "src/playback/manager/diagnostic_query_tests/snapshot_tests.rs",
    "src/playback/manager/diagnostic_query_tests/boundary_tests.rs",
];

fn source_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn read(relative: &str) -> Result<String, String> {
    std::fs::read_to_string(source_path(relative)).map_err(|error| format!("{error}"))
}

#[test]
fn manager_diagnostic_files_stay_in_family_tree() {
    for file in PRODUCTION_FILES.iter().chain(TEST_FILES.iter()) {
        assert!(source_path(file).exists(), "{file}");
    }
}

#[test]
fn manager_diagnostic_manager_rs_only_registers_diagnostic_module() -> Result<(), String> {
    let source = read("src/playback/manager/mod.rs")?;

    assert!(source.contains("mod diagnostic_query;"));
    assert!(source.contains("mod diagnostic_query_tests;"));
    assert!(!source.contains("TapDiagnosticReportSnapshot"));
    assert!(source.lines().count() <= 220);
    Ok(())
}

#[test]
fn manager_diagnostic_query_avoids_forbidden_exposure_paths() -> Result<(), String> {
    let forbidden = [
        "tauri::command",
        "commands",
        "events",
        "PlaybackState",
        "WorkerState",
        "OutputRuntimeStatus",
        "state.error",
        "output_status",
        "clock",
        "OutputSink",
        "Wasapi",
        "real_transport",
        "frontend",
    ];

    for file in PRODUCTION_FILES {
        let source = read(file)?;
        for token in forbidden {
            assert!(!source.contains(token), "{file}: {token}");
        }
    }
    Ok(())
}

#[test]
fn manager_diagnostic_query_does_not_expand_product_exposure_surfaces() -> Result<(), String> {
    for file in [
        "src/playback/commands/mod.rs",
        "src/playback/events/mod.rs",
        "src/playback/state/mod.rs",
        "src/playback/worker/state.rs",
        "src/playback/engine/mod.rs",
        "src/playback/backends/mpv.rs",
    ] {
        assert!(!read(file)?.contains("tap_diagnostic"), "{file}");
    }
    Ok(())
}

#[test]
fn manager_diagnostic_snapshot_is_owned_compact_and_has_no_forbidden_payloads() -> Result<(), String>
{
    let source = read("src/playback/manager/diagnostic_query/snapshot.rs")?;

    assert!(source.contains("#[derive(Debug, Clone, PartialEq, Eq)]"));
    assert!(!source.contains("Vec<f32>"));
    assert!(!source.contains("source_path"));
    assert!(!source.contains("PlaybackTrack"));
    assert!(!source.contains("&'"));
    assert!(!source.contains("AudioRoutePipelineTap,"));
    assert!(!source.contains("NativePipelineRouteTapDiagnosticPolicy"));
    Ok(())
}
