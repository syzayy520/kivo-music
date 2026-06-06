pub mod activity_entry;
pub mod activity_log;
pub mod activity_recorder;
pub mod activity_snapshot;
pub mod audio_bridge;
pub mod audio_buffer;
pub mod audio_route;
pub mod audio_route_coordinator;
pub mod audio_route_integration;
pub mod audio_route_pipeline_tap;
pub mod backend_status;
pub mod backends;
pub mod capabilities;
pub mod clock;
pub mod command_activity;
pub mod commands;
pub mod core_profile;
pub mod decoder;
pub mod decoder_request;
pub mod decoder_runtime_state;
pub mod decoder_session;
pub mod decoders;
pub mod engine;
pub mod errors;
pub mod events;
pub mod lifecycle;
pub mod lifecycle_activity_log;
pub mod lifecycle_activity_snapshot;
pub mod lifecycle_commands;
pub mod lifecycle_event;
pub mod lifecycle_recorder;
pub mod lyrics_clock;
pub mod manager;
pub mod manager_queue;
pub mod metadata;
pub mod native_output;
pub mod native_pipeline;
pub mod native_pipeline_route_tap_diagnostic_policy;
pub mod output;
pub mod output_frame;
pub mod output_policy;
pub mod output_sink;
pub mod output_wasapi;
pub mod path;
pub mod playback_event_bridge;
pub mod playback_event_bus;
pub mod playback_event_dispatcher;
pub mod playback_worker_command;
pub mod playback_worker_state;
pub mod playback_worker_transition;
pub mod queue;
pub mod queue_policy;
pub mod state;
pub mod timeline;
pub mod types;
pub mod volume;
pub mod windows_audio;

mod native_null_output;
mod native_pipeline_buffer;
mod native_pipeline_clock;
mod native_pipeline_decoder;
mod native_pipeline_drain;
mod native_pipeline_loop;
mod native_pipeline_output;
mod native_pipeline_progress;
mod native_pipeline_route_tap;
mod native_pipeline_runtime;
mod native_pipeline_state;
mod native_pipeline_worker;

#[cfg(test)]
mod activity_log_tests;

#[cfg(test)]
mod activity_log_state_tests;

#[cfg(test)]
mod activity_recorder_tests;

#[cfg(test)]
mod audio_buffer_tests;

#[cfg(test)]
mod audio_bridge_tests;

#[cfg(test)]
mod audio_route_tests;

#[cfg(test)]
mod audio_route_coordinator_tests;

#[cfg(test)]
mod audio_route_integration_tests;

#[cfg(test)]
mod audio_route_pipeline_tap_tests;

#[cfg(test)]
mod command_activity_tests;

#[cfg(test)]
mod decoder_request_tests;

#[cfg(test)]
mod decoder_runtime_state_tests;

#[cfg(test)]
mod decoder_session_tests;

#[cfg(test)]
mod lifecycle_activity_log_tests;

#[cfg(test)]
mod lifecycle_event_tests;

#[cfg(test)]
mod lifecycle_recorder_tests;

#[cfg(test)]
mod manager_queue_tests;

#[cfg(test)]
mod native_pipeline_buffer_tests;

#[cfg(test)]
mod native_pipeline_clock_tests;

#[cfg(test)]
mod native_pipeline_drain_tests;

#[cfg(test)]
mod native_pipeline_loop_tests;

#[cfg(test)]
mod native_pipeline_progress_tests;

#[cfg(test)]
mod native_pipeline_route_tap_tests;

#[cfg(test)]
mod native_pipeline_route_tap_diagnostic_policy_tests;

#[cfg(test)]
mod native_pipeline_tests;

#[cfg(test)]
mod native_pipeline_worker_route_tests;

#[cfg(test)]
mod native_pipeline_worker_route_edge_tests;

#[cfg(test)]
mod native_pipeline_worker_runtime_tests;

#[cfg(test)]
mod output_frame_tests;

#[cfg(test)]
mod output_sink_contract_tests;

#[cfg(test)]
mod output_sink_tests;

#[cfg(test)]
mod output_wasapi_root_tests;

#[cfg(test)]
mod queue_policy_tests;

#[cfg(test)]
mod playback_worker_command_tests;

#[cfg(test)]
mod playback_event_dispatcher_tests;

#[cfg(test)]
mod playback_event_bus_tests;

#[cfg(test)]
mod playback_event_bridge_tests;

#[cfg(test)]
mod playback_worker_state_tests;

#[cfg(test)]
mod playback_worker_transition_tests;
