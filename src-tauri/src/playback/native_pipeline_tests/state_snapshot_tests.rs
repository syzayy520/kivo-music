use super::*;

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
