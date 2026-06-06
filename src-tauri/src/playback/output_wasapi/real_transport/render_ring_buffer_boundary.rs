#![allow(dead_code)]

mod config;
mod constants;
mod error;
mod format;
mod outcome;
mod runner;
mod source;

#[allow(unused_imports)]
pub(crate) use config::RenderRingBufferBoundaryConfig;
#[allow(unused_imports)]
pub(crate) use constants::{
    MAX_RENDER_RING_BUFFER_BOUNDARY_FRAMES_PER_WRITE, MAX_RENDER_RING_BUFFER_BOUNDARY_ITERATIONS,
    MAX_SYNTHETIC_ZERO_SEED_FRAMES,
};
#[allow(unused_imports)]
pub(crate) use error::{map_ring_buffer_boundary_error, RenderRingBufferBoundaryError};
#[allow(unused_imports)]
pub(crate) use format::render_format_from_wasapi_cache_for_transport_boundary;
#[allow(unused_imports)]
pub(crate) use outcome::RenderRingBufferBoundaryOutcome;
#[allow(unused_imports)]
pub(crate) use runner::{
    run_render_ring_buffer_boundary, run_render_ring_buffer_boundary_with_callbacks,
};
#[allow(unused_imports)]
pub(crate) use source::build_synthetic_zero_ring_buffer_source;
