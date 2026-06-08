//! Runtime event emission buffer.
//!
//! Pure memory buffer for events emitted during driver steps.

#[allow(clippy::module_inception)]
pub mod event_buffer;
pub mod event_drain;

pub use event_buffer::EventBuffer;
pub use event_drain::drain_buffer;
