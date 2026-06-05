//! Validation errors for the output-thread owned state contract.

/// Errors returned when a contract violates the expected ownership rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WasapiOutputThreadOwnedStateError {
    /// Contract stage must be `ContractOnly`.
    InvalidStage,
    /// `sink_owner` must be `SplitProducerConsumer`.
    InvalidSinkOwner,
    /// `wasapi_context_owner` must be `OutputThread`.
    WasapiContextMustBeOutputThreadOwned,
    /// `ring_buffer_producer_owner` must be `MainThread`.
    RingBufferProducerMustBeMainThreadOwned,
    /// `ring_buffer_consumer_owner` must be `OutputThread`.
    RingBufferConsumerMustBeOutputThreadOwned,
    /// `adapter_slot_owner` must be `OutputThread`.
    AdapterSlotMustBeOutputThreadOwned,
    /// `command_channel_owner` must be `NotAllocated`.
    CommandChannelMustNotBeAllocatedYet,
    /// `thread_spawned` must be `false`.
    ThreadMustNotBeSpawnedYet,
    /// `channel_created` must be `false`.
    ChannelMustNotBeCreatedYet,
    /// `audio_client_start_allowed` must be `false`.
    AudioClientStartMustNotBeAllowedYet,
    /// `native_pipeline_connected` must be `false`.
    NativePipelineMustNotBeConnectedYet,
    /// `playback_capabilities_enabled` must be `false`.
    PlaybackCapabilitiesMustNotBeEnabledYet,
}
