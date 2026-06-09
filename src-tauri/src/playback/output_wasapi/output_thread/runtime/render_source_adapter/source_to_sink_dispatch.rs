//! Source to sink dispatch.
//!
//! Full dispatch chain: RenderSourceRequest -> RenderSource -> RenderSourceResult -> SinkRequest -> SinkConsumer -> DispatchOutcome.
//! No threads, no IO, no WASAPI.

use crate::playback::output_wasapi::output_thread::runtime::render_source_adapter::{
    AdapterContext, AdapterError, AdapterOutcome,
};
use crate::playback::output_wasapi::output_thread::runtime::render_source_mapping::source_to_sink_mapper;
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::{
    consumer_invoker, DispatchContext, DispatchOutcome,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::consumer::consumer_contract::SinkConsumer;
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSource, RenderSourceRequest, RenderSourceResult,
};

/// Result of a source-to-sink dispatch chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceToSinkOutcome {
    /// Outcome from the source adapter.
    pub source_outcome: AdapterOutcome,
    /// Outcome from the sink dispatch.
    pub sink_outcome: DispatchOutcome,
}

/// Execute the full source-to-sink dispatch chain.
///
/// Maps RenderSourceRequest -> RenderSource -> RenderSourceResult -> SinkRequest -> SinkConsumer -> DispatchOutcome.
/// Pure memory — no threads, no IO, no WASAPI.
pub fn dispatch_source_to_sink<S: RenderSource, C: SinkConsumer>(
    source: &mut S,
    consumer: &mut C,
    request: &RenderSourceRequest,
    sample_rate: u32,
    channel_count: u16,
) -> Result<SourceToSinkOutcome, AdapterError> {
    let adapter_context = AdapterContext {
        driver_result: crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult::Idle,
        frame_count: 0,
        sample_rate,
        channel_count,
    };

    // Step 1: Invoke source with request.
    let source_outcome = crate::playback::output_wasapi::output_thread::runtime::render_source_adapter::source_invoker::invoke_render_source(
        source,
        request,
        &adapter_context,
    )?;

    // Step 2: Map source outcome to sink request.
    let source_result = map_adapter_outcome_to_source_result(&source_outcome);
    let sink_request = source_to_sink_mapper::map_source_result_to_sink_request(
        &source_result,
        sample_rate,
        channel_count,
    );

    // Step 3: Invoke sink consumer.
    let dispatch_context = DispatchContext {
        driver_result: crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult::Idle,
        frame_count: 0,
        sample_rate,
        channel_count,
    };
    let sink_outcome =
        consumer_invoker::invoke_sink_consumer(consumer, &sink_request, &dispatch_context)
            .map_err(|err| AdapterError::Internal {
                description: format!("sink dispatch failed: {}", err),
            })?;

    Ok(SourceToSinkOutcome {
        source_outcome,
        sink_outcome,
    })
}

/// Map an AdapterOutcome back to a RenderSourceResult for chaining.
fn map_adapter_outcome_to_source_result(outcome: &AdapterOutcome) -> RenderSourceResult {
    match outcome {
        AdapterOutcome::Packet {
            frames_provided,
            bytes_read,
        } => RenderSourceResult::Packet {
            frames_provided: *frames_provided,
            bytes_read: *bytes_read,
        },
        AdapterOutcome::Exhausted => RenderSourceResult::Exhausted,
        AdapterOutcome::Skipped => RenderSourceResult::Skipped,
        AdapterOutcome::Noop => RenderSourceResult::Noop,
        AdapterOutcome::Failed => RenderSourceResult::Noop, // Map failed to noop for now.
    }
}
