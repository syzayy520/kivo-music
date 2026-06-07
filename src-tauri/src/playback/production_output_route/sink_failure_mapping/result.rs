use crate::playback::errors::PlaybackError;

use super::super::ProductionOutputRouteSinkFailure;

#[derive(Debug)]
// Adapter-only until a route contract caller handles neutral submit classifications.
#[allow(dead_code)]
pub(crate) enum ProductionOutputRouteSinkFailureMappingResult {
    Mapped(ProductionOutputRouteSinkFailure),
    Unmapped(PlaybackError),
}
