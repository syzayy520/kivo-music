//! Output-thread owned state default contract and validation.
//!
//! Provides a default contract builder and a validation function
//! that enforces the ownership boundary established by P0-074E5-PRE.

use super::output_thread_owned_state_error::WasapiOutputThreadOwnedStateError;
use super::output_thread_owned_state_report::{
    WasapiOutputThreadOwnedStateContract, WasapiOutputThreadOwnedStateOwner,
    WasapiOutputThreadOwnedStateReport, WasapiOutputThreadOwnedStateStage,
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
#[allow(dead_code)] // temporary until P0-074E5B command contract wiring
pub(crate) fn default_wasapi_output_thread_owned_state_contract(
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
/// Returns `Ok(report)` if the contract is valid, or a specific error variant
/// describing which rule was violated.
#[allow(dead_code)] // temporary until P0-074E5B command contract wiring
pub(crate) fn validate_wasapi_output_thread_owned_state_contract(
    contract: WasapiOutputThreadOwnedStateContract,
) -> Result<WasapiOutputThreadOwnedStateReport, WasapiOutputThreadOwnedStateError> {
    if contract.stage != WasapiOutputThreadOwnedStateStage::ContractOnly {
        return Err(WasapiOutputThreadOwnedStateError::InvalidStage);
    }
    if contract.sink_owner != WasapiOutputThreadOwnedStateOwner::SplitProducerConsumer {
        return Err(WasapiOutputThreadOwnedStateError::InvalidSinkOwner);
    }
    if contract.wasapi_context_owner != WasapiOutputThreadOwnedStateOwner::OutputThread {
        return Err(WasapiOutputThreadOwnedStateError::WasapiContextMustBeOutputThreadOwned);
    }
    if contract.ring_buffer_producer_owner != WasapiOutputThreadOwnedStateOwner::MainThread {
        return Err(WasapiOutputThreadOwnedStateError::RingBufferProducerMustBeMainThreadOwned);
    }
    if contract.ring_buffer_consumer_owner != WasapiOutputThreadOwnedStateOwner::OutputThread {
        return Err(WasapiOutputThreadOwnedStateError::RingBufferConsumerMustBeOutputThreadOwned);
    }
    if contract.adapter_slot_owner != WasapiOutputThreadOwnedStateOwner::OutputThread {
        return Err(WasapiOutputThreadOwnedStateError::AdapterSlotMustBeOutputThreadOwned);
    }
    if contract.command_channel_owner != WasapiOutputThreadOwnedStateOwner::NotAllocated {
        return Err(WasapiOutputThreadOwnedStateError::CommandChannelMustNotBeAllocatedYet);
    }
    if contract.thread_spawned {
        return Err(WasapiOutputThreadOwnedStateError::ThreadMustNotBeSpawnedYet);
    }
    if contract.channel_created {
        return Err(WasapiOutputThreadOwnedStateError::ChannelMustNotBeCreatedYet);
    }
    if contract.audio_client_start_allowed {
        return Err(WasapiOutputThreadOwnedStateError::AudioClientStartMustNotBeAllowedYet);
    }
    if contract.native_pipeline_connected {
        return Err(WasapiOutputThreadOwnedStateError::NativePipelineMustNotBeConnectedYet);
    }
    if contract.playback_capabilities_enabled {
        return Err(WasapiOutputThreadOwnedStateError::PlaybackCapabilitiesMustNotBeEnabledYet);
    }
    Ok(WasapiOutputThreadOwnedStateReport {
        contract,
        valid_for_current_scaffold: true,
    })
}
