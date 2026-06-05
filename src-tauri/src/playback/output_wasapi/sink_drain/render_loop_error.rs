//! Error types for render loop single-step operations.

use super::tick_error::WasapiDrainTickError;

/// Error from a render loop step.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WasapiRenderLoopStepError {
    /// The underlying manual drain tick returned an error.
    Tick(WasapiDrainTickError),
}

impl From<WasapiDrainTickError> for WasapiRenderLoopStepError {
    fn from(e: WasapiDrainTickError) -> Self {
        Self::Tick(e)
    }
}
