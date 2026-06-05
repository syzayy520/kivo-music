#![allow(dead_code)]

use super::pcm_adapter::{
    validate_render_bytes, validate_render_format, PcmAdapterError, PcmRenderFormat,
    PcmSampleFormat,
};
use super::ring_buffer::{RingBuffer, RingBufferError, RingBufferFormat};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RingBufferSourceConfig {
    pub max_frames: u32,
    pub render_format: PcmRenderFormat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RingBufferSourceStatus {
    Chunk(RingBufferSourceChunk),
    EmptyOpen,
    EmptyClosed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RingBufferSourceChunk {
    pub requested_frames: u32,
    pub available_frames_before_peek: u32,
    pub frames: u32,
    pub bytes: Vec<u8>,
    pub partial: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RingBufferSourceCommitReport {
    pub requested_frames: u32,
    pub available_frames_before_commit: u32,
    pub consumed_frames: u32,
    pub available_frames_after_commit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RingBufferSourceError {
    InvalidMaxFrames,
    RingBufferFormatMismatch,
    RingBufferReadFailed(RingBufferError),
    PcmAdapter(PcmAdapterError),
    PeekedFrameCountMismatch {
        requested_frames: u32,
        available_frames: u32,
        peeked_frames: u32,
    },
    CommitFrameCountMismatch {
        requested_frames: u32,
        available_frames: u32,
        consumed_frames: u32,
    },
    ByteLengthOverflow,
}

pub(crate) fn peek_ring_buffer_source_chunk(
    buffer: &RingBuffer,
    config: RingBufferSourceConfig,
) -> Result<RingBufferSourceStatus, RingBufferSourceError> {
    let config = validate_config(config)?;
    validate_format_agreement(buffer.format(), config.render_format)?;

    let available_before = buffer.available_frames();
    if available_before == 0 {
        return if buffer.is_closed() {
            Ok(RingBufferSourceStatus::EmptyClosed)
        } else {
            Ok(RingBufferSourceStatus::EmptyOpen)
        };
    }

    let frames_to_peek = config.max_frames.min(available_before);
    let mut bytes = vec![0_u8; checked_byte_len(frames_to_peek, config.render_format.block_align)?];
    let peeked_frames = buffer
        .peek_frames(&mut bytes)
        .map_err(RingBufferSourceError::RingBufferReadFailed)?;

    if peeked_frames == 0 || peeked_frames > frames_to_peek {
        return Err(RingBufferSourceError::PeekedFrameCountMismatch {
            requested_frames: frames_to_peek,
            available_frames: available_before,
            peeked_frames,
        });
    }

    if buffer.available_frames() != available_before {
        return Err(RingBufferSourceError::PeekedFrameCountMismatch {
            requested_frames: frames_to_peek,
            available_frames: available_before,
            peeked_frames,
        });
    }

    let actual_len = checked_byte_len(peeked_frames, config.render_format.block_align)?;
    bytes.truncate(actual_len);
    validate_render_bytes(config.render_format, peeked_frames, &bytes)
        .map_err(RingBufferSourceError::PcmAdapter)?;

    Ok(RingBufferSourceStatus::Chunk(RingBufferSourceChunk {
        requested_frames: config.max_frames,
        available_frames_before_peek: available_before,
        frames: peeked_frames,
        partial: peeked_frames < config.max_frames,
        bytes,
    }))
}

pub(crate) fn commit_ring_buffer_source_frames(
    buffer: &mut RingBuffer,
    frames: u32,
) -> Result<RingBufferSourceCommitReport, RingBufferSourceError> {
    let available_before = buffer.available_frames();

    if frames == 0 {
        return Ok(RingBufferSourceCommitReport {
            requested_frames: 0,
            available_frames_before_commit: available_before,
            consumed_frames: 0,
            available_frames_after_commit: available_before,
        });
    }

    if frames > available_before {
        return Err(RingBufferSourceError::CommitFrameCountMismatch {
            requested_frames: frames,
            available_frames: available_before,
            consumed_frames: 0,
        });
    }

    let consumed = buffer
        .consume_frames(frames)
        .map_err(RingBufferSourceError::RingBufferReadFailed)?;

    if consumed != frames {
        return Err(RingBufferSourceError::CommitFrameCountMismatch {
            requested_frames: frames,
            available_frames: available_before,
            consumed_frames: consumed,
        });
    }

    Ok(RingBufferSourceCommitReport {
        requested_frames: frames,
        available_frames_before_commit: available_before,
        consumed_frames: consumed,
        available_frames_after_commit: buffer.available_frames(),
    })
}

fn validate_config(
    config: RingBufferSourceConfig,
) -> Result<RingBufferSourceConfig, RingBufferSourceError> {
    if config.max_frames == 0 {
        return Err(RingBufferSourceError::InvalidMaxFrames);
    }

    validate_render_format(config.render_format).map_err(RingBufferSourceError::PcmAdapter)?;

    if config.render_format.sample_format != PcmSampleFormat::Float32Interleaved {
        return Err(RingBufferSourceError::PcmAdapter(
            PcmAdapterError::UnsupportedSampleFormat,
        ));
    }

    Ok(config)
}

fn validate_format_agreement(
    buffer_format: RingBufferFormat,
    render_format: PcmRenderFormat,
) -> Result<(), RingBufferSourceError> {
    let matches = buffer_format.sample_rate_hz == render_format.sample_rate_hz
        && buffer_format.channels == render_format.channels
        && buffer_format.bits_per_sample == render_format.bits_per_sample
        && buffer_format.block_align == render_format.block_align;

    if matches {
        Ok(())
    } else {
        Err(RingBufferSourceError::RingBufferFormatMismatch)
    }
}

fn checked_byte_len(frames: u32, block_align: u16) -> Result<usize, RingBufferSourceError> {
    let frames = usize::try_from(frames).map_err(|_| RingBufferSourceError::ByteLengthOverflow)?;

    frames
        .checked_mul(usize::from(block_align))
        .ok_or(RingBufferSourceError::ByteLengthOverflow)
}
