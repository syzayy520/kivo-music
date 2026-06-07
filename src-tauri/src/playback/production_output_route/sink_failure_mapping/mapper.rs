use crate::playback::output_submit_error::classification::OutputSubmitErrorClassification;
use crate::playback::output_submit_error::operation::OutputSubmitOperation;
use crate::playback::output_submit_error::result::OutputSubmitErrorClassificationResult;

use super::super::ProductionOutputRouteSinkFailure;
use super::result::ProductionOutputRouteSinkFailureMappingResult;

// Adapter-only until a route contract caller consumes neutral submit classifications.
#[allow(dead_code)]
pub(crate) struct ProductionOutputRouteSinkFailureMapper;

// Adapter-only until a route contract caller consumes neutral submit classifications.
#[allow(dead_code)]
impl ProductionOutputRouteSinkFailureMapper {
    pub(in crate::playback) fn map_output_submit_classification_ref(
        &self,
        classification: &OutputSubmitErrorClassification,
    ) -> ProductionOutputRouteSinkFailureMappingResult {
        let operation = classification.operation();
        match operation {
            OutputSubmitOperation::SubmitFrame => {
                ProductionOutputRouteSinkFailureMappingResult::Mapped(
                    ProductionOutputRouteSinkFailure::new(
                        operation.as_str(),
                        classification.message().to_owned(),
                    ),
                )
            }
        }
    }

    pub(crate) fn map_output_submit_classification(
        &self,
        classification: OutputSubmitErrorClassification,
    ) -> ProductionOutputRouteSinkFailureMappingResult {
        let operation = classification.operation();
        match operation {
            OutputSubmitOperation::SubmitFrame => {
                ProductionOutputRouteSinkFailureMappingResult::Mapped(
                    ProductionOutputRouteSinkFailure::new(
                        operation.as_str(),
                        classification.message().to_owned(),
                    ),
                )
            }
        }
    }

    pub(in crate::playback) fn map_output_submit_classification_result_ref(
        &self,
        result: &OutputSubmitErrorClassificationResult,
    ) -> ProductionOutputRouteSinkFailureMappingResult {
        match result {
            OutputSubmitErrorClassificationResult::Classified(classification) => {
                self.map_output_submit_classification_ref(classification)
            }
            OutputSubmitErrorClassificationResult::Unmapped(error) => {
                ProductionOutputRouteSinkFailureMappingResult::Unmapped(error.clone())
            }
        }
    }

    pub(crate) fn map_output_submit_classification_result(
        &self,
        result: OutputSubmitErrorClassificationResult,
    ) -> ProductionOutputRouteSinkFailureMappingResult {
        match result {
            OutputSubmitErrorClassificationResult::Classified(classification) => {
                self.map_output_submit_classification(classification)
            }
            OutputSubmitErrorClassificationResult::Unmapped(error) => {
                ProductionOutputRouteSinkFailureMappingResult::Unmapped(error)
            }
        }
    }
}
