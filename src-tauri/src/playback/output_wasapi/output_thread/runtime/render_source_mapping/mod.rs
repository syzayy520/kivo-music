//! Render source mapping types.
//!
//! Pure memory mappings between render source and sink boundary types.
//! No actual rendering, no WASAPI, no IO.

pub mod sink_to_source_mapper;
pub mod source_to_sink_mapper;

pub use sink_to_source_mapper::map_sink_request_to_source_request;
pub use source_to_sink_mapper::map_source_result_to_sink_request;
