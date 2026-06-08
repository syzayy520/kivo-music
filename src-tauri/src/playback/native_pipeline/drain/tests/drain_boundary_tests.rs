use std::path::Path;

use crate::playback::errors::PlaybackError;
use crate::playback::native_pipeline::NativePipeline;

fn playback_source_path(path: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("playback")
        .join(path)
}

fn playback_source(path: &str) -> String {
    std::fs::read_to_string(playback_source_path(path)).expect("read playback source")
}

#[test]
fn drain_boundary_empty_buffer_keeps_exact_backend_error() {
    let mut pipeline = NativePipeline::new();

    let result = pipeline.drain_next_frame_to_output();

    match result {
        Err(PlaybackError::Backend(message)) => {
            assert_eq!(message, "pipeline buffer is empty, no frame to drain");
        }
        Err(other) => panic!("expected backend error, got {other}"),
        Ok(_) => panic!("expected backend error, got success"),
    }
}

#[test]
fn drain_boundary_uses_strategy_a_without_duplicate_module_root() {
    let file_root = playback_source_path("native_pipeline_drain.rs");
    let directory_root = playback_source_path("native_pipeline_drain/mod.rs");

    assert!(file_root.exists());
    assert!(!directory_root.exists());

    let root = playback_source("native_pipeline_drain.rs");
    for module_name in ["clock", "error", "frame", "status", "submit"] {
        assert!(root.contains(&format!("mod {module_name};")));
    }
}

#[test]
fn drain_boundary_root_is_thin_orchestration_only() {
    let root = playback_source("native_pipeline_drain.rs");

    assert!(root.contains("self.drain_output_frame()?"));
    assert!(root.contains("self.submit_drained_output_frame(frame)"));
    assert!(root.contains("self.apply_drain_submit_success_status(status)"));
    assert!(root.contains("self.apply_drain_success_clock_position(drained_position_ms)"));
    assert!(root.contains("error::handle_drain_submit_error(self, error)"));
    assert!(!root.contains("self.output.submit_frame("));
    assert!(!root.contains("self.state.output_status ="));
    assert!(!root.contains("self.clock.start_at("));
    assert!(!root.contains("self.clock.set_position("));
    assert!(!root.contains("pipeline buffer is empty, no frame to drain"));
}

#[test]
fn drain_boundary_submodules_keep_single_responsibility_markers() {
    let frame = playback_source("native_pipeline_drain/frame.rs");
    let submit = playback_source("native_pipeline_drain/submit.rs");
    let status = playback_source("native_pipeline_drain/status.rs");
    let clock = playback_source("native_pipeline_drain/clock.rs");
    let error = playback_source("native_pipeline_drain/error.rs");

    assert!(frame.contains("drain_next_frame()"));
    assert!(frame.contains("empty_buffer_error"));
    assert!(!frame.contains("submit_frame"));
    assert!(!frame.contains("output_status"));
    assert!(!frame.contains("clock"));

    assert!(submit.contains("self.output.submit_frame(frame)"));
    assert!(!submit.contains("output_status"));
    assert!(!submit.contains("start_at"));
    assert!(!submit.contains("set_position"));

    assert!(status.contains("self.state.output_status = status"));
    assert!(status.contains("self.state.output_status = self.output.status()"));
    assert!(!status.contains("start_at"));
    assert!(!status.contains("set_position"));

    assert!(clock.contains("self.clock.is_started()"));
    assert!(clock.contains("self.clock.set_position(drained_position_ms)"));
    assert!(clock.contains("self.clock.start_at(drained_position_ms)"));
    assert!(!clock.contains("output_status"));
    assert!(!clock.contains("submit_frame"));

    assert!(error.contains("pipeline buffer is empty, no frame to drain"));
    assert!(error.contains("refresh_drain_submit_error_status()"));
    assert!(error.contains("Err(error)"));
}
