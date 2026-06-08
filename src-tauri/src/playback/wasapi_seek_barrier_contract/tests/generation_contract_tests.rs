//! Generation contract tests.
//!
//! Pure contract semantics only. No runtime, WASAPI, NativePipeline,
//! seek, manager, command, event, or frontend tests.

use super::super::generation::{
    AckGeneration, FrameGeneration, RenderEpoch, SeekOutputGeneration, StaleGeneration,
};
use super::super::render_epoch::{
    RenderEpochDecision, RenderEpochMismatch, RenderEpochObservedGeneration,
};

#[test]
fn stale_generation_expected_actual_mismatch() {
    let stale = StaleGeneration::new(SeekOutputGeneration::new(4), SeekOutputGeneration::new(2));
    assert_eq!(stale.expected.value(), 4);
    assert_eq!(stale.actual.value(), 2);
}

#[test]
fn ack_generation_value_vs_seek_output_generation_value() {
    let ack_gen = AckGeneration::new(5);
    let seek_gen = SeekOutputGeneration::new(5);
    assert_eq!(ack_gen.value(), seek_gen.value());
}

#[test]
fn render_epoch_newtype_separated_from_decision_and_mismatch() {
    let active = RenderEpoch::new(3);
    let stale_frame = FrameGeneration::new(1);
    let observed = RenderEpochObservedGeneration::Frame(stale_frame);
    let mismatch = RenderEpochMismatch { active, observed };
    assert!(mismatch.is_frame());
    assert!(!mismatch.is_command());
    let decision = RenderEpochDecision::RejectStale;
    assert_eq!(decision, RenderEpochDecision::RejectStale);
}
