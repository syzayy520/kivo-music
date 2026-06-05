//! Error type for the render step adapter.

use super::render_loop_error::WasapiRenderLoopStepError;

/// Error from a render step adapter invocation.
///
/// Wraps the underlying render loop step error without swallowing it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WasapiRenderStepAdapterError {
    /// The underlying render loop step returned an error.
    Step(WasapiRenderLoopStepError),
}

impl From<WasapiRenderLoopStepError> for WasapiRenderStepAdapterError {
    fn from(e: WasapiRenderLoopStepError) -> Self {
        Self::Step(e)
    }
}
