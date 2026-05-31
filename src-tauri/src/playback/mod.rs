pub mod activity_entry;
pub mod activity_log;
pub mod activity_recorder;
pub mod activity_snapshot;
pub mod backend_status;
pub mod backends;
pub mod capabilities;
pub mod clock;
pub mod commands;
pub mod core_profile;
pub mod decoder;
pub mod decoder_request;
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
pub mod metadata;
pub mod native_output;
pub mod output;
pub mod output_policy;
pub mod path;
pub mod queue;
pub mod queue_policy;
pub mod state;
pub mod timeline;
pub mod types;
pub mod volume;
pub mod windows_audio;

#[cfg(test)]
mod activity_log_tests;

#[cfg(test)]
mod activity_log_state_tests;

#[cfg(test)]
mod activity_recorder_tests;

#[cfg(test)]
mod lifecycle_activity_log_tests;

#[cfg(test)]
mod lifecycle_event_tests;

#[cfg(test)]
mod lifecycle_recorder_tests;
