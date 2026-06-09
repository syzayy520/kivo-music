//! Device buffer writer types.
//!
//! Pure data types for a device buffer writer implementation.
//! No actual audio data, no WASAPI, no IO, no threads.
//! Device-agnostic boundary for future real WASAPI device buffer writer.

pub mod fake_writer;
pub mod frame_bytes;
pub mod sink_integration;
pub mod wasapi_writer;
pub mod write_error;
pub mod write_request;
pub mod write_result;
pub mod writer_cursor;
pub mod writer_state;
pub mod writer_trait;

// Re-export primary types for convenience.
pub use fake_writer::FakeDeviceBufferWriter;
pub use sink_integration::{
    create_empty_writer, create_test_writer, create_writer_with_format, invoke_test_close,
    invoke_test_flush, invoke_test_write, write_all_packets,
};
pub use wasapi_writer::{
    WasapiDeviceBufferWriter, WasapiDeviceBufferWriterConfig, WasapiDeviceBufferWriterState,
};
pub use write_error::WriteError;
pub use write_request::WriteRequest;
pub use write_result::WriteResult;
pub use writer_cursor::WriterCursor;
pub use writer_state::WriterState;
pub use writer_trait::DeviceBufferWriter;
