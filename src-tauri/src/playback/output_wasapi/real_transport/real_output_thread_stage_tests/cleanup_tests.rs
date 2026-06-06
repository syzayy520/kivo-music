use super::super::thread_error::RealOutputThreadSkeletonError;
use super::super::thread_stages::{resolve_thread_stage_results, PostStartStageResults};
use super::fixtures::{boundary_ok, padding_loop_ok, render_loop_ok, successful_stage_results};

#[test]
fn stop_failure_has_priority_over_boundary_failure() {
    let results = PostStartStageResults {
        render_loop_result: Ok(render_loop_ok()),
        render_padding_loop_result: Ok(padding_loop_ok()),
        render_ring_buffer_boundary_result: Err(
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryFailed("boundary".to_string()),
        ),
    };

    assert_eq!(
        resolve_thread_stage_results(Err("stop".to_string()), results, None).unwrap_err(),
        RealOutputThreadSkeletonError::AudioClientStopFailed("stop".to_string())
    );
}

#[test]
fn render_loop_failure_has_priority_over_later_stage_failures() {
    let render_loop_error =
        RealOutputThreadSkeletonError::RenderSilenceLoopFailed("loop".to_string());
    let results = PostStartStageResults {
        render_loop_result: Err(render_loop_error.clone()),
        render_padding_loop_result: Err(
            RealOutputThreadSkeletonError::RenderPaddingLoopPaddingStateFailed(
                "padding".to_string(),
            ),
        ),
        render_ring_buffer_boundary_result: Err(
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryFailed("boundary".to_string()),
        ),
    };

    assert_eq!(
        resolve_thread_stage_results(Ok(()), results, None).unwrap_err(),
        render_loop_error
    );
}

#[test]
fn padding_failure_has_priority_over_boundary_failure() {
    let padding_error =
        RealOutputThreadSkeletonError::RenderPaddingLoopPaddingStateFailed("padding".to_string());
    let results = PostStartStageResults {
        render_loop_result: Ok(render_loop_ok()),
        render_padding_loop_result: Err(padding_error.clone()),
        render_ring_buffer_boundary_result: Err(
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryFailed("boundary".to_string()),
        ),
    };

    assert_eq!(
        resolve_thread_stage_results(Ok(()), results, None).unwrap_err(),
        padding_error
    );
}

#[test]
fn boundary_failure_is_not_masked_by_missing_worker_report() {
    let boundary_error =
        RealOutputThreadSkeletonError::RenderRingBufferBoundaryFailed("boundary".to_string());
    let results = PostStartStageResults {
        render_loop_result: Ok(render_loop_ok()),
        render_padding_loop_result: Ok(padding_loop_ok()),
        render_ring_buffer_boundary_result: Err(boundary_error.clone()),
    };

    assert_eq!(
        resolve_thread_stage_results(Ok(()), results, None).unwrap_err(),
        boundary_error
    );
}

#[test]
fn worker_did_not_report_is_checked_only_after_all_stages_succeed() {
    assert_eq!(
        resolve_thread_stage_results(Ok(()), successful_stage_results(), None).unwrap_err(),
        RealOutputThreadSkeletonError::WorkerDidNotReport
    );
}

#[test]
fn source_order_keeps_close_before_result_resolution() {
    let source = include_str!("../thread.rs");
    let close_index = source.find("opened_context.close()").unwrap();
    let resolve_index = source.find("let resolved =").unwrap();
    let stage_index = source.find("let stage_results").unwrap();
    let worker_index = source.find("let worker_loop_result").unwrap();

    assert!(stage_index < worker_index);
    assert!(close_index < resolve_index);
    assert!(source.contains("stage_results.can_run_worker_loop()"));
    assert!(source.contains("let stop_result"));
    assert!(source.contains("let mut wasapi_context_closed"));
    assert_eq!(boundary_ok().frames_written_total, 0);
}
