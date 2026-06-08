//! Runtime spawn logic.
//!
//! Pure function that creates a RuntimeHandle from a SpawnRequest.

use crate::playback::output_wasapi::output_thread::runtime::spawn::spawn_request::SpawnRequest;
use crate::playback::output_wasapi::output_thread::runtime::thread_runtime::runtime_handle::RuntimeHandle;
use crate::playback::output_wasapi::output_thread::state::thread_lifecycle::OutputThreadLifecycle;

/// Create a RuntimeHandle from a SpawnRequest.
///
/// If `auto_start` is true, lifecycle is set to Starting.
/// Otherwise, lifecycle is set to NotStarted.
pub fn create_runtime_handle(request: &SpawnRequest) -> RuntimeHandle {
    let lifecycle = if request.auto_start {
        OutputThreadLifecycle::Starting
    } else {
        OutputThreadLifecycle::NotStarted
    };

    RuntimeHandle::new(request.handle, lifecycle)
}
