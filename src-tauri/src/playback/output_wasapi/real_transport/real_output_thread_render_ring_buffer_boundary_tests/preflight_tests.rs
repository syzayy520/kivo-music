use super::super::render_ring_buffer_boundary::{
    run_render_ring_buffer_boundary, RenderRingBufferBoundaryError, RenderRingBufferBoundaryOutcome,
};
use super::fixtures::{assert_err, cfg};
use crate::playback::output_wasapi::wasapi_context::WasapiContext;

#[test]
fn disabled_outcome_does_not_require_context_or_started() {
    let config = cfg(false, 0, 0, 0);
    assert_eq!(
        run_render_ring_buffer_boundary(None, false, config).expect("run"),
        RenderRingBufferBoundaryOutcome::disabled(config)
    );
}

#[test]
fn enabled_preflight_order_is_explicit() {
    assert_err(
        run_render_ring_buffer_boundary(None, false, cfg(true, 1, 1, 0)),
        RenderRingBufferBoundaryError::RequiresContext,
    );
    assert_err(
        run_render_ring_buffer_boundary(Some(&WasapiContext::new()), false, cfg(true, 1, 1, 0)),
        RenderRingBufferBoundaryError::RequiresStartedClient,
    );
    assert_err(
        run_render_ring_buffer_boundary(Some(&WasapiContext::new()), true, cfg(true, 1, 1, 0)),
        RenderRingBufferBoundaryError::MissingFormatCache,
    );
}

#[test]
fn skipped_after_prior_failure_outcome_fields() {
    let outcome = RenderRingBufferBoundaryOutcome::skipped_after_prior_failure(cfg(true, 2, 3, 4));
    assert!(outcome.requested);
    assert!(outcome.skipped_after_prior_failure);
    assert!(!outcome.started);
    assert!(!outcome.completed);
}
