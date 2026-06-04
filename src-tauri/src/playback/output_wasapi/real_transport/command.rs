//! Typed command for real transport channel.
//!
//! Wraps runtime intents and transport lifecycle commands.
//! Does not reference any sync/thread/WASAPI primitives.

use super::output_thread_runtime_intent::OutputThreadRuntimeIntent;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRealTransportCommand {
    /// Forward a runtime intent to the transport receiver.
    RuntimeIntent(OutputThreadRuntimeIntent),
    /// Signal the transport to close. Does not close a real thread.
    CloseTransport,
}

impl OutputThreadRealTransportCommand {
    /// Create a command wrapping a runtime intent.
    #[allow(dead_code)]
    pub(crate) fn runtime_intent(intent: OutputThreadRuntimeIntent) -> Self {
        Self::RuntimeIntent(intent)
    }

    /// Create a close-transport command.
    #[allow(dead_code)]
    pub(crate) fn close_transport() -> Self {
        Self::CloseTransport
    }

    /// Returns true if this is a CloseTransport command.
    #[allow(dead_code)]
    pub(crate) fn is_close(self) -> bool {
        matches!(self, Self::CloseTransport)
    }

    /// Extract the inner runtime intent, if present.
    #[allow(dead_code)]
    pub(crate) fn as_runtime_intent(self) -> Option<OutputThreadRuntimeIntent> {
        match self {
            Self::RuntimeIntent(intent) => Some(intent),
            Self::CloseTransport => None,
        }
    }
}
