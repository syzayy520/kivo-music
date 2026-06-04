use super::output_thread_mock_scenario::OutputThreadMockScenario;
use super::output_thread_mock_scenarios;

/// Return all named mock scenarios in fixed order.
#[allow(dead_code)]
pub(crate) fn all_mock_scenarios() -> Vec<OutputThreadMockScenario> {
    vec![
        output_thread_mock_scenarios::normal_audio(),
        output_thread_mock_scenarios::empty_running(),
        output_thread_mock_scenarios::no_capacity(),
        output_thread_mock_scenarios::shutdown_requested(),
        output_thread_mock_scenarios::paused_empty(),
        output_thread_mock_scenarios::paused_with_frames(),
        output_thread_mock_scenarios::flush_empty(),
        output_thread_mock_scenarios::closed_empty(),
        output_thread_mock_scenarios::closed_with_remaining(),
        output_thread_mock_scenarios::non_running(),
    ]
}

/// Return all scenario names in fixed order.
#[allow(dead_code)]
pub(crate) fn scenario_names() -> Vec<&'static str> {
    vec![
        "normal_audio",
        "empty_running",
        "no_capacity",
        "shutdown_requested",
        "paused_empty",
        "paused_with_frames",
        "flush_empty",
        "closed_empty",
        "closed_with_remaining",
        "non_running",
    ]
}
