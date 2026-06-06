use crate::playback::native_pipeline_route_tap_diagnostic_policy::NativePipelineRouteTapDiagnosticPolicy;

#[derive(Debug, Default)]
pub(in crate::playback::backends::native) struct TapDiagnosticState {
    policy: Option<NativePipelineRouteTapDiagnosticPolicy>,
}

impl TapDiagnosticState {
    pub(in crate::playback::backends::native) fn install(
        &mut self,
        policy: NativePipelineRouteTapDiagnosticPolicy,
    ) {
        self.policy = Some(policy);
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(in crate::playback::backends::native) fn policy(
        &self,
    ) -> Option<&NativePipelineRouteTapDiagnosticPolicy> {
        self.policy.as_ref()
    }

    pub(in crate::playback::backends::native) fn policy_mut(
        &mut self,
    ) -> Option<&mut NativePipelineRouteTapDiagnosticPolicy> {
        self.policy.as_mut()
    }
}
