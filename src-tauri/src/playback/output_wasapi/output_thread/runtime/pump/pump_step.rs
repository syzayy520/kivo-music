//! Runtime pump step.
//!
//! Single-step runtime pump that chains thread loop, driver, and sink dispatch
//! into one pure-memory operation.

use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;
use crate::playback::output_wasapi::output_thread::runtime::pump::pump_context::PumpContext;
use crate::playback::output_wasapi::output_thread::runtime::pump::pump_outcome::PumpOutcome;
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::driver_sink_dispatch::dispatch_driver_result;
use crate::playback::output_wasapi::output_thread::runtime::thread_loop::loop_result::LoopResult;
use crate::playback::output_wasapi::output_thread::runtime::thread_loop::loop_step::execute_loop_step;
use crate::playback::output_wasapi::output_thread::sink_boundary::consumer::consumer_contract::SinkConsumer;

/// Execute a single runtime pump tick.
///
/// Chains: LoopStep → DriverResult → SinkRequest → SinkConsumer → PumpOutcome.
/// Pure memory — no I/O, no thread spawn, no device access.
pub fn execute_pump_tick<C: SinkConsumer>(ctx: &PumpContext, consumer: &mut C) -> PumpOutcome {
    let loop_outcome =
        execute_loop_step(&ctx.state, ctx.command.clone(), ctx.config.max_idle_steps);

    let driver_result = match loop_outcome.result {
        LoopResult::Continue => DriverResult::Continue,
        LoopResult::Stop => DriverResult::Stop,
        LoopResult::Error(_) => DriverResult::Error,
    };

    let dispatch_outcome = if driver_result == DriverResult::Continue {
        dispatch_driver_result(
            consumer,
            driver_result,
            ctx.config.frame_count,
            ctx.config.sample_rate,
            ctx.config.channel_count,
        )
        .ok()
    } else {
        None
    };

    PumpOutcome::from_loop_result(
        loop_outcome.result,
        loop_outcome.state,
        driver_result,
        dispatch_outcome,
    )
}
