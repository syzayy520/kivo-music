//! Request validation and error mapping for WasapiDeviceBufferWriter.
//!
//! Validates WritePacket parameters and maps RenderClientFailure
//! to WriteError for cross-boundary error propagation.

use super::super::WriteError;
use super::WasapiDeviceBufferWriter;
use crate::playback::output_wasapi::render_client_boundary::RenderClientFailure;

impl WasapiDeviceBufferWriter {
    /// Validates WritePacket request parameters.
    ///
    /// Checks that frame_count, channel_count, and sample_rate are all > 0.
    /// Returns InvalidRequest error if any parameter is zero.
    pub(super) fn validate_write_packet(
        &self,
        frame_count: u64,
        sample_rate: u32,
        channel_count: u16,
    ) -> Result<(), WriteError> {
        if frame_count == 0 {
            return Err(WriteError::InvalidRequest {
                reason: "frame_count must be > 0".to_string(),
            });
        }
        if channel_count == 0 {
            return Err(WriteError::InvalidRequest {
                reason: "channel_count must be > 0".to_string(),
            });
        }
        if sample_rate == 0 {
            return Err(WriteError::InvalidRequest {
                reason: "sample_rate must be > 0".to_string(),
            });
        }
        Ok(())
    }

    /// Maps a RenderClientFailure to a WriteError for cross-boundary error propagation.
    ///
    /// BufferTooSmall → WriteFailed (buffer full, not a WouldBlock at this level).
    /// DeviceLost → WriteFailed (hardware failure).
    /// Other failures map to their closest WriteError equivalent.
    pub(super) fn map_render_client_failure(failure: RenderClientFailure) -> WriteError {
        match failure {
            RenderClientFailure::BufferTooSmall { .. } => WriteError::WriteFailed {
                description: format!("render client buffer too small: {}", failure),
            },
            RenderClientFailure::DeviceLost => WriteError::WriteFailed {
                description: "render client device lost".to_string(),
            },
            RenderClientFailure::RenderClientUnavailable => WriteError::WriteFailed {
                description: "render client unavailable".to_string(),
            },
            RenderClientFailure::InvalidRequest { reason } => WriteError::InvalidRequest { reason },
            RenderClientFailure::BufferAcquisitionFailed { description } => {
                WriteError::WriteFailed {
                    description: format!(
                        "render client buffer acquisition failed: {}",
                        description
                    ),
                }
            }
            RenderClientFailure::BufferReleaseFailed { description } => WriteError::WriteFailed {
                description: format!("render client buffer release failed: {}", description),
            },
            RenderClientFailure::PaddingUnavailable => WriteError::Internal {
                description: "render client padding unavailable".to_string(),
            },
            RenderClientFailure::Internal { description } => WriteError::Internal { description },
        }
    }
}
