use crate::playback::errors::PlaybackError;
use crate::playback::output_submit_error::classifier::OutputSubmitErrorClassifier;
use crate::playback::output_submit_error::result::OutputSubmitErrorClassificationResult;

#[derive(Debug)]
pub(in crate::playback) struct NativePipelineDrainSubmitErrorObservation {
    classification_result: OutputSubmitErrorClassificationResult,
}

impl NativePipelineDrainSubmitErrorObservation {
    fn new(classification_result: OutputSubmitErrorClassificationResult) -> Self {
        Self {
            classification_result,
        }
    }

    #[cfg(test)]
    pub(in crate::playback) fn classification_result(
        &self,
    ) -> &OutputSubmitErrorClassificationResult {
        &self.classification_result
    }

    fn into_classification_result(self) -> OutputSubmitErrorClassificationResult {
        self.classification_result
    }
}

#[derive(Debug)]
pub(in crate::playback) struct NativePipelineDrainObservedSubmitError {
    observation: NativePipelineDrainSubmitErrorObservation,
}

impl NativePipelineDrainObservedSubmitError {
    #[cfg(test)]
    pub(in crate::playback) fn observation(&self) -> &NativePipelineDrainSubmitErrorObservation {
        &self.observation
    }

    pub(in crate::playback) fn into_error(self) -> PlaybackError {
        match self.observation.into_classification_result() {
            OutputSubmitErrorClassificationResult::Classified(classification) => {
                PlaybackError::Output(classification.message().to_owned())
            }
            OutputSubmitErrorClassificationResult::Unmapped(error) => error,
        }
    }
}

pub(in crate::playback) fn observe_submit_frame_error(
    error: PlaybackError,
) -> NativePipelineDrainObservedSubmitError {
    let classifier = OutputSubmitErrorClassifier;
    let classification_result = classifier.classify_submit_frame_error(error);

    NativePipelineDrainObservedSubmitError {
        observation: NativePipelineDrainSubmitErrorObservation::new(classification_result),
    }
}
