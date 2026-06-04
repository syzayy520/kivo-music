pub mod buffer;
pub mod client;
pub mod config;
pub mod device;
pub mod errors;
pub mod format;
pub mod frame_bridge;
pub mod initialize;
pub mod output_thread_boundary;
pub mod padding_query;
pub mod platform;
pub mod render_client;
pub mod reset_boundary;
pub mod ring_buffer;
pub mod ring_buffer_output_thread;
pub mod silent_loop;
pub mod sink;
pub(crate) mod sink_silent_helper;
pub mod start_stop;
pub mod status;

#[cfg(test)]
mod buffer_tests;

#[cfg(test)]
mod client_tests;

#[cfg(test)]
mod device_tests;

#[cfg(test)]
mod format_tests;

#[cfg(test)]
mod initialize_tests;

#[cfg(test)]
mod padding_query_tests;

#[cfg(test)]
mod render_client_tests;

#[cfg(test)]
mod reset_boundary_tests;

#[cfg(test)]
mod silent_loop_tests;

#[cfg(test)]
mod start_stop_tests;

#[cfg(test)]
mod output_thread_boundary_tests;

#[cfg(test)]
mod ring_buffer_output_thread_tests;

#[cfg(test)]
mod ring_buffer_tests;

#[cfg(test)]
mod sink_tests;

#[cfg(test)]
mod sink_lifecycle_tests;

#[cfg(test)]
mod sink_ring_buffer_tests;

#[cfg(test)]
mod sink_silent_helper_tests;

#[cfg(test)]
mod frame_bridge_tests;
