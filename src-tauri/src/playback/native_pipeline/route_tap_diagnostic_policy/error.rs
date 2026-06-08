use crate::playback::audio_route_pipeline_tap::AudioRoutePipelineTapError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativeTapDiagnosticPolicyError {
    InvalidDiagnosticRouteCapacityFrames,
    TapCreation(AudioRoutePipelineTapError),
    TapReset(AudioRoutePipelineTapError),
}
