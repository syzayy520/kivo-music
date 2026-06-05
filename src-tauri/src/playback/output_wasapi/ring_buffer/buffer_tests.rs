use super::buffer::RingBuffer;
use super::types::RingBufferFormat;

fn sample_format() -> RingBufferFormat {
    RingBufferFormat {
        sample_rate_hz: 44100,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
    }
}

#[test]
fn ring_buffer_format_is_comparable() {
    let a = sample_format();
    let b = sample_format();
    assert_eq!(a, b);

    let c = RingBufferFormat {
        sample_rate_hz: 48000,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
    };
    assert_ne!(a, c);
}

#[test]
fn ring_buffer_exposes_format_read_only() {
    let fmt = sample_format();
    let rb = RingBuffer::new(fmt, 1024).unwrap();
    assert_eq!(rb.format(), fmt);

    // format unchanged after write
    let bytes = vec![0u8; 8]; // 1 frame
    let mut rb = rb;
    rb.write_frames(&bytes).unwrap();
    assert_eq!(rb.format(), fmt);
}
