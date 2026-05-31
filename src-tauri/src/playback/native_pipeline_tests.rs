use super::decoder_request::AudioDecoderOpenRequest;
use super::decoder_runtime_state::DecoderRuntimeState;
use super::errors::PlaybackError;
use super::native_pipeline::NativePipeline;
use super::output::OutputSettings;

fn request() -> AudioDecoderOpenRequest {
    AudioDecoderOpenRequest {
        track_id: "track-77".to_string(),
        source_path: "C:/Music/track-77.flac".to_string(),
    }
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
