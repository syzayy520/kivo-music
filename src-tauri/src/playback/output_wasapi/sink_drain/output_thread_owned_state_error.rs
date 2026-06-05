//! Validation errors for the output-thread owned state contract.

/// Errors returned when a contract violates the expected ownership rules.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WasapiOutputThreadOwnedStateError {
    /// Contract stage must be `ContractOnly`.
    InvalidStage,
    /// `wasapi_context_owner` must be `OutputThread`.
    InvalidContextOwner,
    /// `sink_owner` must be `SplitProducerConsumer`.
    InvalidSinkOwner,
    /// `ring_buffer_producer_owner` must be `MainThread`.
    InvalidRingBufferProducerOwner,
    /// `ring_buffer_consumer_owner` must be `OutputThread`.
    InvalidRingBufferConsumerOwner,
    /// `adapter_slot_owner` must be `OutputThread`.
    InvalidAdapterSlotOwner,
    /// `command_channel_owner` must be `NotAllocated`.
    InvalidCommandChannelOwner,
    /// `thread_spawned` must be `false`.
    ThreadAlreadySpawned,
    /// `channel_created` must be `false`.
    ChannelAlreadyCreated,
    /// `audio_client_start_allowed` must be `false`.
    StartNotAllowedYet,
    /// `native_pipeline_connected` must be `false`.
    NativePipelineAlreadyConnected,
    /// `playback_capabilities_enabled` must be `false`.
    CapabilitiesAlreadyEnabled,
}
