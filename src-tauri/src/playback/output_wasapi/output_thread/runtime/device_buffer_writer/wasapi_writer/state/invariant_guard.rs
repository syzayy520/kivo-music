//! WasapiDeviceBufferWriterState invariant checking.

use super::buffer_lifecycle::BufferLifecycle;
use super::WasapiDeviceBufferWriterState;

impl WasapiDeviceBufferWriterState {
    /// Checks runtime invariants and returns any violations.
    ///
    /// Returns Ok(()) if all invariants hold, or Err(description) if violated.
    /// Used for debug assertions and runtime health checks.
    pub fn check_invariants(&self, capacity_frames: u64) -> Result<(), String> {
        // buffer_fill_frames must not exceed capacity
        if self.buffer_fill_frames > capacity_frames {
            return Err(format!(
                "buffer_fill_frames ({}) exceeds capacity ({})",
                self.buffer_fill_frames, capacity_frames
            ));
        }

        // write_head must be within capacity
        if capacity_frames > 0 && self.write_head >= capacity_frames {
            return Err(format!(
                "write_head ({}) exceeds capacity ({})",
                self.write_head, capacity_frames
            ));
        }

        // lifecycle must match actual state
        let expected_lifecycle = if self.is_closed {
            BufferLifecycle::Closed
        } else if self.buffer_fill_frames == 0 {
            BufferLifecycle::Empty
        } else if self.buffer_fill_frames >= capacity_frames {
            BufferLifecycle::Full
        } else {
            BufferLifecycle::Partial
        };
        if self.lifecycle != expected_lifecycle {
            return Err(format!(
                "lifecycle mismatch: expected {:?}, got {:?}",
                expected_lifecycle, self.lifecycle
            ));
        }

        // consecutive_would_blocks must not exceed max
        if self.consecutive_would_blocks > self.max_consecutive_would_blocks {
            return Err(format!(
                "consecutive_would_blocks ({}) exceeds max ({})",
                self.consecutive_would_blocks, self.max_consecutive_would_blocks
            ));
        }

        Ok(())
    }

    /// Returns true if all runtime invariants hold.
    pub fn invariants_hold(&self, capacity_frames: u64) -> bool {
        self.check_invariants(capacity_frames).is_ok()
    }
}
