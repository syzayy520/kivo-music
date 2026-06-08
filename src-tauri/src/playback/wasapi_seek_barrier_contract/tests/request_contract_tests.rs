//! Request contract tests.
//!
//! Pure contract semantics only. No runtime, WASAPI, NativePipeline,
//! seek, manager, command, event, or frontend tests.

use super::super::ack::RenderBarrierAck;
use super::super::barrier_request::{WasapiSeekBarrierReason, WasapiSeekBarrierRequest};
use super::super::barrier_target::{WasapiBarrierTarget, WasapiBarrierTargets};
use super::super::generation::{AckGeneration, RenderEpoch, SeekOutputGeneration};
use super::super::ordering::WasapiSeekBarrierOrdering;
use super::super::queue_policy::{StaleCommandPolicy, WasapiQueuePolicy};
use super::super::reset_policy::WasapiResetPolicy;

#[test]
fn barrier_request_binds_generation_targets_ordering_and_policies() {
    let request = WasapiSeekBarrierRequest {
        generation: SeekOutputGeneration::new(10),
        render_epoch: RenderEpoch::new(5),
        reason: WasapiSeekBarrierReason::PlayingSeek,
        targets: WasapiBarrierTargets::new(&[
            WasapiBarrierTarget::RuntimeQueue,
            WasapiBarrierTarget::RingBuffer,
            WasapiBarrierTarget::AudioClientDeviceBuffer,
        ]),
        queue_policy: WasapiQueuePolicy::RejectStaleCommands,
        stale_command_policy: StaleCommandPolicy::ConvertToStaleAck,
        reset_policy: WasapiResetPolicy::StopResetStart,
        ordering: WasapiSeekBarrierOrdering::TwoPhaseRenderBarrier,
    };
    assert_eq!(request.generation.value(), 10);
    assert_eq!(request.render_epoch.value(), 5);
    assert!(request.is_playing_seek());
    assert!(request.targets.contains(WasapiBarrierTarget::RingBuffer));
    assert!(request
        .targets
        .contains(WasapiBarrierTarget::AudioClientDeviceBuffer));
    assert!(request.reset_policy.requires_stop_start_cycle());
    assert!(request.ordering.requires_external_decoder_seek_success());
}

#[test]
fn paused_seek_still_requires_render_barrier_ack() {
    let gen = SeekOutputGeneration::new(3);
    let request = WasapiSeekBarrierRequest {
        generation: gen,
        render_epoch: RenderEpoch::new(1),
        reason: WasapiSeekBarrierReason::PausedSeek,
        targets: WasapiBarrierTargets::new(&[WasapiBarrierTarget::RingBuffer]),
        queue_policy: WasapiQueuePolicy::DrainBeforeBarrier,
        stale_command_policy: StaleCommandPolicy::Reject,
        reset_policy: WasapiResetPolicy::FlushOnly,
        ordering: WasapiSeekBarrierOrdering::QueueDrainBeforeRingBufferReset,
    };
    assert!(request.is_paused_seek());
    let ack = RenderBarrierAck::completed(AckGeneration::new(gen.value()));
    assert!(ack.satisfies_render_barrier(gen));
}
