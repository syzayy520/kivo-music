use std::cell::Cell;

use super::pcm_adapter::{PcmRenderFormat, PcmSampleFormat};
use super::ring_buffer::{RingBuffer, RingBufferFormat};
use super::ring_buffer_render_boundary::{
    pcm_render_format_from_cache_for_boundary, run_padding_aware_ring_buffer_render_boundary,
    run_padding_aware_ring_buffer_render_boundary_with_callbacks, validate_boundary_write_report,
    RingBufferRenderBoundaryConfig, RingBufferRenderBoundaryError,
    MAX_RING_BUFFER_RENDER_BOUNDARY_FRAMES_PER_WRITE, MAX_RING_BUFFER_RENDER_BOUNDARY_ITERATIONS,
};
use super::wasapi_context::{
    WasapiContext, WasapiFormatCache, WasapiPaddingStateSnapshot, WasapiRenderWriteError,
    WasapiRenderWriteReport,
};

fn fmt() -> PcmRenderFormat {
    PcmRenderFormat {
        sample_rate_hz: 44_100,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
        sample_format: PcmSampleFormat::Float32Interleaved,
    }
}
fn cache() -> WasapiFormatCache {
    WasapiFormatCache {
        sample_rate_hz: 44_100,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
        avg_bytes_per_sec: 352_800,
        format_tag: 3,
        cb_size: 0,
    }
}
fn rb(capacity: u32) -> RingBuffer {
    RingBuffer::new(
        RingBufferFormat {
            sample_rate_hz: 44_100,
            channels: 2,
            bits_per_sample: 32,
            block_align: 8,
        },
        capacity,
    )
    .expect("buffer")
}
fn data(frames: u32) -> Vec<u8> {
    vec![0; frames as usize * 8]
}
fn snap(available: u32) -> WasapiPaddingStateSnapshot {
    WasapiPaddingStateSnapshot {
        buffer_frame_capacity: 8,
        current_padding_frames: 8 - available,
        available_frames: available,
    }
}
fn report(frames: u32, bytes: u32) -> WasapiRenderWriteReport {
    WasapiRenderWriteReport {
        frames_written: frames,
        bytes_written: bytes,
        used_silent_flag: false,
        sample_rate_hz: 44_100,
        channels: 2,
    }
}
fn cfg(iterations: u32, max_frames_per_write: u32) -> RingBufferRenderBoundaryConfig {
    RingBufferRenderBoundaryConfig {
        iterations,
        max_frames_per_write,
    }
}
fn run_ok(buffer: &mut RingBuffer, available: u32, max: u32) -> (u32, u32) {
    let seen = Cell::new(0);
    let out = run_padding_aware_ring_buffer_render_boundary_with_callbacks(
        buffer,
        fmt(),
        cfg(1, max),
        || Ok(snap(available)),
        |frames, bytes| {
            seen.set(frames);
            Ok(report(frames, bytes.len() as u32))
        },
    )
    .expect("run");
    (seen.get(), out.frames_committed_total)
}

#[test]
fn boundary_rejects_invalid_config() {
    let mut buffer = rb(2);
    let cases = [
        cfg(0, 1),
        cfg(MAX_RING_BUFFER_RENDER_BOUNDARY_ITERATIONS + 1, 1),
        cfg(1, 0),
        cfg(1, MAX_RING_BUFFER_RENDER_BOUNDARY_FRAMES_PER_WRITE + 1),
    ];
    for case in cases {
        assert!(
            run_padding_aware_ring_buffer_render_boundary_with_callbacks(
                &mut buffer,
                fmt(),
                case,
                || Ok(snap(1)),
                |_, _| Ok(report(1, 8))
            )
            .is_err()
        );
    }
}

#[test]
fn open_empty_closed_and_padding_empty_paths_do_not_write() {
    let writes = Cell::new(0);
    let mut has_data = rb(2);
    has_data.write_frames(&data(1)).expect("write");
    let skipped = run_padding_aware_ring_buffer_render_boundary_with_callbacks(
        &mut has_data,
        fmt(),
        cfg(1, 2),
        || Ok(snap(0)),
        |_, _| {
            writes.set(writes.get() + 1);
            Ok(report(1, 8))
        },
    )
    .expect("run");
    assert_eq!(skipped.iterations_skipped_no_available, 1);
    assert_eq!(has_data.available_frames(), 1);

    let empty = run_padding_aware_ring_buffer_render_boundary_with_callbacks(
        &mut rb(2),
        fmt(),
        cfg(1, 2),
        || Ok(snap(2)),
        |_, _| {
            writes.set(writes.get() + 1);
            Ok(report(1, 8))
        },
    )
    .expect("run");
    assert_eq!(empty.iterations_source_empty_open, 1);

    let mut closed = rb(2);
    closed.close();
    let done = run_padding_aware_ring_buffer_render_boundary_with_callbacks(
        &mut closed,
        fmt(),
        cfg(1, 2),
        || Ok(snap(2)),
        |_, _| Ok(report(1, 8)),
    )
    .expect("run");
    assert!(done.source_closed && done.completed);
    assert_eq!(writes.get(), 0);
}

