use crate::playback::errors::PlaybackError;

use super::classification::OutputSubmitErrorClassification;

// Contract-only until a route adapter or drain observation ticket handles classification results.
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum OutputSubmitErrorClassificationResult {
    Classified(OutputSubmitErrorClassification),
    Unmapped(PlaybackError),
}
