use super::windows_audio_render_write_plan::WindowsAudioRenderWritePlan;

#[test]
fn write_plan_allows_full_request_when_capacity_is_enough() {
    let plan = WindowsAudioRenderWritePlan::plan(256, 512);

    assert_eq!(plan.requested_frame_count, 256);
    assert_eq!(plan.available_frame_count, 512);
    assert_eq!(plan.writable_frame_count, 256);
    assert_eq!(plan.deferred_frame_count, 0);
    assert!(plan.can_write);
}

#[test]
fn write_plan_defers_overflow_when_request_exceeds_capacity() {
    let plan = WindowsAudioRenderWritePlan::plan(512, 128);

    assert_eq!(plan.requested_frame_count, 512);
    assert_eq!(plan.available_frame_count, 128);
    assert_eq!(plan.writable_frame_count, 128);
    assert_eq!(plan.deferred_frame_count, 384);
    assert!(plan.can_write);
}

#[test]
fn write_plan_blocks_when_no_capacity_is_available() {
    let plan = WindowsAudioRenderWritePlan::plan(512, 0);

    assert_eq!(plan.requested_frame_count, 512);
    assert_eq!(plan.available_frame_count, 0);
    assert_eq!(plan.writable_frame_count, 0);
    assert_eq!(plan.deferred_frame_count, 512);
    assert!(!plan.can_write);
}

#[test]
fn write_plan_blocks_zero_frame_requests() {
    let plan = WindowsAudioRenderWritePlan::plan(0, 512);

    assert_eq!(plan.requested_frame_count, 0);
    assert_eq!(plan.available_frame_count, 512);
    assert_eq!(plan.writable_frame_count, 0);
    assert_eq!(plan.deferred_frame_count, 0);
    assert!(!plan.can_write);
}
