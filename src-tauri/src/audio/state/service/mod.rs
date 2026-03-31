#[cfg(test)]
mod control_tests;
mod lock_poison;
mod player_thread;
mod queue;
mod queue_state;
#[cfg(test)]
mod queue_state_tests;
#[cfg(test)]
mod queue_tests;
#[allow(clippy::module_inception)]
mod service;
#[cfg(test)]
mod service_tests;
#[cfg(windows)]
mod session_buffers;
mod session_cleanup;
#[cfg(windows)]
mod session_loop;
#[cfg(windows)]
mod session_loop_helpers;
#[cfg(windows)]
mod session_loop_runtime;
#[cfg(windows)]
mod session_output_runtime;
#[cfg(test)]
mod test_support;
mod tick;
#[cfg(test)]
mod tick_tests;
mod types;
mod volume;
#[cfg(windows)]
mod windows_device;

pub use service::AudioService;
pub use types::{AudioSessionState, AudioSessionStatus, PlayStartOptions, QueueSnapshot};
