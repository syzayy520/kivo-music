use super::scenario::OutputThreadMockScenario;
use super::scenarios;

/// Return all named mock scenarios in fixed order.
#[allow(dead_code)]
pub(crate) fn all_mock_scenarios() -> Vec<OutputThreadMockScenario> {
    vec![
        scenarios::normal_audio(),
        scenarios::empty_running(),
        scenarios::no_capacity(),
        scenarios::shutdown_requested(),
        scenarios::paused_empty(),
        scenarios::paused_with_frames(),
        scenarios::flush_empty(),
        scenarios::closed_empty(),
        scenarios::closed_with_remaining(),
        scenarios::non_running(),
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
