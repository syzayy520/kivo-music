use crate::playback::errors::PlaybackError;

use super::super::ProductionOutputRouteSinkFailure;

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum ProductionOutputRouteSinkFailureMappingResult {
    Mapped(ProductionOutputRouteSinkFailure),
    Unmapped(PlaybackError),
}
