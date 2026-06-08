//! Sink dispatch event routing tests.
//!
//! Tests for map_dispatch_outcome_to_event.

use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::event_routing::map_dispatch_outcome_to_event;
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::DispatchOutcome;

// ── map_dispatch_outcome_to_event tests ────────────────────────────────

#[test]
fn event_routing_success_returns_none() {
    let outcome = DispatchOutcome::Success {
        frames_processed: 100,
        bytes_written: 400,
    };
    assert!(map_dispatch_outcome_to_event(&outcome).is_none());
}

#[test]
fn event_routing_silence_filled_returns_none() {
    let outcome = DispatchOutcome::SilenceFilled { frames_written: 50 };
    assert!(map_dispatch_outcome_to_event(&outcome).is_none());
}

#[test]
fn event_routing_skipped_returns_none() {
    assert!(map_dispatch_outcome_to_event(&DispatchOutcome::Skipped).is_none());
}

#[test]
fn event_routing_noop_returns_none() {
    assert!(map_dispatch_outcome_to_event(&DispatchOutcome::Noop).is_none());
}

#[test]
fn event_routing_failed_returns_none() {
    assert!(map_dispatch_outcome_to_event(&DispatchOutcome::Failed).is_none());
}

#[test]
fn event_routing_all_variants_return_none() {
    let variants = vec![
        DispatchOutcome::Success {
            frames_processed: 0,
            bytes_written: 0,
        },
        DispatchOutcome::SilenceFilled { frames_written: 0 },
        DispatchOutcome::Skipped,
        DispatchOutcome::Noop,
        DispatchOutcome::Failed,
    ];
    for v in variants {
        assert!(
            map_dispatch_outcome_to_event(&v).is_none(),
            "expected None for {:?}",
            v
        );
    }
}
