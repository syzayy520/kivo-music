//! Output-thread owned state default contract and validation.
//!
//! Provides a default contract builder and a validation function
//! that enforces the ownership boundary established by P0-074E5-PRE.

use super::output_thread_owned_state_error::WasapiOutputThreadOwnedStateError;
use super::output_thread_owned_state_report::{
    WasapiOutputThreadOwnedStateContract, WasapiOutputThreadOwnedStateOwner,
    WasapiOutputThreadOwnedStateStage,
};

/// Build the default output-thread owned state contract.
///
/// The default contract expresses:
/// - Stage: ContractOnly (no real resources)
/// - Sink: SplitProducerConsumer
/// - WasapiContext: OutputThread
/// - RingBuffer producer: MainThread
/// - RingBuffer consumer: OutputThread
/// - Adapter slot: OutputThread
/// - Command channel: NotAllocated
/// - All runtime flags: false
#[allow(dead_code)]
pub(crate) fn build_default_output_thread_owned_state_contract(
) -> WasapiOutputThreadOwnedStateContract {
    WasapiOutputThreadOwnedStateContract {
        stage: WasapiOutputThreadOwnedStateStage::ContractOnly,

        sink_owner: WasapiOutputThreadOwnedStateOwner::SplitProducerConsumer,
        wasapi_context_owner: WasapiOutputThreadOwnedStateOwner::OutputThread,
        ring_buffer_producer_owner: WasapiOutputThreadOwnedStateOwner::MainThread,
        ring_buffer_consumer_owner: WasapiOutputThreadOwnedStateOwner::OutputThread,
        adapter_slot_owner: WasapiOutputThreadOwnedStateOwner::OutputThread,
        command_channel_owner: WasapiOutputThreadOwnedStateOwner::NotAllocated,

        thread_spawned: false,
        channel_created: false,
        audio_client_start_allowed: false,
        native_pipeline_connected: false,
        playback_capabilities_enabled: false,
    }
}

/// Validate an output-thread owned state contract against the expected rules.
///
/// Returns `Ok(())` if the contract is valid, or a specific error variant
/// describing which rule was violated.
#[allow(dead_code)]
pub(crate) fn validate_output_thread_owned_state_contract(
    contract: &WasapiOutputThreadOwnedStateContract,
) -> Result<(), WasapiOutputThreadOwnedStateError> {
    if contract.stage != WasapiOutputThreadOwnedStateStage::ContractOnly {
        return Err(WasapiOutputThreadOwnedStateError::InvalidStage);
    }
    if contract.wasapi_context_owner != WasapiOutputThreadOwnedStateOwner::OutputThread {
        return Err(WasapiOutputThreadOwnedStateError::InvalidContextOwner);
    }
    if contract.sink_owner != WasapiOutputThreadOwnedStateOwner::SplitProducerConsumer {
        return Err(WasapiOutputThreadOwnedStateError::InvalidSinkOwner);
    }
    if contract.ring_buffer_producer_owner != WasapiOutputThreadOwnedStateOwner::MainThread {
        return Err(WasapiOutputThreadOwnedStateError::InvalidRingBufferProducerOwner);
    }
    if contract.ring_buffer_consumer_owner != WasapiOutputThreadOwnedStateOwner::OutputThread {
        return Err(WasapiOutputThreadOwnedStateError::InvalidRingBufferConsumerOwner);
    }
    if contract.adapter_slot_owner != WasapiOutputThreadOwnedStateOwner::OutputThread {
        return Err(WasapiOutputThreadOwnedStateError::InvalidAdapterSlotOwner);
    }
    if contract.command_channel_owner != WasapiOutputThreadOwnedStateOwner::NotAllocated {
        return Err(WasapiOutputThreadOwnedStateError::InvalidCommandChannelOwner);
    }
    if contract.thread_spawned {
        return Err(WasapiOutputThreadOwnedStateError::ThreadAlreadySpawned);
    }
    if contract.channel_created {
        return Err(WasapiOutputThreadOwnedStateError::ChannelAlreadyCreated);
    }
    if contract.audio_client_start_allowed {
        return Err(WasapiOutputThreadOwnedStateError::StartNotAllowedYet);
    }
    if contract.native_pipeline_connected {
        return Err(WasapiOutputThreadOwnedStateError::NativePipelineAlreadyConnected);
    }
    if contract.playback_capabilities_enabled {
        return Err(WasapiOutputThreadOwnedStateError::CapabilitiesAlreadyEnabled);
    }
    Ok(())
}
