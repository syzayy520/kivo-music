use crate::playback::errors::PlaybackError;

use super::super::ProductionOutputRouteSinkFailure;
use super::operation::ProductionOutputRouteSinkFailureOperation;
use super::result::ProductionOutputRouteSinkFailureMappingResult;

#[allow(dead_code)]
pub(crate) struct ProductionOutputRouteSinkFailureMapper;

#[allow(dead_code)]
impl ProductionOutputRouteSinkFailureMapper {
    pub(crate) fn map_submit_frame_error(
        &self,
        error: PlaybackError,
    ) -> ProductionOutputRouteSinkFailureMappingResult {
        match error {
            PlaybackError::Output(message) => {
                let operation = ProductionOutputRouteSinkFailureOperation::SubmitFrame;
                ProductionOutputRouteSinkFailureMappingResult::Mapped(
                    ProductionOutputRouteSinkFailure::new(operation.as_str(), message),
                )
            }
            other => ProductionOutputRouteSinkFailureMappingResult::Unmapped(other),
        }
    }
}
