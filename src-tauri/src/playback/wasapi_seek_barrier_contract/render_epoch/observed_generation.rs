use super::super::generation::{CommandGeneration, FrameGeneration};

/// Observed generation in a render epoch mismatch.
///
/// Distinguishes whether the mismatch involves a frame or a command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderEpochObservedGeneration {
    /// A frame whose generation does not match the active epoch.
    Frame(FrameGeneration),
    /// A command whose generation does not match the active epoch.
    Command(CommandGeneration),
}

impl RenderEpochObservedGeneration {
    /// Whether this observed generation is a frame.
    pub fn is_frame(self) -> bool {
        matches!(self, Self::Frame(_))
    }

    /// Whether this observed generation is a command.
    pub fn is_command(self) -> bool {
        matches!(self, Self::Command(_))
    }
}
