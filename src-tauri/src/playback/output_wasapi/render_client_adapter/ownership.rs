//! Render client ownership placeholder.
//!
//! Defines the ownership model for a real WASAPI render client without
//! holding any actual COM objects or Windows resources. This is a pure
//! placeholder for future IAudioRenderClient ownership.
//!
//! **IMPORTANT**: This struct does NOT:
//! - Hold COM pointers
//! - Own IAudioRenderClient
//! - Own IAudioClient
//! - Hold any Windows resource handles

/// Ownership state for a render client adapter.
///
/// Tracks whether the adapter conceptually owns a render client without
/// holding any real Windows resources. Future real WASAPI integration
/// will add COM pointer fields here.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RenderClientOwnership {
    /// Whether the adapter has been assigned ownership of a render client.
    has_owner: bool,
    /// Human-readable device identifier (e.g., "Speakers (Realtek)").
    device_id: Option<String>,
    /// Audio format description (e.g., "48000Hz 2ch f32").
    format_description: Option<String>,
}

impl RenderClientOwnership {
    /// Creates a new unowned placeholder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an owned placeholder with device info.
    pub fn with_device(device_id: String, format_description: String) -> Self {
        Self {
            has_owner: true,
            device_id: Some(device_id),
            format_description: Some(format_description),
        }
    }

    /// Returns true if the adapter owns a render client.
    pub fn has_owner(&self) -> bool {
        self.has_owner
    }

    /// Returns the device identifier, if any.
    pub fn device_id(&self) -> Option<&str> {
        self.device_id.as_deref()
    }

    /// Returns the format description, if any.
    pub fn format_description(&self) -> Option<&str> {
        self.format_description.as_deref()
    }

    /// Clears ownership (e.g., on device lost or close).
    pub fn clear(&mut self) {
        self.has_owner = false;
        self.device_id = None;
        self.format_description = None;
    }
}
