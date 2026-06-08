pub mod activity_log;
pub mod activity_snapshot;
pub mod commands;
pub mod event;
pub mod recorder;

// Root-level aliases preserving old module paths for callers
pub use self::activity_log as lifecycle_activity_log;
pub use self::activity_snapshot as lifecycle_activity_snapshot;
pub use self::commands as lifecycle_commands;
pub use self::event as lifecycle_event;
pub use self::recorder as lifecycle_recorder;

// Re-export parent dependencies so moved files' super:: references still resolve
use super::backends::backend_types::PlaybackBackendDescriptor;
use super::backends::mpv;
use super::clock;
use super::errors::PlaybackResult;

#[derive(Clone, Debug)]
pub struct PlaybackLifecycle {
    pub backend: PlaybackBackendDescriptor,
}

impl PlaybackLifecycle {
    pub fn initialize() -> Self {
        Self {
            backend: mpv::descriptor(),
        }
    }

    pub fn shutdown(self) -> PlaybackResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests;