#[test]
fn success_and_caps_commit_only_written_frames() {
    let mut success = rb(4);
    success.write_frames(&data(2)).expect("write");
    let (seen, committed) = run_ok(&mut success, 4, 4);
    assert_eq!((seen, committed, success.available_frames()), (2, 2, 0));

    let mut partial = rb(4);
    partial.write_frames(&data(1)).expect("write");
    assert_eq!(run_ok(&mut partial, 4, 4), (1, 1));

    let mut padding_cap = rb(8);
    padding_cap.write_frames(&data(5)).expect("write");
    assert_eq!(run_ok(&mut padding_cap, 2, 5).0, 2);

    let mut config_cap = rb(8);
    config_cap.write_frames(&data(5)).expect("write");
    assert_eq!(run_ok(&mut config_cap, 5, 3).0, 3);
}

#[test]
fn write_and_report_failures_keep_buffer_available() {
    let mut write_fail = rb(4);
    write_fail.write_frames(&data(2)).expect("write");
    assert!(matches!(
        run_padding_aware_ring_buffer_render_boundary_with_callbacks(
            &mut write_fail,
            fmt(),
            cfg(1, 2),
            || Ok(snap(2)),
            |_, _| Err(RingBufferRenderBoundaryError::WasapiWrite(
                WasapiRenderWriteError::NotOpen
            ))
        ),
        Err(RingBufferRenderBoundaryError::WasapiWrite(_))
    ));
    assert_eq!(write_fail.available_frames(), 2);

    let cases = [
        report(1, 16),
        report(2, 8),
        WasapiRenderWriteReport {
            used_silent_flag: true,
            ..report(2, 16)
        },
    ];
    for fake in cases {
        let mut buffer = rb(4);
        buffer.write_frames(&data(2)).expect("write");
        assert!(
            run_padding_aware_ring_buffer_render_boundary_with_callbacks(
                &mut buffer,
                fmt(),
                cfg(1, 2),
                || Ok(snap(2)),
                |_, _| Ok(fake.clone())
            )
            .is_err()
        );
        assert_eq!(buffer.available_frames(), 2);
    }
}

#[test]
fn multiple_iterations_are_bounded() {
    let mut buffer = rb(8);
    buffer.write_frames(&data(4)).expect("write");
    let calls = Cell::new(0);
    let out = run_padding_aware_ring_buffer_render_boundary_with_callbacks(
        &mut buffer,
        fmt(),
        cfg(3, 1),
        || Ok(snap(8)),
        |frames, bytes| {
            calls.set(calls.get() + 1);
            Ok(report(frames, bytes.len() as u32))
        },
    )
    .expect("run");
    assert_eq!(
        (
            calls.get(),
            out.iterations_completed,
            buffer.available_frames()
        ),
        (3, 3, 1)
    );
}

#[test]
fn production_wrapper_and_format_boundaries_are_explicit() {
    let mut buffer = rb(2);
    assert_eq!(
        run_padding_aware_ring_buffer_render_boundary(
            &WasapiContext::new(),
            &mut buffer,
            cfg(1, 1)
        ),
        Err(RingBufferRenderBoundaryError::MissingFormatCache)
    );
    assert_eq!(
        pcm_render_format_from_cache_for_boundary(&cache()),
        Ok(fmt())
    );
    let mut bad = cache();
    bad.format_tag = 1;
    assert_eq!(
        pcm_render_format_from_cache_for_boundary(&bad),
        Err(RingBufferRenderBoundaryError::UnsupportedFormat)
    );
}

#[test]
fn write_report_validator_returns_specific_errors() {
    assert_eq!(
        validate_boundary_write_report(&report(1, 16), 2, 16).unwrap_err(),
        RingBufferRenderBoundaryError::WrittenFrameCountMismatch {
            expected: 2,
            actual: 1
        }
    );
    assert_eq!(
        validate_boundary_write_report(&report(2, 8), 2, 16).unwrap_err(),
        RingBufferRenderBoundaryError::WrittenByteCountMismatch {
            expected: 16,
            actual: 8
        }
    );
}

#[test]
#[cfg(target_os = "windows")]
#[ignore]
fn windows_ignored_boundary_smoke() {
    let mut context = WasapiContext::new();
    if context.open().is_err() {
        return;
    }
    let mut buffer = rb(2);
    buffer.write_frames(&data(1)).expect("write");
    let result = run_padding_aware_ring_buffer_render_boundary(&context, &mut buffer, cfg(1, 1));
    context.close();
    if let Ok(outcome) = result {
        assert!(outcome.iterations_completed <= 1);
    }
}
