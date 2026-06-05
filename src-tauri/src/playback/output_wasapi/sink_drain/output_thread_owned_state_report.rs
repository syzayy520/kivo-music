//! Output-thread owned state contract types.
//!
//! Defines the ownership boundary for a future real output thread.
//! Pure data types — no runtime resources, no threads, no channels.

/// Describes which actor owns a particular resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WasapiOutputThreadOwnedStateOwner {
    /// The resource has not been allocated yet.
    NotAllocated,
    /// The main thread owns the resource.
    MainThread,
    /// The future output thread owns the resource.
    OutputThread,
    /// Producer/consumer split across threads (used for RingBuffer boundary).
    SplitProducerConsumer,
}

/// Lifecycle stage of the output-thread owned state.
///
/// Only `ContractOnly` exists in this ticket.
/// Future tickets may add `ThreadSpawned`, `Running`, etc.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WasapiOutputThreadOwnedStateStage {
    /// Contract descriptor exists, but no real resources have been created.
    ContractOnly,
}

/// Ownership contract for a future real output thread.
///
/// Describes who owns what resource, without creating any real resources.
/// Validated by `validate_output_thread_owned_state_contract`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct WasapiOutputThreadOwnedStateContract {
    pub(crate) stage: WasapiOutputThreadOwnedStateStage,

    pub(crate) sink_owner: WasapiOutputThreadOwnedStateOwner,
    pub(crate) wasapi_context_owner: WasapiOutputThreadOwnedStateOwner,
    pub(crate) ring_buffer_producer_owner: WasapiOutputThreadOwnedStateOwner,
    pub(crate) ring_buffer_consumer_owner: WasapiOutputThreadOwnedStateOwner,
    pub(crate) adapter_slot_owner: WasapiOutputThreadOwnedStateOwner,
    pub(crate) command_channel_owner: WasapiOutputThreadOwnedStateOwner,

    pub(crate) thread_spawned: bool,
    pub(crate) channel_created: bool,
    pub(crate) audio_client_start_allowed: bool,
    pub(crate) native_pipeline_connected: bool,
    pub(crate) playback_capabilities_enabled: bool,
}
