use crate::playback::native_pipeline::NativePipeline;

use super::state::TapDiagnosticState;

pub(in crate::playback::backends::native) fn close_on_stop(
    state: &mut TapDiagnosticState,
    pipeline: &mut NativePipeline,
) {
    if let Some(policy) = state.policy_mut() {
        let _ = policy.close_detach_on_stop(pipeline);
    }
}

pub(in crate::playback::backends::native) fn close_on_shutdown(
    state: &mut TapDiagnosticState,
    pipeline: &mut NativePipeline,
) {
    if let Some(policy) = state.policy_mut() {
        let _ = policy.close_detach_on_shutdown(pipeline);
    }
}
