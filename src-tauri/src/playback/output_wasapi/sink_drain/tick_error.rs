//! Error types for manual drain tick operations.

use super::error::WasapiRingBufferDrainError;

/// Error from a manual drain tick.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WasapiDrainTickError {
    /// The underlying drain helper returned an error.
    Drain(WasapiRingBufferDrainError),
}

impl From<WasapiRingBufferDrainError> for WasapiDrainTickError {
    fn from(e: WasapiRingBufferDrainError) -> Self {
        Self::Drain(e)
    }
}
