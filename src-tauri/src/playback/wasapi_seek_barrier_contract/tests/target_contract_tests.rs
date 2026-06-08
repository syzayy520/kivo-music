//! Target contract tests.
//!
//! Pure contract semantics only. No runtime, WASAPI, NativePipeline,
//! seek, manager, command, event, or frontend tests.

use super::super::barrier_target::{WasapiBarrierTarget, WasapiBarrierTargets};

#[test]
fn ring_buffer_reset_target_is_not_full_device_barrier() {
    let targets = WasapiBarrierTargets::new(&[WasapiBarrierTarget::RingBuffer]);
    assert!(targets.contains(WasapiBarrierTarget::RingBuffer));
    assert!(!targets.contains(WasapiBarrierTarget::AudioClientDeviceBuffer));
    assert!(!targets.contains(WasapiBarrierTarget::AudioRenderClientBuffer));
}

#[test]
fn target_set_preserves_insertion_order_without_dedup() {
    let targets = WasapiBarrierTargets::new(&[
        WasapiBarrierTarget::RingBuffer,
        WasapiBarrierTarget::RenderPlan,
        WasapiBarrierTarget::RingBuffer,
    ]);
    assert_eq!(targets.len(), 3);
    let collected: Vec<_> = targets.iter().copied().collect();
    assert_eq!(collected[0], WasapiBarrierTarget::RingBuffer);
    assert_eq!(collected[1], WasapiBarrierTarget::RenderPlan);
    assert_eq!(collected[2], WasapiBarrierTarget::RingBuffer);
    assert!(targets.contains(WasapiBarrierTarget::RenderPlan));
}
