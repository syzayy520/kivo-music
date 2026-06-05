use super::scenario::OutputThreadMockScenario;
use super::scenario_result::OutputThreadMockScenarioResult;
use super::sequence::run_mock_sequence;

/// Run a mock scenario and return the result.
///
/// Pure function — no thread, no external state.
#[allow(dead_code)]
pub(crate) fn run_mock_scenario(
    scenario: OutputThreadMockScenario,
) -> OutputThreadMockScenarioResult {
    let sequence = run_mock_sequence(
        scenario.buffer,
        scenario.renderer,
        scenario.control,
        scenario.max_steps,
    );
    OutputThreadMockScenarioResult::new(scenario.name, sequence)
}
