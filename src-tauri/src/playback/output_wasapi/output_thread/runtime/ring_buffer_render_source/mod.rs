//! Ring buffer render source types.
//!
//! Pure data types for a ring buffer render source implementation.
//! No actual audio data, no WASAPI, no IO, no threads.
//! Device-agnostic boundary for future real RingBuffer consumer.

pub mod adapter_integration;
pub mod buffer_cursor;
pub mod buffer_packet;
pub mod buffer_state;
pub mod fake_ring_buffer_source;
pub mod read_error;
pub mod read_result;

// Re-export primary types for convenience.
pub use adapter_integration::{
    create_empty_source, create_source_with_eos, create_test_source, execute_test_dispatch,
    invoke_test_flush, invoke_test_peek, invoke_test_read, read_all_packets,
};
pub use buffer_cursor::BufferCursor;
pub use buffer_packet::BufferPacket;
pub use buffer_state::BufferState;
pub use fake_ring_buffer_source::FakeRingBufferSource;
pub use read_error::RingBufferReadError;
pub use read_result::RingBufferReadResult;
