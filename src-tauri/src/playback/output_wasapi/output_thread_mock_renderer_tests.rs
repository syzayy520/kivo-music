use super::output_thread_mock_renderer::OutputThreadMockRenderer;

#[test]
fn full_renderer_has_no_capacity() {
    let renderer = OutputThreadMockRenderer::full();
    assert!(!renderer.has_capacity());
    assert_eq!(renderer.free_frames(), 0);
}

#[test]
fn renderer_with_free_frames_has_capacity() {
    let renderer = OutputThreadMockRenderer::with_free_frames(512);
    assert!(renderer.has_capacity());
    assert_eq!(renderer.free_frames(), 512);
}

#[test]
fn free_frames_accessor_returns_value() {
    let renderer = OutputThreadMockRenderer::new(1024);
    assert_eq!(renderer.free_frames(), 1024);
}
