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
mod output_thread_mock_assertions_tests;

#[cfg(test)]
mod output_thread_mock_coverage_tests;

#[cfg(test)]
mod output_thread_mock_expected_behavior_tests;

#[cfg(test)]
mod output_thread_mock_expectation_tests;

#[cfg(test)]
mod output_thread_mock_golden_tests;

#[cfg(test)]
mod output_thread_mock_regression_tests;

#[cfg(test)]
mod output_thread_mock_regression_matrix_tests;

#[cfg(test)]
mod output_thread_mock_smoke_tests;

#[cfg(test)]
mod frame_bridge_tests;

include!("output_thread_runtime_test_modules.rs");








