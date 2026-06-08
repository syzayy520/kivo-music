pub mod bridge;
pub mod bus;
pub mod dispatcher;

// Root-level aliases preserving old module paths for callers
pub use self::bridge as playback_event_bridge;
pub use self::bus as playback_event_bus;
pub use self::dispatcher as playback_event_dispatcher;

// Re-export parent dependencies so moved files' super:: references still resolve
use super::activity_log;
use super::activity_snapshot;
use super::errors;
use super::events;
use super::state;
