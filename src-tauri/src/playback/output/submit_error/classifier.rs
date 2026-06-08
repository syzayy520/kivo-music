use crate::playback::errors::PlaybackError;

use super::classification::OutputSubmitErrorClassification;
use super::operation::OutputSubmitOperation;
use super::result::OutputSubmitErrorClassificationResult;

// Contract-only until a route adapter or drain observation ticket adds a live caller.
#[allow(dead_code)]
pub(crate) struct OutputSubmitErrorClassifier;

// Contract-only until a route adapter or drain observation ticket observes submit failures.
#[allow(dead_code)]
impl OutputSubmitErrorClassifier {
    pub(crate) fn classify_submit_frame_error(
        &self,
        error: PlaybackError,
    ) -> OutputSubmitErrorClassificationResult {
        match error {
            PlaybackError::Output(message) => OutputSubmitErrorClassificationResult::Classified(
                OutputSubmitErrorClassification::new(OutputSubmitOperation::SubmitFrame, message),
            ),
            other => OutputSubmitErrorClassificationResult::Unmapped(other),
        }
    }
}
