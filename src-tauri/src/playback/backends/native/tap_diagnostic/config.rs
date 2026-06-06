use crate::playback::native_pipeline_route_tap_diagnostic_policy::{
    NativePipelineRouteTapDiagnosticPolicy, NativeTapDiagnosticConfig,
};

use super::state::TapDiagnosticState;

pub(in crate::playback::backends::native) fn state_from_config(
    config: Option<NativeTapDiagnosticConfig>,
) -> TapDiagnosticState {
    let mut state = TapDiagnosticState::default();
    if let Some(config) = config {
        state.install(NativePipelineRouteTapDiagnosticPolicy::new(config));
    }
    state
}
