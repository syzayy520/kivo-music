use super::pcm_adapter::{validate_render_bytes, PcmRenderFormat, PcmSampleFormat};
use super::ring_buffer::{RingBuffer, RingBufferFormat};
use super::ring_buffer_source::{
    commit_ring_buffer_source_frames, peek_ring_buffer_source_chunk, RingBufferSourceConfig,
    RingBufferSourceError, RingBufferSourceStatus,
};

fn buffer_format() -> RingBufferFormat {
    RingBufferFormat {
        sample_rate_hz: 44_100,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
    }
}

fn render_format() -> PcmRenderFormat {
    PcmRenderFormat {
        sample_rate_hz: 44_100,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
        sample_format: PcmSampleFormat::Float32Interleaved,
    }
}

fn config(max_frames: u32) -> RingBufferSourceConfig {
    RingBufferSourceConfig {
        max_frames,
        render_format: render_format(),
    }
}

fn buffer(capacity_frames: u32) -> RingBuffer {
    RingBuffer::new(buffer_format(), capacity_frames).expect("buffer")
}

fn frame_bytes(values: &[u8]) -> Vec<u8> {
    values.iter().flat_map(|value| [*value; 8]).collect()
}

fn chunk(status: RingBufferSourceStatus) -> super::ring_buffer_source::RingBufferSourceChunk {
    match status {
        RingBufferSourceStatus::Chunk(chunk) => chunk,
        other => panic!("expected chunk, got {other:?}"),
    }
}

#[test]
fn ring_buffer_source_peek_chunk_matches_pcm_adapter_format() {
    let mut rb = buffer(4);
    rb.write_frames(&frame_bytes(&[1, 2])).expect("write");
    let chunk = chunk(peek_ring_buffer_source_chunk(&rb, config(2)).expect("peek"));
    assert_eq!(chunk.requested_frames, 2);
    assert_eq!(chunk.available_frames_before_peek, 2);
    assert_eq!(chunk.frames, 2);
    assert_eq!(chunk.bytes.len(), 16);
    assert!(validate_render_bytes(render_format(), chunk.frames, &chunk.bytes).is_ok());
}

#[test]
fn ring_buffer_source_peek_does_not_consume_frames() {
    let mut rb = buffer(4);
    rb.write_frames(&frame_bytes(&[1, 2])).expect("write");
    let before = rb.available_frames();
    let _ = peek_ring_buffer_source_chunk(&rb, config(2)).expect("peek");
    assert_eq!(rb.available_frames(), before);
}

#[test]
fn ring_buffer_source_commit_consumes_after_explicit_call() {
    let mut rb = buffer(4);
    rb.write_frames(&frame_bytes(&[1, 2, 3])).expect("write");
    let chunk = chunk(peek_ring_buffer_source_chunk(&rb, config(2)).expect("peek"));
    let report = commit_ring_buffer_source_frames(&mut rb, chunk.frames).expect("commit");
    assert_eq!(report.consumed_frames, 2);
    assert_eq!(report.available_frames_after_commit, 1);
}

#[test]
fn ring_buffer_source_partial_peek_when_available_less_than_requested() {
    let mut rb = buffer(4);
    rb.write_frames(&frame_bytes(&[1])).expect("write");
    let chunk = chunk(peek_ring_buffer_source_chunk(&rb, config(3)).expect("peek"));
    assert!(chunk.partial);
    assert_eq!(chunk.frames, 1);
    assert_eq!(chunk.bytes.len(), 8);
}

#[test]
fn ring_buffer_source_empty_statuses_are_explicit() {
    let mut closed = buffer(2);
    closed.close();

    assert_eq!(
        peek_ring_buffer_source_chunk(&buffer(2), config(1)),
        Ok(RingBufferSourceStatus::EmptyOpen)
    );
    assert_eq!(
        peek_ring_buffer_source_chunk(&closed, config(1)),
        Ok(RingBufferSourceStatus::EmptyClosed)
    );
}

#[test]
fn ring_buffer_source_format_mismatches_are_rejected() {
    let cases: [fn(&mut PcmRenderFormat); 4] = [
        |format| format.sample_rate_hz = 48_000,
        |format| {
            format.channels = 1;
            format.block_align = 4;
        },
        |format| format.bits_per_sample = 16,
        |format| format.block_align = 4,
    ];

    for mutate in cases {
        let mut cfg = config(1);
        mutate(&mut cfg.render_format);
        assert!(peek_ring_buffer_source_chunk(&buffer(2), cfg).is_err());
    }
}

#[test]
fn ring_buffer_source_unsupported_pcm_sample_format_rejected() {
    let mut cfg = config(1);
    cfg.render_format.sample_format = PcmSampleFormat::Signed16Interleaved;

    assert!(matches!(
        peek_ring_buffer_source_chunk(&buffer(2), cfg),
        Err(RingBufferSourceError::PcmAdapter(_))
    ));
}

#[test]
fn ring_buffer_source_invalid_max_frames_rejected() {
    assert_eq!(
        peek_ring_buffer_source_chunk(&buffer(2), config(0)),
        Err(RingBufferSourceError::InvalidMaxFrames)
    );
}

#[test]
fn ring_buffer_source_commit_zero_frames_is_noop() {
    let mut rb = buffer(2);
    rb.write_frames(&frame_bytes(&[1])).expect("write");

    let report = commit_ring_buffer_source_frames(&mut rb, 0).expect("commit");

    assert_eq!(report.consumed_frames, 0);
    assert_eq!(report.available_frames_before_commit, 1);
    assert_eq!(report.available_frames_after_commit, 1);
    assert_eq!(rb.available_frames(), 1);
}

#[test]
fn ring_buffer_source_commit_more_than_available_returns_error_without_partial_consume() {
    let mut rb = buffer(2);
    rb.write_frames(&frame_bytes(&[1])).expect("write");

    assert_eq!(
        commit_ring_buffer_source_frames(&mut rb, 2),
        Err(RingBufferSourceError::CommitFrameCountMismatch {
            requested_frames: 2,
            available_frames: 1,
            consumed_frames: 0,
        })
    );
    assert_eq!(rb.available_frames(), 1);
}

#[test]
fn ring_buffer_source_wraparound_peek_preserves_order() {
    let mut rb = buffer(3);
    rb.write_frames(&frame_bytes(&[1, 2, 3])).expect("write");
    commit_ring_buffer_source_frames(&mut rb, 2).expect("commit");
    rb.write_frames(&frame_bytes(&[4, 5])).expect("write");

    let chunk = chunk(peek_ring_buffer_source_chunk(&rb, config(3)).expect("peek"));

    assert_eq!(chunk.frames, 3);
    assert_eq!(chunk.bytes, frame_bytes(&[3, 4, 5]));
}

#[test]
fn ring_buffer_source_no_silence_fill_on_underflow() {
    let mut rb = buffer(4);
    rb.write_frames(&frame_bytes(&[7])).expect("write");

    let chunk = chunk(peek_ring_buffer_source_chunk(&rb, config(4)).expect("peek"));

    assert_eq!(chunk.frames, 1);
    assert_eq!(chunk.bytes.len(), 8);
    assert_eq!(chunk.bytes, frame_bytes(&[7]));
}
