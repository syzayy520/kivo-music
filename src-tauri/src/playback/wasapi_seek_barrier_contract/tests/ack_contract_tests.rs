//! Ack contract tests.
//!
//! Pure contract semantics only. No runtime, WASAPI, NativePipeline,
//! seek, manager, command, event, or frontend tests.

use super::super::ack::{QueueDrainAck, RenderBarrierAck, ResetDeviceAck, StaleGenerationAck};
use super::super::failure::WasapiSeekBarrierFailure;
use super::super::generation::{AckGeneration, SeekOutputGeneration, StaleGeneration};
use super::super::ordering::WasapiSeekBarrierOrdering;

#[test]
fn render_barrier_completed_does_not_allow_timeline_commit() {
    let gen = SeekOutputGeneration::new(1);
    let ack = RenderBarrierAck::completed(AckGeneration::new(gen.value()));
    assert!(ack.satisfies_render_barrier(gen));
    let ordering = WasapiSeekBarrierOrdering::TwoPhaseRenderBarrier;
    assert!(ordering.requires_future_orchestration());
    assert!(ordering.requires_external_decoder_seek_success());
}

#[test]
fn stale_ack_never_satisfies_render_barrier() {
    let expected = SeekOutputGeneration::new(5);
    let ack = RenderBarrierAck::stale_generation(AckGeneration::new(3));
    assert!(!ack.satisfies_render_barrier(expected));
    assert!(ack.is_stale_generation());
}

#[test]
fn reset_device_ack_must_match_generation() {
    let gen = SeekOutputGeneration::new(7);
    let matching = ResetDeviceAck::completed(AckGeneration::new(7));
    let mismatched = ResetDeviceAck::completed(AckGeneration::new(8));
    assert!(matching.satisfies_device_reset_barrier(gen));
    assert!(!mismatched.satisfies_device_reset_barrier(gen));
}

#[test]
fn queue_drain_ack_blocks_stale_generation() {
    let expected = SeekOutputGeneration::new(4);
    let stale = QueueDrainAck::stale_generation(AckGeneration::new(2));
    assert!(!stale.satisfies_queue_drain_barrier(expected));
    assert!(stale.is_stale_generation());
    let mismatch = StaleGeneration::new(SeekOutputGeneration::new(4), SeekOutputGeneration::new(2));
    let detailed = StaleGenerationAck::new(AckGeneration::new(2), mismatch);
    assert_eq!(detailed.mismatch().expected.value(), 4);
    assert_eq!(detailed.mismatch().actual.value(), 2);
}

#[test]
fn unsupported_barrier_blocks_completion() {
    let gen = SeekOutputGeneration::new(2);
    let ack = RenderBarrierAck::unsupported(AckGeneration::new(gen.value()));
    assert!(ack.is_unsupported());
    assert!(!ack.satisfies_render_barrier(gen));
    let fail = WasapiSeekBarrierFailure::UnsupportedBarrier("target not supported".to_string());
    assert!(!fail.is_stale_generation());
    assert!(!fail.is_device_lost());
}
