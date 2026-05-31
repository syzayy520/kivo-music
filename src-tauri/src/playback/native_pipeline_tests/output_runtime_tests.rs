use super::*;

#[test]
fn output_settings_and_status_are_stored_in_pipeline_state() {
    let mut pipeline = NativePipeline::new();
    let mut settings = OutputSettings::default();
    settings.selected_device_id = Some("device-1".to_string());
    settings.exclusive_mode = true;
    let mut status = OutputRuntimeStatus::default();
    status.active_device_id = Some("device-1".to_string());
    status.is_open = true;

    pipeline.set_output_settings(settings.clone());
    pipeline.set_output_status(status.clone());

    let state = pipeline.state();
    assert_eq!(
        state.output_settings.selected_device_id,
        settings.selected_device_id
    );
    assert_eq!(
        state.output_settings.exclusive_mode,
        settings.exclusive_mode
    );
    assert_eq!(
        state.output_status.active_device_id,
        status.active_device_id
    );
    assert_eq!(state.output_status.is_open, status.is_open);
}

#[test]
fn note_frame_submitted_updates_output_runtime_counters() {
    let mut pipeline = NativePipeline::new();
    let frame = output_frame();

    pipeline.note_frame_submitted(&frame);
    pipeline.note_frame_submitted(&frame);

    let state = pipeline.state();
    assert!(state.output_status.is_active);
    assert_eq!(state.output_status.pending_frames, 2);
}
