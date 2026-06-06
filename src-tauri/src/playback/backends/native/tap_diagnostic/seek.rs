use crate::playback::native_pipeline::NativePipeline;

use super::state::TapDiagnosticState;

pub(in crate::playback::backends::native) fn reset_after_seek_success(
    state: &mut TapDiagnosticState,
    pipeline: &mut NativePipeline,
) {
    if let Some(policy) = state.policy_mut() {
        let _ = policy.reset_on_seek(pipeline);
    }
}
