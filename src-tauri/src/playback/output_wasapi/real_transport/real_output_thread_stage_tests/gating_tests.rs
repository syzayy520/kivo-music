use std::cell::RefCell;

use super::super::thread_error::RealOutputThreadSkeletonError;
use super::super::thread_stages::run_post_start_stages_with;
use super::fixtures::{boundary_ok, padding_loop_ok, render_loop_ok, stage_configs};

#[test]
fn prior_render_loop_failure_skips_padding_and_boundary() {
    let calls = RefCell::new(Vec::new());
    let results = run_post_start_stages_with(
        stage_configs(true, 2, 3, 4),
        || {
            calls.borrow_mut().push("render_loop");
            Err(RealOutputThreadSkeletonError::RenderSilenceLoopFailed(
                "loop".to_string(),
            ))
        },
        || {
            calls.borrow_mut().push("padding_loop");
            Ok(padding_loop_ok())
        },
        || {
            calls.borrow_mut().push("ring_buffer_boundary");
            Ok(boundary_ok())
        },
    );

    assert_eq!(*calls.borrow(), vec!["render_loop"]);
    assert!(
        results
            .render_ring_buffer_boundary_result
            .as_ref()
            .unwrap()
            .skipped_after_prior_failure
    );
    assert!(!results.can_run_worker_loop());
}

#[test]
fn padding_failure_skips_boundary_and_prevents_worker_loop() {
    let calls = RefCell::new(Vec::new());
    let results = run_post_start_stages_with(
        stage_configs(true, 2, 3, 4),
        || {
            calls.borrow_mut().push("render_loop");
            Ok(render_loop_ok())
        },
        || {
            calls.borrow_mut().push("padding_loop");
            Err(
                RealOutputThreadSkeletonError::RenderPaddingLoopPaddingStateFailed(
                    "padding".to_string(),
                ),
            )
        },
        || {
            calls.borrow_mut().push("ring_buffer_boundary");
            Ok(boundary_ok())
        },
    );

    assert_eq!(*calls.borrow(), vec!["render_loop", "padding_loop"]);
    assert!(
        results
            .render_ring_buffer_boundary_result
            .as_ref()
            .unwrap()
            .skipped_after_prior_failure
    );
    assert!(!results.can_run_worker_loop());
}

#[test]
fn boundary_failure_prevents_worker_loop_after_ordered_stage_calls() {
    let calls = RefCell::new(Vec::new());
    let results = run_post_start_stages_with(
        stage_configs(true, 1, 1, 1),
        || {
            calls.borrow_mut().push("render_loop");
            Ok(render_loop_ok())
        },
        || {
            calls.borrow_mut().push("padding_loop");
            Ok(padding_loop_ok())
        },
        || {
            calls.borrow_mut().push("ring_buffer_boundary");
            Err(
                RealOutputThreadSkeletonError::RenderRingBufferBoundaryFailed(
                    "boundary".to_string(),
                ),
            )
        },
    );

    assert_eq!(
        *calls.borrow(),
        vec!["render_loop", "padding_loop", "ring_buffer_boundary"]
    );
    assert!(!results.can_run_worker_loop());
}
