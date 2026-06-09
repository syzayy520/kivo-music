//! In-memory render source types.
//!
//! Pure data types for an in-memory render source implementation.
//! No actual audio data, no WASAPI, no IO, no threads.

pub mod adapter_integration;
pub mod exhaustion;
pub mod inmemory_source;
pub mod source_packet;
pub mod source_queue;

// Re-export primary types for convenience.
pub use adapter_integration::{
    create_empty_source, create_source_with_eos, create_test_source, execute_test_dispatch,
    invoke_test_flush, invoke_test_peek, invoke_test_read, read_all_packets,
};
pub use exhaustion::{
    create_mixed_size_source, drain_until_exhausted, verify_repeated_exhaustion_returns_exhausted,
    verify_reset_allows_reread, DrainResult, SourceStepOutcome,
};
pub use inmemory_source::InMemoryRenderSource;
pub use source_packet::SourcePacketMetadata;
pub use source_queue::SourceQueue;
