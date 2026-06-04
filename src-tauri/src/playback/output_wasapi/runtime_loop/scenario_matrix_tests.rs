use super::output_thread_runtime_loop_scenario::OutputThreadRuntimeLoopScenarioKind;
use super::output_thread_runtime_loop_scenario_matrix::runtime_loop_scenario_matrix;
use super::output_thread_runtime_loop_scenario_runner::plan_runtime_loop_scenario;

#[test]
fn matrix_contains_required_scenarios() {
    let matrix = runtime_loop_scenario_matrix();
    let kinds: Vec<_> = matrix.iter().map(|s| s.kind).collect();
    assert!(kinds.contains(&OutputThreadRuntimeLoopScenarioKind::AcceptedStart));
    assert!(kinds.contains(&OutputThreadRuntimeLoopScenarioKind::RejectedStart));
    assert!(kinds.contains(&OutputThreadRuntimeLoopScenarioKind::RenderAudioPlan));
    assert!(kinds.contains(&OutputThreadRuntimeLoopScenarioKind::RenderSilencePlan));
    assert!(kinds.contains(&OutputThreadRuntimeLoopScenarioKind::AcceptedStop));
    assert!(kinds.contains(&OutputThreadRuntimeLoopScenarioKind::AcceptedClose));
    assert!(kinds.contains(&OutputThreadRuntimeLoopScenarioKind::RenderExitPlan));
    assert!(kinds.contains(&OutputThreadRuntimeLoopScenarioKind::RejectedIntentIgnored));
    assert_eq!(matrix.len(), 8);
}

#[test]
fn matrix_scenarios_have_names() {
    let matrix = runtime_loop_scenario_matrix();
    for scenario in &matrix {
        assert!(!scenario.name.is_empty());
    }
}

#[test]
fn matrix_scenarios_have_bounded_steps() {
    let matrix = runtime_loop_scenario_matrix();
    for scenario in &matrix {
        assert!(scenario.max_steps > 0);
        assert!(scenario.max_steps <= 10);
    }
}

#[test]
fn matrix_runs_all_scenarios() {
    let matrix = runtime_loop_scenario_matrix();
    for scenario in &matrix {
        let run = plan_runtime_loop_scenario(scenario);
        assert!(run.has_decisions());
        assert_eq!(run.name, scenario.name);
    }
}

#[test]
fn matrix_render_audio_scenario_is_classification_only() {
    let matrix = runtime_loop_scenario_matrix();
    let audio = matrix
        .iter()
        .find(|s| s.kind == OutputThreadRuntimeLoopScenarioKind::RenderAudioPlan)
        .unwrap();
    let run = plan_runtime_loop_scenario(audio);
    assert!(!run.is_terminal());
    assert_eq!(
        run.final_state,
        super::output_thread_runtime_loop_state::OutputThreadRuntimeLoopState::Active
    );
}

#[test]
fn matrix_does_not_require_runtime_resources() {
    let matrix = runtime_loop_scenario_matrix();
    assert_eq!(matrix.len(), 8);
    for scenario in &matrix {
        assert!(scenario.is_pure_scenario());
    }
}
