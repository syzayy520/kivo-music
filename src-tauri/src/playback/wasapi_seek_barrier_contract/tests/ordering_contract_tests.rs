//! Ordering contract tests.
//!
//! Pure contract semantics only. No runtime, WASAPI, NativePipeline,
//! seek, manager, command, event, or frontend tests.

use super::super::ordering::WasapiSeekBarrierOrdering;

#[test]
fn two_phase_render_barrier_requires_external_decoder_seek_success() {
    let ordering = WasapiSeekBarrierOrdering::TwoPhaseRenderBarrier;
    assert!(ordering.requires_external_decoder_seek_success());
    assert!(ordering.requires_future_orchestration());
    assert!(!ordering.is_render_side_only());
}

#[test]
fn render_barrier_ack_is_render_side_only() {
    let ordering = WasapiSeekBarrierOrdering::QueueDrainBeforeRingBufferReset;
    assert!(ordering.is_render_side_only());
    assert!(!ordering.requires_external_decoder_seek_success());
    assert!(ordering.requires_future_orchestration());
}
