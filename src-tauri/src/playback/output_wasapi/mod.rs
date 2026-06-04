pub mod buffer;
pub mod client;
pub mod config;
pub mod device;
pub mod errors;
pub mod format;
pub mod frame_bridge;
pub mod initialize;
pub mod output_thread_boundary;
pub(crate) mod output_thread_buffer_snapshot;
pub(crate) mod output_thread_consumer_plan;
pub(crate) mod output_thread_control;
pub(crate) mod output_thread_errors;
pub(crate) mod output_thread_mock_buffer;
pub(crate) mod output_thread_mock_harness;
pub(crate) mod output_thread_mock_renderer;
pub(crate) mod output_thread_mock_result;
pub(crate) mod output_thread_mock_scenario;
pub(crate) mod output_thread_mock_scenario_matrix;
pub(crate) mod output_thread_mock_scenario_result;
pub(crate) mod output_thread_mock_scenario_runner;
pub(crate) mod output_thread_mock_scenario_summary;
pub(crate) mod output_thread_mock_scenarios;
pub(crate) mod output_thread_mock_sequence;
pub(crate) mod output_thread_plan_invariants;
pub(crate) mod output_thread_plan_projection;
pub(crate) mod output_thread_plan_validation;
pub(crate) mod output_thread_render_plan;
pub(crate) mod output_thread_state;
pub(crate) mod output_thread_transition_validation;
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
mod output_thread_state_tests;

#[cfg(test)]
mod output_thread_control_tests;

#[cfg(test)]
mod output_thread_errors_tests;

#[cfg(test)]
mod output_thread_buffer_snapshot_tests;

#[cfg(test)]
mod output_thread_render_plan_tests;

#[cfg(test)]
mod output_thread_consumer_plan_tests;

#[cfg(test)]
mod output_thread_plan_validation_tests;

#[cfg(test)]
mod output_thread_plan_projection_tests;

#[cfg(test)]
mod output_thread_plan_invariants_tests;

#[cfg(test)]
mod output_thread_transition_validation_tests;

#[cfg(test)]
mod output_thread_mock_buffer_tests;

#[cfg(test)]
mod output_thread_mock_renderer_tests;

#[cfg(test)]
mod output_thread_mock_result_tests;

#[cfg(test)]
mod output_thread_mock_harness_tests;

#[cfg(test)]
mod output_thread_mock_sequence_tests;

#[cfg(test)]
mod output_thread_mock_scenario_tests;

#[cfg(test)]
mod output_thread_mock_scenario_summary_tests;

#[cfg(test)]
mod output_thread_mock_scenario_result_tests;

#[cfg(test)]
mod output_thread_mock_scenario_runner_tests;

#[cfg(test)]
mod output_thread_mock_scenarios_tests;

#[cfg(test)]
mod output_thread_mock_scenario_matrix_tests;

#[cfg(test)]
mod frame_bridge_tests;
