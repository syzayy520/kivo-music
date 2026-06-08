//! Sink consumer contract types.
//!
//! Device-agnostic trait and snapshot types for a future sink consumer.
//! No WASAPI, no IO, no actual rendering.

pub mod consumer_contract;
pub mod consumer_snapshot;

pub use consumer_contract::SinkConsumer;
pub use consumer_snapshot::ConsumerSnapshot;
