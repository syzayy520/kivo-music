//! Tests for output-thread owned state contract.

use super::output_thread_owned_state::{
    default_wasapi_output_thread_owned_state_contract,
    validate_wasapi_output_thread_owned_state_contract,
};
use super::output_thread_owned_state_error::WasapiOutputThreadOwnedStateError;
use super::output_thread_owned_state_report::{
    WasapiOutputThreadOwnedStateOwner, WasapiOutputThreadOwnedStateStage,
};

/// A: Default contract stage is ContractOnly and all runtime flags are false.
#[test]
fn default_contract_is_contract_only() {
    let c = default_wasapi_output_thread_owned_state_contract();
    assert_eq!(c.stage, WasapiOutputThreadOwnedStateStage::ContractOnly);
    assert!(!c.thread_spawned);
    assert!(!c.channel_created);
    assert!(!c.audio_client_start_allowed);
    assert!(!c.native_pipeline_connected);
    assert!(!c.playback_capabilities_enabled);
}

/// B: Default contract records future context owner as OutputThread.
#[test]
fn default_contract_records_future_context_owner() {
    let c = default_wasapi_output_thread_owned_state_contract();
    assert_eq!(
        c.wasapi_context_owner,
        WasapiOutputThreadOwnedStateOwner::OutputThread
    );
}

/// C: Default contract records split ring buffer ownership.
#[test]
fn default_contract_records_split_ring_buffer_ownership() {
    let c = default_wasapi_output_thread_owned_state_contract();
    assert_eq!(
        c.sink_owner,
        WasapiOutputThreadOwnedStateOwner::SplitProducerConsumer
    );
    assert_eq!(
        c.ring_buffer_producer_owner,
        WasapiOutputThreadOwnedStateOwner::MainThread
    );
    assert_eq!(
        c.ring_buffer_consumer_owner,
        WasapiOutputThreadOwnedStateOwner::OutputThread
    );
}

/// D: Default contract records adapter slot owner as OutputThread.
#[test]
fn default_contract_records_adapter_slot_owner() {
    let c = default_wasapi_output_thread_owned_state_contract();
    assert_eq!(
        c.adapter_slot_owner,
        WasapiOutputThreadOwnedStateOwner::OutputThread
    );
}

/// E: Default contract keeps command channel unallocated.
#[test]
fn default_contract_keeps_command_channel_unallocated() {
    let c = default_wasapi_output_thread_owned_state_contract();
    assert_eq!(
        c.command_channel_owner,
        WasapiOutputThreadOwnedStateOwner::NotAllocated
    );
}

/// F: Validation accepts default contract and returns report.
#[test]
fn validation_accepts_default_contract() {
    let c = default_wasapi_output_thread_owned_state_contract();
    let report = validate_wasapi_output_thread_owned_state_contract(c).unwrap();
    assert!(report.valid_for_current_scaffold);
    assert_eq!(report.contract, c);
}

/// G: Validation rejects MainThread context owner.
#[test]
fn validation_rejects_main_thread_context_owner() {
    let mut c = default_wasapi_output_thread_owned_state_contract();
    c.wasapi_context_owner = WasapiOutputThreadOwnedStateOwner::MainThread;
    assert_eq!(
        validate_wasapi_output_thread_owned_state_contract(c),
        Err(WasapiOutputThreadOwnedStateError::WasapiContextMustBeOutputThreadOwned)
    );
}

/// H: Validation rejects OutputThread producer owner.
#[test]
fn validation_rejects_output_thread_producer_owner() {
    let mut c = default_wasapi_output_thread_owned_state_contract();
    c.ring_buffer_producer_owner = WasapiOutputThreadOwnedStateOwner::OutputThread;
    assert_eq!(
        validate_wasapi_output_thread_owned_state_contract(c),
        Err(WasapiOutputThreadOwnedStateError::RingBufferProducerMustBeMainThreadOwned)
    );
}

/// I: Validation rejects MainThread consumer owner.
#[test]
fn validation_rejects_main_thread_consumer_owner() {
    let mut c = default_wasapi_output_thread_owned_state_contract();
    c.ring_buffer_consumer_owner = WasapiOutputThreadOwnedStateOwner::MainThread;
    assert_eq!(
        validate_wasapi_output_thread_owned_state_contract(c),
        Err(WasapiOutputThreadOwnedStateError::RingBufferConsumerMustBeOutputThreadOwned)
    );
}

/// J: Validation rejects MainThread adapter slot owner.
#[test]
fn validation_rejects_main_thread_adapter_slot_owner() {
    let mut c = default_wasapi_output_thread_owned_state_contract();
    c.adapter_slot_owner = WasapiOutputThreadOwnedStateOwner::MainThread;
    assert_eq!(
        validate_wasapi_output_thread_owned_state_contract(c),
        Err(WasapiOutputThreadOwnedStateError::AdapterSlotMustBeOutputThreadOwned)
    );
}

/// K: Validation rejects allocated channel.
#[test]
fn validation_rejects_allocated_channel() {
    let mut c = default_wasapi_output_thread_owned_state_contract();
    c.command_channel_owner = WasapiOutputThreadOwnedStateOwner::MainThread;
    assert_eq!(
        validate_wasapi_output_thread_owned_state_contract(c),
        Err(WasapiOutputThreadOwnedStateError::CommandChannelMustNotBeAllocatedYet)
    );
}

/// L: Validation rejects thread_spawned = true.
#[test]
fn validation_rejects_thread_spawned() {
    let mut c = default_wasapi_output_thread_owned_state_contract();
    c.thread_spawned = true;
    assert_eq!(
        validate_wasapi_output_thread_owned_state_contract(c),
        Err(WasapiOutputThreadOwnedStateError::ThreadMustNotBeSpawnedYet)
    );
}

/// M: Validation rejects channel_created = true.
#[test]
fn validation_rejects_channel_created() {
    let mut c = default_wasapi_output_thread_owned_state_contract();
    c.channel_created = true;
    assert_eq!(
        validate_wasapi_output_thread_owned_state_contract(c),
        Err(WasapiOutputThreadOwnedStateError::ChannelMustNotBeCreatedYet)
    );
}

/// N: Validation rejects audio_client_start_allowed = true.
#[test]
fn validation_rejects_start_allowed() {
    let mut c = default_wasapi_output_thread_owned_state_contract();
    c.audio_client_start_allowed = true;
    assert_eq!(
        validate_wasapi_output_thread_owned_state_contract(c),
        Err(WasapiOutputThreadOwnedStateError::AudioClientStartMustNotBeAllowedYet)
    );
}

/// O: Validation rejects native_pipeline_connected = true.
#[test]
fn validation_rejects_native_pipeline_connected() {
    let mut c = default_wasapi_output_thread_owned_state_contract();
    c.native_pipeline_connected = true;
    assert_eq!(
        validate_wasapi_output_thread_owned_state_contract(c),
        Err(WasapiOutputThreadOwnedStateError::NativePipelineMustNotBeConnectedYet)
    );
}

/// P: Validation rejects playback_capabilities_enabled = true.
#[test]
fn validation_rejects_playback_capabilities_enabled() {
    let mut c = default_wasapi_output_thread_owned_state_contract();
    c.playback_capabilities_enabled = true;
    assert_eq!(
        validate_wasapi_output_thread_owned_state_contract(c),
        Err(WasapiOutputThreadOwnedStateError::PlaybackCapabilitiesMustNotBeEnabledYet)
    );
}
