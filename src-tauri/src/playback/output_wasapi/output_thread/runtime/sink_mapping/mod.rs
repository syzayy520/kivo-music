//! Sink mapping types.
//!
//! Pure memory mappings between runtime types and sink boundary types.
//! No actual rendering, no WASAPI, no IO.

pub mod driver_result_mapper;
pub mod sink_event_mapper;

pub use driver_result_mapper::map_driver_result_to_sink_request;
pub use sink_event_mapper::map_sink_result_to_thread_event;
