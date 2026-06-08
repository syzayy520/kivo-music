//! Driver sink dispatch.
//!
//! Full dispatch chain: DriverResult -> SinkRequest -> SinkConsumer -> DispatchOutcome.
//! No WASAPI, no IO, no actual rendering.

use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::consumer_invoker;
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::{
    DispatchContext, DispatchError, DispatchOutcome,
};
use crate::playback::output_wasapi::output_thread::runtime::sink_mapping::driver_result_mapper;
use crate::playback::output_wasapi::output_thread::sink_boundary::consumer::consumer_contract::SinkConsumer;

/// Execute the full dispatch chain for a driver result.
///
/// Maps DriverResult → SinkRequest → invokes SinkConsumer → returns DispatchOutcome.
/// Pure memory — no threads, no IO, no WASAPI.
pub fn dispatch_driver_result<C: SinkConsumer>(
    consumer: &mut C,
    driver_result: DriverResult,
    frame_count: u64,
    sample_rate: u32,
    channel_count: u16,
) -> Result<DispatchOutcome, DispatchError> {
    let request = driver_result_mapper::map_driver_result_to_sink_request(
        driver_result,
        frame_count,
        sample_rate,
        channel_count,
    );

    let context = DispatchContext {
        driver_result,
        frame_count,
        sample_rate,
        channel_count,
    };

    consumer_invoker::invoke_sink_consumer(consumer, &request, &context)
}
