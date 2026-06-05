#![allow(dead_code)]

use super::pcm_adapter::{
    validate_render_bytes, validate_render_format, PcmAdapterError, PcmRenderFormat,
    PcmSampleFormat,
};
use super::ring_buffer::RingBuffer;
use super::ring_buffer_source::{
    commit_ring_buffer_source_frames, peek_ring_buffer_source_chunk, RingBufferSourceConfig,
    RingBufferSourceError, RingBufferSourceStatus,
};
use super::wasapi_context::{
    WasapiContext, WasapiFormatCache, WasapiPaddingStateError, WasapiPaddingStateSnapshot,
    WasapiRenderWriteError, WasapiRenderWriteReport,
};

pub(crate) const MAX_RING_BUFFER_RENDER_BOUNDARY_ITERATIONS: u32 = 3;
pub(crate) const MAX_RING_BUFFER_RENDER_BOUNDARY_FRAMES_PER_WRITE: u32 = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RingBufferRenderBoundaryConfig {
    pub iterations: u32,
    pub max_frames_per_write: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct RingBufferRenderBoundaryOutcome {
    pub started: bool,
    pub completed: bool,
    pub iterations_requested: u32,
    pub iterations_completed: u32,
    pub iterations_skipped_no_available: u32,
    pub iterations_source_empty_open: u32,
    pub source_closed: bool,
    pub max_frames_per_write: u32,
    pub write_attempts: u32,
    pub frames_peeked_total: u32,
    pub frames_written_total: u32,
    pub frames_committed_total: u32,
    pub last_capacity: u32,
    pub last_padding: u32,
    pub last_available: u32,
    pub used_silent_flag: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RingBufferRenderBoundaryError {
    InvalidIterationCount,
    InvalidFrameCount,
    IterationCountTooLarge,
    FrameCountTooLarge,
    MissingFormatCache,
    UnsupportedFormat,
    PcmAdapter(PcmAdapterError),
    PaddingStateFailed(WasapiPaddingStateError),
    Source(RingBufferSourceError),
    WasapiWrite(WasapiRenderWriteError),
    WrittenFrameCountMismatch { expected: u32, actual: u32 },
    WrittenByteCountMismatch { expected: u32, actual: u32 },
    UnexpectedSilentFlag,
    CommitAfterWriteFailed(RingBufferSourceError),
    ByteLengthOverflow,
}

pub(crate) fn run_padding_aware_ring_buffer_render_boundary(
    context: &WasapiContext,
    buffer: &mut RingBuffer,
    config: RingBufferRenderBoundaryConfig,
) -> Result<RingBufferRenderBoundaryOutcome, RingBufferRenderBoundaryError> {
    let cache = context
        .format_cache()
        .ok_or(RingBufferRenderBoundaryError::MissingFormatCache)?;
    let render_format = pcm_render_format_from_cache_for_boundary(cache)?;

    run_padding_aware_ring_buffer_render_boundary_with_callbacks(
        buffer,
        render_format,
        config,
        || {
            context
                .padding_state_snapshot()
                .map_err(RingBufferRenderBoundaryError::PaddingStateFailed)
        },
        |frames, bytes| {
            context
                .write_render_buffer_bytes(frames, bytes)
                .map_err(RingBufferRenderBoundaryError::WasapiWrite)
        },
    )
}

pub(crate) fn run_padding_aware_ring_buffer_render_boundary_with_callbacks<P, W>(
    buffer: &mut RingBuffer,
    render_format: PcmRenderFormat,
    config: RingBufferRenderBoundaryConfig,
    mut padding_snapshot: P,
    mut write_bytes: W,
) -> Result<RingBufferRenderBoundaryOutcome, RingBufferRenderBoundaryError>
where
    P: FnMut() -> Result<WasapiPaddingStateSnapshot, RingBufferRenderBoundaryError>,
    W: FnMut(u32, &[u8]) -> Result<WasapiRenderWriteReport, RingBufferRenderBoundaryError>,
{
    let config = validate_config(config)?;
    let render_format =
        validate_render_format(render_format).map_err(RingBufferRenderBoundaryError::PcmAdapter)?;
    let mut outcome = RingBufferRenderBoundaryOutcome {
        started: true,
        iterations_requested: config.iterations,
        max_frames_per_write: config.max_frames_per_write,
        ..RingBufferRenderBoundaryOutcome::default()
    };

    for _ in 0..config.iterations {
        let snapshot = padding_snapshot()?;
        update_snapshot_fields(&mut outcome, snapshot);
        if snapshot.available_frames == 0 {
            outcome.iterations_skipped_no_available += 1;
            outcome.iterations_completed += 1;
            continue;
        }

        let frames_wanted = snapshot.available_frames.min(config.max_frames_per_write);
        let source_config = RingBufferSourceConfig {
            max_frames: frames_wanted,
            render_format,
        };
        match peek_ring_buffer_source_chunk(buffer, source_config)
            .map_err(RingBufferRenderBoundaryError::Source)?
        {
            RingBufferSourceStatus::EmptyOpen => {
                outcome.iterations_source_empty_open += 1;
                outcome.iterations_completed += 1;
            }
            RingBufferSourceStatus::EmptyClosed => {
                outcome.source_closed = true;
                outcome.iterations_completed += 1;
                outcome.completed = true;
                break;
            }
            RingBufferSourceStatus::Chunk(chunk) => {
                let expected_bytes = checked_u32_len(chunk.bytes.len())?;
                validate_render_bytes(render_format, chunk.frames, &chunk.bytes)
                    .map_err(RingBufferRenderBoundaryError::PcmAdapter)?;
                let report = write_bytes(chunk.frames, &chunk.bytes)?;
                validate_boundary_write_report(&report, chunk.frames, expected_bytes)?;
                let commit_report = commit_ring_buffer_source_frames(buffer, chunk.frames)
                    .map_err(RingBufferRenderBoundaryError::CommitAfterWriteFailed)?;
                outcome.write_attempts += 1;
                outcome.frames_peeked_total += chunk.frames;
                outcome.frames_written_total += report.frames_written;
                outcome.frames_committed_total += commit_report.consumed_frames;
                outcome.iterations_completed += 1;
            }
        }
    }

    outcome.completed = true;
    Ok(outcome)
}

pub(crate) fn pcm_render_format_from_cache_for_boundary(
    cache: &WasapiFormatCache,
) -> Result<PcmRenderFormat, RingBufferRenderBoundaryError> {
    if !cache.is_float32() {
        return Err(RingBufferRenderBoundaryError::UnsupportedFormat);
    }

    validate_render_format(PcmRenderFormat {
        sample_rate_hz: cache.sample_rate_hz,
        channels: cache.channels,
        bits_per_sample: cache.bits_per_sample,
        block_align: cache.block_align,
        sample_format: PcmSampleFormat::Float32Interleaved,
    })
    .map_err(RingBufferRenderBoundaryError::PcmAdapter)
}

pub(crate) fn validate_boundary_write_report(
    report: &WasapiRenderWriteReport,
    expected_frames: u32,
    expected_bytes: u32,
) -> Result<(), RingBufferRenderBoundaryError> {
    if report.frames_written != expected_frames {
        return Err(RingBufferRenderBoundaryError::WrittenFrameCountMismatch {
            expected: expected_frames,
            actual: report.frames_written,
        });
    }
    if report.bytes_written != expected_bytes {
        return Err(RingBufferRenderBoundaryError::WrittenByteCountMismatch {
            expected: expected_bytes,
            actual: report.bytes_written,
        });
    }
    if report.used_silent_flag {
        return Err(RingBufferRenderBoundaryError::UnexpectedSilentFlag);
    }
    Ok(())
}

fn update_snapshot_fields(
    outcome: &mut RingBufferRenderBoundaryOutcome,
    snapshot: WasapiPaddingStateSnapshot,
) {
    outcome.last_capacity = snapshot.buffer_frame_capacity;
    outcome.last_padding = snapshot.current_padding_frames;
    outcome.last_available = snapshot.available_frames;
}

fn validate_config(
    config: RingBufferRenderBoundaryConfig,
) -> Result<RingBufferRenderBoundaryConfig, RingBufferRenderBoundaryError> {
    if config.iterations == 0 {
        return Err(RingBufferRenderBoundaryError::InvalidIterationCount);
    }
    if config.max_frames_per_write == 0 {
        return Err(RingBufferRenderBoundaryError::InvalidFrameCount);
    }
    if config.iterations > MAX_RING_BUFFER_RENDER_BOUNDARY_ITERATIONS {
        return Err(RingBufferRenderBoundaryError::IterationCountTooLarge);
    }
    if config.max_frames_per_write > MAX_RING_BUFFER_RENDER_BOUNDARY_FRAMES_PER_WRITE {
        return Err(RingBufferRenderBoundaryError::FrameCountTooLarge);
    }
    Ok(config)
}

fn checked_u32_len(len: usize) -> Result<u32, RingBufferRenderBoundaryError> {
    u32::try_from(len).map_err(|_| RingBufferRenderBoundaryError::ByteLengthOverflow)
}
