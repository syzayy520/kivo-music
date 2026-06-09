//! Sink integration for device buffer writer.
//!
//! Helper functions for testing the writer-to-sink dispatch chain
//! using FakeDeviceBufferWriter. No threads, no IO, no WASAPI.

use super::fake_writer::FakeDeviceBufferWriter;
use super::{DeviceBufferWriter, WriteError, WriteRequest, WriteResult};

/// Creates a FakeDeviceBufferWriter with given capacity for integration testing.
pub fn create_test_writer(capacity: u64) -> FakeDeviceBufferWriter {
    FakeDeviceBufferWriter::new(capacity)
}

/// Creates an empty FakeDeviceBufferWriter with default capacity.
pub fn create_empty_writer() -> FakeDeviceBufferWriter {
    FakeDeviceBufferWriter::empty()
}

/// Creates a FakeDeviceBufferWriter with specified format.
pub fn create_writer_with_format(
    capacity: u64,
    sample_rate: u32,
    channel_count: u16,
) -> FakeDeviceBufferWriter {
    FakeDeviceBufferWriter::with_format(capacity, sample_rate, channel_count)
}

/// Invokes a FakeDeviceBufferWriter with a WritePacket request and returns the result.
pub fn invoke_test_write(
    writer: &mut FakeDeviceBufferWriter,
    frame_count: u64,
    sample_rate: u32,
    channel_count: u16,
) -> Result<WriteResult, WriteError> {
    let request = WriteRequest::WritePacket {
        frame_count,
        sample_rate,
        channel_count,
    };
    writer.process_request(&request)
}

/// Invokes a FakeDeviceBufferWriter with a Flush request and returns the result.
pub fn invoke_test_flush(writer: &mut FakeDeviceBufferWriter) -> Result<WriteResult, WriteError> {
    writer.process_request(&WriteRequest::Flush)
}

/// Invokes a FakeDeviceBufferWriter with a Close request and returns the result.
pub fn invoke_test_close(writer: &mut FakeDeviceBufferWriter) -> Result<WriteResult, WriteError> {
    writer.process_request(&WriteRequest::Close)
}

/// Writes multiple packets to a writer until capacity is reached, returning (total_frames, total_bytes).
pub fn write_all_packets(
    writer: &mut FakeDeviceBufferWriter,
    frame_count: u64,
    sample_rate: u32,
    channel_count: u16,
) -> (u64, u64) {
    let mut total_frames = 0;
    let mut total_bytes = 0;

    loop {
        match invoke_test_write(writer, frame_count, sample_rate, channel_count) {
            Ok(WriteResult::Written {
                frames_written,
                bytes_written,
            }) => {
                total_frames += frames_written;
                total_bytes += bytes_written;
            }
            Ok(_) => break,
            Err(_) => break,
        }
    }

    (total_frames, total_bytes)
}
