use super::native_pipeline::NativePipeline;
use super::playback_worker_command::PlaybackWorkerCommand;
use super::playback_worker_state::{PlaybackWorkerPhase, PlaybackWorkerState};

#[test]
fn route_seek_from_playing_keeps_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-1");
    state.mark_playing();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::Seek { position_ms: 9_999 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-1"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command seek is not implemented yet")
    );
}

#[test]
fn route_play_from_playing_keeps_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-2");
    state.mark_playing();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Play);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-2"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command play is not implemented yet")
    );
}

#[test]
fn route_pause_from_paused_keeps_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-3");
    state.mark_paused();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Pause);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-3"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command pause is not implemented yet")
    );
}

#[test]
fn route_resume_from_playing_keeps_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-4");
    state.mark_playing();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Resume);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-4"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command resume is not implemented yet")
    );
}

#[test]
fn route_shutdown_from_stopped_keeps_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-5");
    state.mark_stopped();

    let next = pipeline
        .route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Shutdown);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-5"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command shutdown is not implemented yet"
        )
    );
}

#[test]
fn route_stop_from_stopped_keeps_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-6");
    state.mark_stopped();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Stop);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-6"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command stop is not implemented yet")
    );
}

#[test]
fn route_set_volume_from_stopped_keeps_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-7");
    state.mark_stopped();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetVolume { level: 0.25 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-7"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}

#[test]
fn route_set_muted_from_stopped_keeps_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-8");
    state.mark_stopped();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetMuted { muted: true },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-8"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_muted is not implemented yet"
        )
    );
}

#[test]
fn route_seek_from_stopped_keeps_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-9");
    state.mark_stopped();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::Seek { position_ms: 2048 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-9"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command seek is not implemented yet")
    );
}

#[test]
fn route_play_from_stopped_moves_to_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-10");
    state.mark_stopped();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Play);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-10"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command play is not implemented yet")
    );
}

#[test]
fn route_resume_from_stopped_moves_to_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-11");
    state.mark_stopped();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Resume);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-11"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command resume is not implemented yet")
    );
}

#[test]
fn route_pause_from_stopped_moves_to_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-12");
    state.mark_stopped();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Pause);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-12"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command pause is not implemented yet")
    );
}

#[test]
fn route_play_from_failed_moves_to_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-13");
    state.mark_failed("edge-failed");

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Play);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-13"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command play is not implemented yet")
    );
}

#[test]
fn route_resume_from_failed_moves_to_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-14");
    state.mark_failed("edge-failed");

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Resume);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-14"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command resume is not implemented yet")
    );
}

#[test]
fn route_pause_from_failed_moves_to_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-15");
    state.mark_failed("edge-failed");

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Pause);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-15"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command pause is not implemented yet")
    );
}

#[test]
fn route_stop_from_failed_moves_to_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-16");
    state.mark_failed("edge-failed");

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Stop);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-16"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command stop is not implemented yet")
    );
}

#[test]
fn route_shutdown_from_failed_moves_to_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-17");
    state.mark_failed("edge-failed");

    let next = pipeline
        .route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Shutdown);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-17"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command shutdown is not implemented yet"
        )
    );
}

#[test]
fn route_seek_from_failed_keeps_failed_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-18");
    state.mark_failed("edge-failed");

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::Seek { position_ms: 8080 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Failed);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-18"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command seek is not implemented yet")
    );
}

#[test]
fn route_set_volume_from_failed_keeps_failed_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-19");
    state.mark_failed("edge-failed");

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetVolume { level: 0.66 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Failed);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-19"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}

#[test]
fn route_set_muted_from_failed_keeps_failed_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-20");
    state.mark_failed("edge-failed");

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetMuted { muted: false },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Failed);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-20"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_muted is not implemented yet"
        )
    );
}

#[test]
fn route_seek_from_idle_keeps_idle_phase() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::Seek { position_ms: 1 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Idle);
    assert_eq!(next.active_track_id, None);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command seek is not implemented yet")
    );
}

#[test]
fn route_set_volume_from_idle_keeps_idle_phase() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetVolume { level: 0.9 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Idle);
    assert_eq!(next.active_track_id, None);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}

#[test]
fn route_set_muted_from_idle_keeps_idle_phase() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetMuted { muted: true },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Idle);
    assert_eq!(next.active_track_id, None);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_muted is not implemented yet"
        )
    );
}

#[test]
fn route_pause_from_idle_moves_to_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Pause);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id, None);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command pause is not implemented yet")
    );
}

#[test]
fn route_stop_from_idle_moves_to_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Stop);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id, None);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command stop is not implemented yet")
    );
}

#[test]
fn route_shutdown_from_idle_moves_to_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let next = pipeline
        .route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Shutdown);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id, None);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command shutdown is not implemented yet"
        )
    );
}

#[test]
fn route_play_from_idle_moves_to_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Play);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id, None);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command play is not implemented yet")
    );
}

#[test]
fn route_resume_from_idle_moves_to_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Resume);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id, None);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command resume is not implemented yet")
    );
}

#[test]
fn route_set_volume_from_paused_keeps_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-21");
    state.mark_paused();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetVolume { level: 0.33 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-21"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}

#[test]
fn route_set_muted_from_paused_keeps_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-22");
    state.mark_paused();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetMuted { muted: true },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-22"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_muted is not implemented yet"
        )
    );
}

#[test]
fn route_set_volume_from_playing_keeps_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-23");
    state.mark_playing();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetVolume { level: 0.44 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-23"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}
