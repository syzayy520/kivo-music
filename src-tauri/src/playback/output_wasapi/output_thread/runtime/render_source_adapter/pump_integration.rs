//! Pump integration seam.
//!
//! Integrates the render source adapter into the runtime pump.
//! Provides a function that executes a pump tick with a source.
//! No threads, no IO, no WASAPI.

use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;
use crate::playback::output_wasapi::output_thread::runtime::pump::pump_context::PumpContext;
use crate::playback::output_wasapi::output_thread::runtime::pump::pump_outcome::PumpOutcome;
use crate::playback::output_wasapi::output_thread::runtime::render_source_adapter::{
    source_to_sink_dispatch, AdapterError, AdapterOutcome,
};
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::DispatchOutcome;
use crate::playback::output_wasapi::output_thread::sink_boundary::consumer::consumer_contract::SinkConsumer;
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSource, RenderSourceRequest,
};

/// Result of a pump tick with source integration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcePumpOutcome {
    /// The base pump outcome.
    pub pump_outcome: PumpOutcome,
    /// The source adapter outcome, if source was involved.
    pub source_outcome: Option<AdapterOutcome>,
    /// The sink dispatch outcome, if sink was involved.
    pub sink_outcome: Option<DispatchOutcome>,
}

/// Execute a single runtime pump tick with source integration.
///
/// Chains: RenderSourceRequest -> RenderSource -> RenderSourceResult -> SinkRequest -> SinkConsumer -> PumpOutcome.
/// Includes source adapter metadata in the outcome.
/// Pure memory — no I/O, no thread spawn, no device access.
pub fn execute_pump_tick_with_source<S: RenderSource, C: SinkConsumer>(
    ctx: &PumpContext,
    source: &mut S,
    consumer: &mut C,
) -> Result<SourcePumpOutcome, AdapterError> {
    // Step 1: Determine the source request based on pump context.
    let source_request = determine_source_request(ctx);

    // Step 2: Execute the source-to-sink dispatch chain.
    let dispatch_result = source_to_sink_dispatch::dispatch_source_to_sink(
        source,
        consumer,
        &source_request,
        ctx.config.sample_rate,
        ctx.config.channel_count,
    )?;

    // Step 3: Build the pump outcome based on dispatch result.
    let pump_outcome = build_pump_outcome_from_dispatch(ctx, &dispatch_result.sink_outcome);

    Ok(SourcePumpOutcome {
        pump_outcome,
        source_outcome: Some(dispatch_result.source_outcome),
        sink_outcome: Some(dispatch_result.sink_outcome),
    })
}

/// Determine the source request based on pump context.
fn determine_source_request(ctx: &PumpContext) -> RenderSourceRequest {
    // If the pump has a command, map it to a source request.
    // For now, always request a packet if the pump is active.
    if ctx.command.is_some() {
        RenderSourceRequest::ReadPacket {
            frame_count: ctx.config.frame_count,
            sample_rate: ctx.config.sample_rate,
            channel_count: ctx.config.channel_count,
        }
    } else {
        RenderSourceRequest::Noop
    }
}

/// Build a pump outcome from a dispatch outcome.
fn build_pump_outcome_from_dispatch(
    ctx: &PumpContext,
    dispatch_outcome: &DispatchOutcome,
) -> PumpOutcome {
    // For now, create a simple Continue outcome.
    // In a real implementation, this would consider the loop state.
    PumpOutcome::Continue {
        state: ctx.state.clone(),
        driver_result: DriverResult::Continue,
        dispatch_outcome: Some(dispatch_outcome.clone()),
        events: Vec::new(),
        consumer_snapshot: None,
    }
}
