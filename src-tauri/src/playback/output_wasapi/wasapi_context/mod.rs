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
#[cfg(target_os = "windows")]
mod windows;

#[cfg(test)]
mod tests;

pub(crate) use context::WasapiContext;
