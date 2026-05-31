use super::*;

#[test]
fn route_load_from_idle_sets_loaded_with_new_track() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Load {
        track: edge_track("edge-load-1"),
    };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);

    assert_eq!(next.phase, PlaybackWorkerPhase::Loaded);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-load-1"));
    assert_eq!(next.last_error, None);
}

#[test]
fn route_load_from_playing_sets_loaded_with_new_track() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-old-1");
    state.mark_playing();
    let command = PlaybackWorkerCommand::Load {
        track: edge_track("edge-load-2"),
    };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);

    assert_eq!(next.phase, PlaybackWorkerPhase::Loaded);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-load-2"));
    assert_eq!(next.last_error, None);
}

#[test]
fn route_load_from_paused_sets_loaded_with_new_track() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-old-2");
    state.mark_paused();
    let command = PlaybackWorkerCommand::Load {
        track: edge_track("edge-load-3"),
    };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);

    assert_eq!(next.phase, PlaybackWorkerPhase::Loaded);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-load-3"));
    assert_eq!(next.last_error, None);
}

#[test]
fn route_load_from_stopped_sets_loaded_with_new_track() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-old-3");
    state.mark_stopped();
    let command = PlaybackWorkerCommand::Load {
        track: edge_track("edge-load-4"),
    };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);

    assert_eq!(next.phase, PlaybackWorkerPhase::Loaded);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-load-4"));
    assert_eq!(next.last_error, None);
}

#[test]
fn route_load_from_failed_sets_loaded_with_new_track_and_clears_error() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-old-4");
    state.mark_failed("edge-error-before-load");
    let command = PlaybackWorkerCommand::Load {
        track: edge_track("edge-load-5"),
    };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);

    assert_eq!(next.phase, PlaybackWorkerPhase::Loaded);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-load-5"));
    assert_eq!(next.last_error, None);
}

#[test]
fn route_load_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Load {
        track: edge_track("edge-load-6"),
    };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Loaded);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-load-6"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command load is not implemented yet")
    );
}

#[test]
fn route_load_from_playing_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-before-load-ctx-1");
    state.mark_playing();
    let command = PlaybackWorkerCommand::Load {
        track: edge_track("edge-load-ctx-1"),
    };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Loaded);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-load-ctx-1"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command load is not implemented yet")
    );
}

#[test]
fn route_load_from_paused_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-before-load-ctx-2");
    state.mark_paused();
    let command = PlaybackWorkerCommand::Load {
        track: edge_track("edge-load-ctx-2"),
    };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Loaded);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-load-ctx-2"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command load is not implemented yet")
    );
}

#[test]
fn route_load_from_stopped_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-before-load-ctx-3");
    state.mark_stopped();
    let command = PlaybackWorkerCommand::Load {
        track: edge_track("edge-load-ctx-3"),
    };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Loaded);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-load-ctx-3"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command load is not implemented yet")
    );
}

#[test]
fn route_load_from_failed_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-before-load-ctx-4");
    state.mark_failed("edge-load-runtime-error-before-ctx");
    let command = PlaybackWorkerCommand::Load {
        track: edge_track("edge-load-ctx-4"),
    };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Loaded);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-load-ctx-4"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command load is not implemented yet")
    );
}
