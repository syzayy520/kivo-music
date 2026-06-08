pub mod buffer;
#[cfg(test)]
mod buffer_tests;
pub mod client;
pub mod config;
pub mod device;
pub mod errors;
pub mod format;
pub mod frame_bridge;
pub mod initialize;
pub mod output_thread_boundary;
pub mod output_thread_state;

include!("output_thread_core_modules.rs");

include!("output_thread_mock_modules.rs");

include!("output_thread_runtime_modules.rs");

pub mod padding_query;
pub(crate) mod pcm_adapter;
#[cfg(test)]
mod pcm_adapter_tests;
pub mod platform;
pub mod render_client;
pub mod reset_boundary;
pub mod ring_buffer;
pub mod ring_buffer_output_thread;
pub(crate) mod ring_buffer_render_boundary;
#[cfg(test)]
mod ring_buffer_render_boundary_tests;
pub(crate) mod ring_buffer_source;
#[cfg(test)]
mod ring_buffer_source_tests;
pub mod silent_loop;
pub mod sink;
pub(crate) mod sink_drain;
pub(crate) mod sink_silent_helper;
pub(crate) mod sink_submission;
pub mod start_stop;
pub mod status;
pub(crate) mod wasapi_context;
pub(crate) mod zero_pcm_render;
#[cfg(test)]
mod zero_pcm_render_tests;

#[cfg(test)]
mod output_thread_state_tests;

include!("output_thread_test_modules.rs");
