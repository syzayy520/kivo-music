//! Runtime mode for WASAPI device buffer writer.
//!
//! Pure state types for distinguishing placeholder vs real runtime.
//! No real WASAPI calls, no COM objects, no audio data processing.

/// Runtime mode for the device buffer writer.
///
/// Distinguishes between placeholder simulation and future real runtime.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum RuntimeMode {
    /// Placeholder mode — simulated buffer behavior.
    #[default]
    Placeholder,
    /// Real runtime mode — actual WASAPI calls (future).
    Real,
    /// Not yet connected to a runtime.
    NotConnected,
    /// Boundary unavailable — runtime cannot be initialized.
    BoundaryUnavailable,
}

impl RuntimeMode {
    /// Returns true if this is placeholder mode.
    pub fn is_placeholder(&self) -> bool {
        matches!(self, Self::Placeholder)
    }

    /// Returns true if this is real runtime mode.
    pub fn is_real(&self) -> bool {
        matches!(self, Self::Real)
    }

    /// Returns true if not yet connected.
    pub fn is_not_connected(&self) -> bool {
        matches!(self, Self::NotConnected)
    }

    /// Returns true if boundary is unavailable.
    pub fn is_boundary_unavailable(&self) -> bool {
        matches!(self, Self::BoundaryUnavailable)
    }

    /// Returns true if the writer can accept requests.
    pub fn can_accept_requests(&self) -> bool {
        matches!(self, Self::Placeholder | Self::Real)
    }
}

/// Runtime kind — high-level classification of the runtime.
///
/// Used for logging and diagnostics.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum RuntimeKind {
    /// Simulated runtime for testing.
    #[default]
    Simulated,
    /// Real WASAPI runtime.
    Wasapi,
    /// Unknown runtime kind.
    Unknown,
}

impl RuntimeKind {
    /// Returns true if this is a simulated runtime.
    pub fn is_simulated(&self) -> bool {
        matches!(self, Self::Simulated)
    }

    /// Returns true if this is a WASAPI runtime.
    pub fn is_wasapi(&self) -> bool {
        matches!(self, Self::Wasapi)
    }
}

/// Readiness state of the runtime.
///
/// Indicates whether the runtime is ready to accept requests.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Readiness {
    /// Runtime is ready to accept requests.
    #[default]
    Ready,
    /// Runtime is not ready (e.g., not initialized).
    NotReady,
    /// Runtime is temporarily unavailable.
    TemporarilyUnavailable,
    /// Runtime is permanently unavailable.
    PermanentlyUnavailable,
}

impl Readiness {
    /// Returns true if the runtime is ready.
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns true if the runtime is not ready.
    pub fn is_not_ready(&self) -> bool {
        matches!(self, Self::NotReady)
    }

    /// Returns true if the runtime is temporarily unavailable.
    pub fn is_temporarily_unavailable(&self) -> bool {
        matches!(self, Self::TemporarilyUnavailable)
    }

    /// Returns true if the runtime is permanently unavailable.
    pub fn is_permanently_unavailable(&self) -> bool {
        matches!(self, Self::PermanentlyUnavailable)
    }

    /// Returns true if the runtime can potentially become ready.
    pub fn can_become_ready(&self) -> bool {
        matches!(self, Self::NotReady | Self::TemporarilyUnavailable)
    }
}
