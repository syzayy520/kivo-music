use super::error::NativeTapDiagnosticPolicyError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeTapDiagnosticConfig {
    pub diagnostic_route_capacity_frames: u32,
}

impl NativeTapDiagnosticConfig {
    pub fn new(
        diagnostic_route_capacity_frames: u32,
    ) -> Result<Self, NativeTapDiagnosticPolicyError> {
        if diagnostic_route_capacity_frames == 0 {
            return Err(NativeTapDiagnosticPolicyError::InvalidDiagnosticRouteCapacityFrames);
        }
        Ok(Self {
            diagnostic_route_capacity_frames,
        })
    }
}
