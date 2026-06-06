use crate::playback::audio_route_pipeline_tap::AudioRoutePipelineTapReport;
use crate::playback::native_pipeline::NativePipeline;
use crate::playback::native_pipeline_route_tap_diagnostic_policy::NativePipelineRouteTapDiagnosticPolicy;

use super::fixtures::{opened, output_frame};

type CloseAction = fn(
    &mut NativePipelineRouteTapDiagnosticPolicy,
    &mut NativePipeline,
) -> Option<AudioRoutePipelineTapReport>;

fn assert_close_action(action: CloseAction) -> Result<(), String> {
    let (mut policy, mut pipeline) = opened(2)?;
    pipeline.tap_audio_route_output_frame_non_fatal(&output_frame(vec![0.0, 0.1]));

    let report =
        action(&mut policy, &mut pipeline).ok_or_else(|| "close report missing".to_string())?;

    assert!(report.closed);
    assert_eq!(report.input_count, 1);
    assert!(pipeline.audio_route_pipeline_tap_report().is_none());
    assert_eq!(policy.last_detached_report(), Some(report));
    assert!(policy.current_stream().is_none());
    Ok(())
}

#[test]
fn stop_closes_detaches_and_retains_report() -> Result<(), String> {
    assert_close_action(NativePipelineRouteTapDiagnosticPolicy::close_detach_on_stop)
}

#[test]
fn close_decoder_closes_detaches_and_retains_report() -> Result<(), String> {
    assert_close_action(NativePipelineRouteTapDiagnosticPolicy::close_detach_on_close_decoder)
}

#[test]
fn shutdown_closes_detaches_and_retains_report() -> Result<(), String> {
    assert_close_action(NativePipelineRouteTapDiagnosticPolicy::close_detach_on_shutdown)
}

#[test]
fn close_actions_are_no_ops_without_tap() -> Result<(), String> {
    let actions: [CloseAction; 3] = [
        NativePipelineRouteTapDiagnosticPolicy::close_detach_on_stop,
        NativePipelineRouteTapDiagnosticPolicy::close_detach_on_close_decoder,
        NativePipelineRouteTapDiagnosticPolicy::close_detach_on_shutdown,
    ];

    for action in actions {
        let mut policy = super::fixtures::policy(2)?;
        let mut pipeline = NativePipeline::new();
        assert!(action(&mut policy, &mut pipeline).is_none());
        assert!(policy.last_detached_report().is_none());
    }
    Ok(())
}
