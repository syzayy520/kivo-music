use crate::playback::decoder::AudioStreamInfo;
use crate::playback::native_pipeline::NativePipeline;

use super::state::TapDiagnosticState;

pub(in crate::playback::backends::native) fn open_after_decoder_open(
    state: &mut TapDiagnosticState,
    pipeline: &mut NativePipeline,
    stream: AudioStreamInfo,
) {
    if let Some(policy) = state.policy_mut() {
        let _ = policy.open_new_track(pipeline, stream);
    }
}
