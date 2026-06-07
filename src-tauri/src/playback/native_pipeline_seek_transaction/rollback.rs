use super::super::native_pipeline::NativePipeline;
use super::snapshot::NativePipelineSeekSnapshot;

/// Restore observable wrapper state from snapshot after a failed seek.
///
/// Restores: decoder_session, decoder_state, last_decoded_frame, buffer, clock.
/// Does NOT restore decoder object private internal state.
pub(super) fn restore(pipeline: &mut NativePipeline, snapshot: NativePipelineSeekSnapshot) {
    pipeline.state.decoder_session = snapshot.decoder_session().clone();
    pipeline.state.decoder_state = snapshot.decoder_state().clone();
    pipeline.state.last_decoded_frame = snapshot.last_decoded_frame().clone();
    pipeline.buffer = snapshot.buffer().clone();
    pipeline.clock = snapshot.clock().clone();
}
