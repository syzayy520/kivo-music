//! Audio command facade.
//!
//! This file remains as `audio/commands.rs` to keep external imports stable,
//! but the implementation is split into smaller single-responsibility modules.

#[path = "commands/devices.rs"]
mod devices;
#[path = "commands/files.rs"]
mod files;
#[path = "commands/playback.rs"]
mod playback;
#[path = "commands/queue.rs"]
mod queue;
#[path = "commands/tick.rs"]
mod tick;
#[path = "commands/types.rs"]
mod types;

pub use devices::*;
pub use files::*;
pub use playback::*;
pub use queue::*;
pub use tick::*;
pub use types::*;
