use std::cell::Cell;

use super::super::render_ring_buffer_boundary::{
    run_render_ring_buffer_boundary_with_callbacks, RenderRingBufferBoundaryError,
};
use super::fixtures::{cfg, fmt, report, snap};
use crate::playback::output_wasapi::ring_buffer_render_boundary::RingBufferRenderBoundaryError;
use crate::playback::output_wasapi::wasapi_context::WasapiRenderWriteError;

#[test]
fn synthetic_source_seed_zero_is_empty_open() {
    let writes = Cell::new(0);
    let outcome = run_render_ring_buffer_boundary_with_callbacks(
        fmt(),
        cfg(true, 1, 2, 0),
        || Ok(snap(2)),
        |_, _| {
            writes.set(writes.get() + 1);
            Ok(report(1, 8))
        },
    )
    .expect("run");
    assert_eq!(outcome.iterations_source_empty_open, 1);
    assert_eq!(writes.get(), 0);
}

#[test]
fn synthetic_source_seeded_writes_all_zero_bytes() {
    let outcome = run_render_ring_buffer_boundary_with_callbacks(
        fmt(),
        cfg(true, 1, 4, 2),
        || Ok(snap(4)),
        |frames, bytes| {
            assert!(bytes.iter().all(|byte| *byte == 0));
            Ok(report(frames, bytes.len() as u32))
        },
    )
    .expect("run");
    assert_eq!(outcome.frames_written_total, 2);
    assert_eq!(outcome.frames_written_total, outcome.frames_committed_total);
}

#[test]
fn synthetic_source_caps_by_padding_and_config() {
    let seen_padding = Cell::new(0);
    run_render_ring_buffer_boundary_with_callbacks(
        fmt(),
        cfg(true, 1, 5, 5),
        || Ok(snap(2)),
        |frames, bytes| {
            seen_padding.set(frames);
            Ok(report(frames, bytes.len() as u32))
        },
    )
    .expect("run");
    assert_eq!(seen_padding.get(), 2);

    let seen_config = Cell::new(0);
    run_render_ring_buffer_boundary_with_callbacks(
        fmt(),
        cfg(true, 1, 3, 5),
        || Ok(snap(5)),
        |frames, bytes| {
            seen_config.set(frames);
            Ok(report(frames, bytes.len() as u32))
        },
    )
    .expect("run");
    assert_eq!(seen_config.get(), 3);
}

#[test]
fn writer_failure_maps_to_boundary_failed() {
    let result = run_render_ring_buffer_boundary_with_callbacks(
        fmt(),
        cfg(true, 1, 2, 2),
        || Ok(snap(2)),
        |_, _| {
            Err(RingBufferRenderBoundaryError::WasapiWrite(
                WasapiRenderWriteError::NotOpen,
            ))
        },
    );
    assert!(matches!(
        result,
        Err(RenderRingBufferBoundaryError::BoundaryFailed(_))
    ));
}
