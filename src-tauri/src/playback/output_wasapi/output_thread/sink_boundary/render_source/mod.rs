//! Render source contract types.
//!
//! Device-agnostic types defining the interface between the output thread
//! runtime and a future audio source provider. No WASAPI, no IO, no actual decoding.

pub mod source_error;
pub mod source_request;
pub mod source_result;

pub use source_error::RenderSourceError;
pub use source_request::RenderSourceRequest;
pub use source_result::RenderSourceResult;
