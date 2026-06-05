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

include!("output_thread_runtime_test_modules.rs");








