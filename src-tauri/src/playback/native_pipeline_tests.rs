use super::decoder::{AudioSampleFormat, AudioStreamInfo};
use super::decoder_request::AudioDecoderOpenRequest;
use super::decoder_runtime_state::{DecoderRuntimePhase, DecoderRuntimeState};
use super::errors::PlaybackError;
use super::native_pipeline::NativePipeline;
use super::output::OutputSettings;

fn request() -> AudioDecoderOpenRequest {
    AudioDecoderOpenRequest {
        track_id: "track-77".to_string(),
        source_path: "C:/Music/track-77.flac".to_string(),
    }
}

fn stream_info() -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz: 48_000,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    }
}

#[test]
fn new_pipeline_starts_with_default_state() {
    let pipeline = NativePipeline::new();
    let state = pipeline.state();

    assert!(state.decoder_request.is_none());
    assert!(state.decoder_session.is_none());
    assert!(state.output_settings.selected_device_id.is_none());
    assert!(state.output_status.last_error.is_none());
}

#[test]
fn pipeline_can_store_request_and_state() {
    let mut pipeline = NativePipeline::new();
    let mut decoder_state = DecoderRuntimeState::idle();
    decoder_state.begin_opening();
    let mut output_settings = OutputSettings::default();
    output_settings.selected_device_id = Some("default".to_string());

    pipeline.set_decoder_request(request());
    pipeline.set_decoder_state(decoder_state.clone());
    pipeline.set_output_settings(output_settings.clone());

    let state = pipeline.state();
    assert_eq!(
        state
            .decoder_request
            .as_ref()
            .map(|item| item.track_id.as_str()),
        Some("track-77")
    );
    assert_eq!(
        state
            .decoder_request
            .as_ref()
            .map(|item| item.source_path.as_str()),
        Some("C:/Music/track-77.flac")
    );
    assert_eq!(
        state.output_settings.selected_device_id,
        output_settings.selected_device_id
    );
    assert_eq!(state.decoder_state.phase, decoder_state.phase);
}

#[test]
fn state_snapshot_is_cloned_and_does_not_mutate_pipeline() {
    let mut pipeline = NativePipeline::new();
    pipeline.set_decoder_request(request());

    let mut snapshot = pipeline.state();
    snapshot.decoder_request = None;
    snapshot.output_settings.selected_device_id = Some("modified".to_string());

    let state = pipeline.state();
    assert_eq!(
        state
            .decoder_request
            .as_ref()
            .map(|item| item.track_id.as_str()),
        Some("track-77")
    );
    assert!(state.output_settings.selected_device_id.is_none());
}

#[test]
fn configure_decoder_open_creates_session_and_marks_open() {
    let mut pipeline = NativePipeline::new();

    pipeline.configure_decoder_open(request(), stream_info(), 2_000);
    pipeline.update_decoder_position(4_500);
    pipeline.count_decoded_frame();

    let state = pipeline.state();
    let session = state
        .decoder_session
        .as_ref()
        .expect("decoder session should be created");

    assert_eq!(state.decoder_state.phase, DecoderRuntimePhase::Open);
    assert_eq!(session.track_id, "track-77");
    assert_eq!(session.source_path, "C:/Music/track-77.flac");
    assert_eq!(session.opened_at_ms, 2_000);
    assert_eq!(session.last_position_ms, 4_500);
    assert_eq!(session.decoded_frame_count, 1);
}

fn assert_unsupported(result: Result<(), PlaybackError>, operation: &str) {
    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(
                message,
                format!("native pipeline {operation} is not implemented yet")
            );
        }
        Err(other) => panic!("expected unsupported operation, got {other}"),
        Ok(_) => panic!("expected unsupported operation, got success"),
    }
}

#[test]
fn runtime_operations_are_typed_unsupported() {
    let mut pipeline = NativePipeline::new();

    assert_unsupported(pipeline.start(), "start");
    assert_unsupported(pipeline.submit(), "submit");
    assert_unsupported(pipeline.shutdown(), "shutdown");
}
