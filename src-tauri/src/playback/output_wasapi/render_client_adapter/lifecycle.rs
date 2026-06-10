//! Adapter lifecycle state machine.
//!
//! Tracks the lifecycle of a real WASAPI render client adapter without
//! holding any real Windows resources. Pure state transitions only.

/// Lifecycle states for a render client adapter.
///
/// Represents the adapter's connection and operational state.
/// No real Windows resources are held — this is pure metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AdapterLifecycle {
    /// Adapter has not been connected to a device.
    #[default]
    NotConnected,
    /// Adapter is connected and ready to accept requests.
    Ready,
    /// The audio device was lost (e.g., unplugged).
    DeviceLost,
    /// Adapter has been explicitly closed.
    Closed,
}

impl std::fmt::Display for AdapterLifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotConnected => write!(f, "NotConnected"),
            Self::Ready => write!(f, "Ready"),
            Self::DeviceLost => write!(f, "DeviceLost"),
            Self::Closed => write!(f, "Closed"),
        }
    }
}

impl AdapterLifecycle {
    /// Returns true if the adapter can accept buffer operations.
    pub fn can_accept_requests(&self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns true if the adapter is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::DeviceLost | Self::Closed)
    }

    /// Returns true if the adapter can transition to Ready.
    pub fn can_become_ready(&self) -> bool {
        matches!(self, Self::NotConnected)
    }

    /// Returns true if the adapter is connected (Ready or DeviceLost).
    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Ready | Self::DeviceLost)
    }

    /// Transition to Ready. Returns false if not allowed.
    pub fn transition_to_ready(&mut self) -> bool {
        if self.can_become_ready() {
            *self = Self::Ready;
            true
        } else {
            false
        }
    }

    /// Transition to DeviceLost. Returns false if not allowed.
    pub fn transition_to_device_lost(&mut self) -> bool {
        if matches!(self, Self::Ready) {
            *self = Self::DeviceLost;
            true
        } else {
            false
        }
    }

    /// Transition to Closed. Returns false if not allowed.
    pub fn transition_to_closed(&mut self) -> bool {
        if !self.is_terminal() {
            *self = Self::Closed;
            true
        } else {
            false
        }
    }
}
