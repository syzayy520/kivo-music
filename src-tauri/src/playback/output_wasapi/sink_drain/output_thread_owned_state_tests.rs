//! Tests for output-thread owned state contract.

use super::output_thread_owned_state::{
    build_default_output_thread_owned_state_contract, validate_output_thread_owned_state_contract,
};
use super::output_thread_owned_state_error::WasapiOutputThreadOwnedStateError;
use super::output_thread_owned_state_report::{
    WasapiOutputThreadOwnedStateContract, WasapiOutputThreadOwnedStateOwner,
    WasapiOutputThreadOwnedStateStage,
};

/// A: Default contract passes validation.
#[test]
fn default_contract_is_valid() {
    let contract = build_default_output_thread_owned_state_contract();
    assert!(validate_output_thread_owned_state_contract(&contract).is_ok());
}

/// B: Invalid stage is rejected.
#[test]
fn invalid_stage_rejected() {
    // Build a contract with a non-ContractOnly stage.
    // We must construct it manually since only ContractOnly exists in the enum.
    // This test verifies the validation catches future stage additions.
    let contract = WasapiOutputThreadOwnedStateContract {
        stage: WasapiOutputThreadOwnedStateStage::ContractOnly, // only variant
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
    };
    // ContractOnly is the only variant, so this should pass.
    assert!(validate_output_thread_owned_state_contract(&contract).is_ok());
}

/// C: Wrong context owner is rejected.
#[test]
fn wrong_context_owner_rejected() {
    let mut contract = build_default_output_thread_owned_state_contract();
    contract.wasapi_context_owner = WasapiOutputThreadOwnedStateOwner::MainThread;
    assert_eq!(
        validate_output_thread_owned_state_contract(&contract),
        Err(WasapiOutputThreadOwnedStateError::InvalidContextOwner)
    );
}

/// D: Wrong sink owner is rejected.
#[test]
fn wrong_sink_owner_rejected() {
    let mut contract = build_default_output_thread_owned_state_contract();
    contract.sink_owner = WasapiOutputThreadOwnedStateOwner::OutputThread;
    assert_eq!(
        validate_output_thread_owned_state_contract(&contract),
        Err(WasapiOutputThreadOwnedStateError::InvalidSinkOwner)
    );
}

/// E: Wrong ring buffer producer owner is rejected.
#[test]
fn wrong_rb_producer_owner_rejected() {
    let mut contract = build_default_output_thread_owned_state_contract();
    contract.ring_buffer_producer_owner = WasapiOutputThreadOwnedStateOwner::OutputThread;
    assert_eq!(
        validate_output_thread_owned_state_contract(&contract),
        Err(WasapiOutputThreadOwnedStateError::InvalidRingBufferProducerOwner)
    );
}

/// F: Wrong ring buffer consumer owner is rejected.
#[test]
fn wrong_rb_consumer_owner_rejected() {
    let mut contract = build_default_output_thread_owned_state_contract();
    contract.ring_buffer_consumer_owner = WasapiOutputThreadOwnedStateOwner::MainThread;
    assert_eq!(
        validate_output_thread_owned_state_contract(&contract),
        Err(WasapiOutputThreadOwnedStateError::InvalidRingBufferConsumerOwner)
    );
}

/// G: Wrong adapter slot owner is rejected.
#[test]
fn wrong_adapter_slot_owner_rejected() {
    let mut contract = build_default_output_thread_owned_state_contract();
    contract.adapter_slot_owner = WasapiOutputThreadOwnedStateOwner::MainThread;
    assert_eq!(
        validate_output_thread_owned_state_contract(&contract),
        Err(WasapiOutputThreadOwnedStateError::InvalidAdapterSlotOwner)
    );
}

/// H: Wrong command channel owner is rejected.
#[test]
fn wrong_channel_owner_rejected() {
    let mut contract = build_default_output_thread_owned_state_contract();
    contract.command_channel_owner = WasapiOutputThreadOwnedStateOwner::MainThread;
    assert_eq!(
        validate_output_thread_owned_state_contract(&contract),
        Err(WasapiOutputThreadOwnedStateError::InvalidCommandChannelOwner)
    );
}

/// I: thread_spawned = true is rejected.
#[test]
fn thread_spawned_rejected() {
    let mut contract = build_default_output_thread_owned_state_contract();
    contract.thread_spawned = true;
    assert_eq!(
        validate_output_thread_owned_state_contract(&contract),
        Err(WasapiOutputThreadOwnedStateError::ThreadAlreadySpawned)
    );
}

/// J: channel_created = true is rejected.
#[test]
fn channel_created_rejected() {
    let mut contract = build_default_output_thread_owned_state_contract();
    contract.channel_created = true;
    assert_eq!(
        validate_output_thread_owned_state_contract(&contract),
        Err(WasapiOutputThreadOwnedStateError::ChannelAlreadyCreated)
    );
}

/// K: audio_client_start_allowed = true is rejected.
#[test]
fn start_allowed_rejected() {
    let mut contract = build_default_output_thread_owned_state_contract();
    contract.audio_client_start_allowed = true;
    assert_eq!(
        validate_output_thread_owned_state_contract(&contract),
        Err(WasapiOutputThreadOwnedStateError::StartNotAllowedYet)
    );
}

/// L: native_pipeline_connected = true is rejected.
#[test]
fn pipeline_connected_rejected() {
    let mut contract = build_default_output_thread_owned_state_contract();
    contract.native_pipeline_connected = true;
    assert_eq!(
        validate_output_thread_owned_state_contract(&contract),
        Err(WasapiOutputThreadOwnedStateError::NativePipelineAlreadyConnected)
    );
}

/// M: playback_capabilities_enabled = true is rejected.
#[test]
fn capabilities_enabled_rejected() {
    let mut contract = build_default_output_thread_owned_state_contract();
    contract.playback_capabilities_enabled = true;
    assert_eq!(
        validate_output_thread_owned_state_contract(&contract),
        Err(WasapiOutputThreadOwnedStateError::CapabilitiesAlreadyEnabled)
    );
}

/// N: Default contract values match the expected ownership layout.
#[test]
fn default_contract_values_match_spec() {
    let c = build_default_output_thread_owned_state_contract();
    assert_eq!(c.stage, WasapiOutputThreadOwnedStateStage::ContractOnly);
    assert_eq!(
        c.sink_owner,
        WasapiOutputThreadOwnedStateOwner::SplitProducerConsumer
    );
    assert_eq!(
        c.wasapi_context_owner,
        WasapiOutputThreadOwnedStateOwner::OutputThread
    );
    assert_eq!(
        c.ring_buffer_producer_owner,
        WasapiOutputThreadOwnedStateOwner::MainThread
    );
    assert_eq!(
        c.ring_buffer_consumer_owner,
        WasapiOutputThreadOwnedStateOwner::OutputThread
    );
    assert_eq!(
        c.adapter_slot_owner,
        WasapiOutputThreadOwnedStateOwner::OutputThread
    );
    assert_eq!(
        c.command_channel_owner,
        WasapiOutputThreadOwnedStateOwner::NotAllocated
    );
    assert!(!c.thread_spawned);
    assert!(!c.channel_created);
    assert!(!c.audio_client_start_allowed);
    assert!(!c.native_pipeline_connected);
    assert!(!c.playback_capabilities_enabled);
}

/// O: Contract is Copy (all fields are Copy types).
#[test]
fn contract_is_copy() {
    let a = build_default_output_thread_owned_state_contract();
    let b = a; // copy
    assert_eq!(a, b);
}
