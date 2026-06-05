//! WASAPI context for real device open/close boundary.
//!
//! This module provides a RAII context that manages the lifecycle of
//! Windows WASAPI resources: COM apartment, device enumerator, endpoint,
//! IAudioClient, and IAudioRenderClient.
//!
//! **This module does NOT:**
//! - Start or stop IAudioClient
//! - Write PCM data
//! - Produce audible output
//! - Create output threads
//! - Use RingBuffer

mod context;
mod format_cache;
mod padding_state;
mod render_error;
#[cfg(target_os = "windows")]
mod render_write;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(test)]
mod padding_state_tests;
#[cfg(test)]
mod render_write_tests;
#[cfg(test)]
mod tests;

pub(crate) use context::WasapiContext;
#[allow(unused_imports)]
pub(crate) use format_cache::WasapiFormatCache;
#[allow(unused_imports)]
pub(crate) use padding_state::{
    calculate_available_frames, WasapiPaddingStateError, WasapiPaddingStateSnapshot,
};
#[allow(unused_imports)]
pub(crate) use render_error::{WasapiRenderWriteError, WasapiRenderWriteReport};
