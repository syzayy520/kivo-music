mod activity;
pub mod audio_bridge;
pub mod audio_buffer;
pub mod audio_route;
mod backend;
pub mod backends;
pub mod clock;
pub mod commands;
pub mod decoder;
pub mod decoders;
pub mod engine;
pub mod errors;
pub mod events;
pub mod lifecycle;
pub mod manager;
pub mod metadata;
pub mod native_pipeline;
pub mod output;
pub mod output_flush_contract;
pub mod output_wasapi;
pub mod path;
mod playback_event;
pub(crate) mod production_output_route;
pub mod queue;
pub mod state;
pub mod types;
pub mod wasapi_seek_barrier_contract;
mod worker;

pub use self::activity::{
    command_activity, entry as activity_entry, log as activity_log, recorder as activity_recorder,
    snapshot as activity_snapshot,
};
pub use self::audio_route::{
    coordinator as audio_route_coordinator, integration as audio_route_integration,
    pipeline_tap as audio_route_pipeline_tap,
};
pub use self::backend::{capabilities, core_profile, status as backend_status};
pub use self::clock::lyrics as lyrics_clock;
pub use self::decoder::{
    request as decoder_request, runtime_state as decoder_runtime_state, session as decoder_session,
};
pub use self::lifecycle::{
    lifecycle_activity_log, lifecycle_activity_snapshot, lifecycle_commands, lifecycle_event,
    lifecycle_recorder,
};
pub use self::manager::queue_state as manager_queue;
pub use self::native_pipeline::route_tap_diagnostic_policy as native_pipeline_route_tap_diagnostic_policy;
pub(in crate::playback) use self::native_pipeline::{
    buffer as native_pipeline_buffer, clock as native_pipeline_clock,
    loop_step as native_pipeline_loop, seek_transaction as native_pipeline_seek_transaction,
    state as native_pipeline_state,
};
pub(in crate::playback) use self::output::native_null as native_null_output;
pub(crate) use self::output::submit_error as output_submit_error;
pub use self::output::{
    native as native_output, policy as output_policy, root_frame as output_frame,
    unsupported_sink as output_sink, windows_audio,
};
pub use self::playback_event::{
    playback_event_bridge, playback_event_bus, playback_event_dispatcher,
};
pub use self::queue::policy as queue_policy;
pub use self::state::{timeline, volume};
pub use self::worker::{
    command as playback_worker_command, state as playback_worker_state,
    transition as playback_worker_transition,
};
