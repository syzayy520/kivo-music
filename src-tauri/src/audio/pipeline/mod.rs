//! Lightweight audio pipeline helpers (dependency-free).
//!
//! These are intentionally minimal and only cover what the shell needs:
//! - channel mapping (1<->2, N->M with truncation/zero-fill)
//! - linear resampling (good enough for shell smoke)

pub mod channel_map;
pub mod linear_resampler;
