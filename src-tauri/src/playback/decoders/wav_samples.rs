pub fn normalize_int_sample(value: i32, bits_per_sample: u16) -> f32 {
    let max = (1_i64 << (bits_per_sample.saturating_sub(1))) - 1;
    if max <= 0 {
        return 0.0;
    }

    (value as f64 / max as f64).clamp(-1.0, 1.0) as f32
}
