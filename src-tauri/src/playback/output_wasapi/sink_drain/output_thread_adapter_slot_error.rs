//! Error type for the output-thread adapter slot.

use super::render_step_adapter_error::WasapiRenderStepAdapterError;

/// Error from an output-thread adapter slot invocation.
///
/// Wraps the underlying render step adapter error without swallowing it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WasapiOutputThreadAdapterSlotError {
    /// The underlying render step adapter returned an error.
    Adapter(WasapiRenderStepAdapterError),
}

impl From<WasapiRenderStepAdapterError> for WasapiOutputThreadAdapterSlotError {
    fn from(e: WasapiRenderStepAdapterError) -> Self {
        Self::Adapter(e)
    }
}
