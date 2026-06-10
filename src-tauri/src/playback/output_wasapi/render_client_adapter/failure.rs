//! Adapter failure types and conversion.
//!
//! Defines adapter-internal failure reasons and converts them to the
//! boundary contract's `RenderClientFailure`. This keeps adapter-specific
//! failure details isolated from the boundary contract.
//!
//! Future real WASAPI integration will add HRESULT-based failure variants here.

use crate::playback::output_wasapi::render_client_boundary::RenderClientFailure;

/// Adapter-internal failure reasons.
///
/// These represent failures specific to the adapter layer. They are
/// converted to `RenderClientFailure` when crossing the boundary.
/// No real Windows error codes — pure metadata only.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdapterFailure {
    /// Adapter is not connected to a device.
    NotConnected,
    /// Adapter lifecycle prevents this operation.
    InvalidLifecycleState {
        /// Current lifecycle state description.
        current: String,
        /// Required lifecycle state description.
        required: String,
    },
    /// Device was lost.
    DeviceLost,
    /// Internal adapter error.
    Internal {
        /// Error description.
        description: String,
    },
}

impl std::fmt::Display for AdapterFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotConnected => write!(f, "adapter not connected"),
            Self::InvalidLifecycleState { current, required } => {
                write!(
                    f,
                    "adapter invalid lifecycle state: current={}, required={}",
                    current, required
                )
            }
            Self::DeviceLost => write!(f, "adapter device lost"),
            Self::Internal { description } => {
                write!(f, "adapter internal error: {}", description)
            }
        }
    }
}

impl AdapterFailure {
    /// Converts this adapter failure to a boundary `RenderClientFailure`.
    pub fn to_render_client_failure(&self) -> RenderClientFailure {
        match self {
            Self::NotConnected => RenderClientFailure::RenderClientUnavailable,
            Self::InvalidLifecycleState { current, required } => {
                RenderClientFailure::InvalidRequest {
                    reason: format!(
                        "adapter lifecycle: current={}, required={}",
                        current, required
                    ),
                }
            }
            Self::DeviceLost => RenderClientFailure::DeviceLost,
            Self::Internal { description } => RenderClientFailure::Internal {
                description: description.clone(),
            },
        }
    }
}

impl From<AdapterFailure> for RenderClientFailure {
    fn from(failure: AdapterFailure) -> Self {
        failure.to_render_client_failure()
    }
}
