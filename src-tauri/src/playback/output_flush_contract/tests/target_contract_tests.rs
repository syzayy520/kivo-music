use super::super::target::FlushTarget;

#[test]
fn target_pipeline_buffer_is_buffer_level() {
    let t = FlushTarget::PipelineBuffer;
    assert!(t.is_buffer_level());
    assert!(!t.is_device_level());
    assert!(!t.requires_render_thread());
}

#[test]
fn target_output_sink_pending_frames_is_buffer_level() {
    let t = FlushTarget::OutputSinkPendingFrames;
    assert!(t.is_buffer_level());
    assert!(!t.is_device_level());
    assert!(!t.requires_render_thread());
}

#[test]
fn target_production_output_route_input_is_neither() {
    let t = FlushTarget::ProductionOutputRouteInput;
    assert!(!t.is_buffer_level());
    assert!(!t.is_device_level());
    assert!(!t.requires_render_thread());
}

#[test]
fn target_wasapi_ring_buffer_is_device_level() {
    let t = FlushTarget::WasapiRingBuffer;
    assert!(!t.is_buffer_level());
    assert!(t.is_device_level());
    assert!(!t.requires_render_thread());
}

#[test]
fn target_render_thread_queue_requires_render_thread() {
    let t = FlushTarget::RenderThreadQueue;
    assert!(!t.is_buffer_level());
    assert!(!t.is_device_level());
    assert!(t.requires_render_thread());
}

#[test]
fn target_device_render_buffer_is_device_level_and_requires_render_thread() {
    let t = FlushTarget::DeviceRenderBuffer;
    assert!(!t.is_buffer_level());
    assert!(t.is_device_level());
    assert!(t.requires_render_thread());
}
