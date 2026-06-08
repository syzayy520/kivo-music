use super::super::ordering::FlushOrdering;

#[test]
fn ordering_output_before_decoder_seek_flushes_before() {
    let o = FlushOrdering::OutputBeforeDecoderSeek;
    assert!(o.flushes_output_before_seek());
    assert!(!o.flushes_output_after_seek());
    assert!(!o.is_two_phase());
}

#[test]
fn ordering_decoder_seek_before_output_flushes_after() {
    let o = FlushOrdering::DecoderSeekBeforeOutput;
    assert!(!o.flushes_output_before_seek());
    assert!(o.flushes_output_after_seek());
    assert!(!o.is_two_phase());
}

#[test]
fn ordering_two_phase_barrier_flushes_both() {
    let o = FlushOrdering::TwoPhaseBarrier;
    assert!(o.flushes_output_before_seek());
    assert!(o.flushes_output_after_seek());
    assert!(o.is_two_phase());
}
