use std::path::{Path, PathBuf};

fn playback_path(relative_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("playback")
        .join(relative_path)
}

fn read_playback_source(relative_path: &str) -> String {
    std::fs::read_to_string(playback_path(relative_path)).expect("read playback source")
}

fn production_contract_files() -> [&'static str; 10] {
    [
        "production_output_route/mod.rs",
        "production_output_route/input/mod.rs",
        "production_output_route/input/audio_output_frame.rs",
        "production_output_route/failure/mod.rs",
        "production_output_route/failure/backpressure.rs",
        "production_output_route/failure/underrun.rs",
        "production_output_route/failure/sink_failure.rs",
        "production_output_route/failure/route_closed.rs",
        "production_output_route/failure/format_mismatch.rs",
        "production_output_route/tests/mod.rs",
    ]
}

#[test]
fn production_output_route_boundary_contract_files_avoid_forbidden_runtime_coupling() {
    let forbidden_tokens = [
        concat!("Output", "Sink"),
        concat!("native_pipeline", "_drain"),
        concat!("Playback", "State"),
        concat!("Worker", "State"),
        concat!("Output", "Runtime", "Status"),
        concat!("state", ".", "error"),
        concat!("tauri", "::", "command"),
        concat!("em", "it"),
        concat!("ev", "ent"),
        concat!("Was", "api"),
        concat!("WAS", "API"),
        concat!("real", "_transport"),
    ];

    for relative_path in production_contract_files() {
        let source = read_playback_source(relative_path);
        for token in forbidden_tokens {
            assert!(
                !source.contains(token),
                "{relative_path} must not depend on forbidden token {token}"
            );
        }
    }
}

#[test]
fn production_output_route_boundary_has_no_owner_config_lifecycle_or_bridge_skeleton() {
    for relative_path in [
        "production_output_route/owner",
        "production_output_route/config",
        "production_output_route/lifecycle",
        "production_output_route/bridge",
        "production_output_route/route.rs",
    ] {
        assert!(
            !playback_path(relative_path).exists(),
            "{relative_path} must not be created by this contract ticket"
        );
    }
}

#[test]
fn production_output_route_boundary_input_does_not_expose_samples_or_serde_contract() {
    let source = read_playback_source("production_output_route/input/audio_output_frame.rs");

    assert!(!source.contains("fn samples"));
    assert!(!source.contains("Serialize"));
    assert!(!source.contains("Deserialize"));
    assert!(!source.contains("serde"));
}
