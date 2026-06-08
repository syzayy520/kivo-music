use super::super::generation::RenderEpoch;
use super::observed_generation::RenderEpochObservedGeneration;

/// Describes a mismatch between the active RenderEpoch and an observed generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderEpochMismatch {
    /// The active render epoch.
    pub active: RenderEpoch,
    /// The observed generation (frame or command).
    pub observed: RenderEpochObservedGeneration,
}

impl RenderEpochMismatch {
    /// Whether this mismatch involves a frame.
    pub fn is_frame(self) -> bool {
        self.observed.is_frame()
    }

    /// Whether this mismatch involves a command.
    pub fn is_command(self) -> bool {
        self.observed.is_command()
    }
}
