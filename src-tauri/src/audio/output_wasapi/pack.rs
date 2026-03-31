//! Sample packing helpers for WASAPI render buffer.
//!
//! We convert interleaved `f32` PCM samples into the device mix format's byte layout.

#[cfg(windows)]
use wasapi::SampleType;

/// Bytes per sample for the given mix format.
#[cfg(windows)]
pub fn bytes_per_sample(sample_type: SampleType, bits_per_sample: usize) -> usize {
    match sample_type {
        SampleType::Float => 4, // IEEE float32
        SampleType::Int => bits_per_sample.div_ceil(8).max(1),
    }
}

#[inline]
fn clamp_f32(x: f32) -> f32 {
    if x.is_nan() {
        0.0
    } else {
        x.clamp(-1.0, 1.0)
    }
}

#[cfg(windows)]
pub fn pack_samples_into(
    out: &mut Vec<u8>,
    samples_f32: &[f32],
    sample_type: SampleType,
    bits_per_sample: usize,
    valid_bits: usize,
) {
    out.clear();

    match sample_type {
        SampleType::Float => {
            out.reserve(samples_f32.len() * 4);
            for s in samples_f32 {
                out.extend_from_slice(&s.to_le_bytes());
            }
        }
        SampleType::Int => {
            // Note:
            // - bits_per_sample is container bits (16/24/32)
            // - valid_bits can be 24 even when container is 32 (24-in-32).
            let container_bytes = bits_per_sample.div_ceil(8).clamp(1, 4);
            out.reserve(samples_f32.len() * container_bytes);

            // We scale by valid bits if present; otherwise by container bits.
            let bits = valid_bits.max(bits_per_sample).min(32);
            let max = (1i64 << (bits.saturating_sub(1))) - 1;
            let min = -(1i64 << (bits.saturating_sub(1)));

            for s in samples_f32 {
                let x = clamp_f32(*s);
                let scaled = (x * (max as f32)).round() as i64;
                let v = scaled.clamp(min, max);

                match container_bytes {
                    2 => {
                        let i = v as i16;
                        out.extend_from_slice(&i.to_le_bytes());
                    }
                    3 => {
                        // 24-bit little endian signed
                        let i = v as i32;
                        let b = i.to_le_bytes();
                        out.extend_from_slice(&b[..3]);
                    }
                    4 => {
                        let i = v as i32;
                        out.extend_from_slice(&i.to_le_bytes());
                    }
                    _ => {
                        // Defensive fallback to 16-bit.
                        let i = v as i16;
                        out.extend_from_slice(&i.to_le_bytes());
                    }
                }
            }
        }
    }
}
