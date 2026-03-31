//! WASAPI output backend(s).
//!
//! Current bring-up focuses on **WASAPI Shared** for stability and broad device compatibility.
//! Implementation is Windows-only.

#[cfg(windows)]
pub(crate) mod pack;

#[cfg(windows)]
pub mod devices;

#[cfg(windows)]
mod stats;

#[cfg(windows)]
pub mod wasapi_shared;

#[cfg(windows)]
mod wasapi_shared_consumer;

#[cfg(windows)]
mod wasapi_shared_consumer_diag;

#[cfg(windows)]
mod wasapi_shared_producer;

#[cfg(windows)]
mod wasapi_shared_summary;

#[cfg(windows)]
pub(crate) use stats::WasapiSoakStats;

#[cfg(windows)]
pub use devices::{list_render_devices, try_list_render_devices, RenderDeviceInfo};
