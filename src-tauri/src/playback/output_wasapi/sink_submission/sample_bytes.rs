//! Pure f32 sample to byte conversion.

/// Convert f32 samples to native-endian bytes.
///
/// Each f32 is written as 4 bytes in platform native byte order.
/// No resampling, no channel mixing, no format negotiation.
pub(crate) fn f32_samples_to_ne_bytes(samples: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 4);
    for &s in samples {
        bytes.extend_from_slice(&s.to_ne_bytes());
    }
    bytes
}
