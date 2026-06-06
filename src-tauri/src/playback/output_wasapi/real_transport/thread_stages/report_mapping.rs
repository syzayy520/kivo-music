use super::super::render_once::RenderSilenceOnceOutcome;
use super::super::thread_report::RealOutputThreadReport;
use super::outcomes::ResolvedThreadStageResults;

pub(crate) struct RealOutputThreadLifecycleReportFields {
    pub thread_started: bool,
    pub owned_state_validated: bool,
    pub panicked: bool,
    pub com_initialized: bool,
    pub com_uninitialized: bool,
    pub wasapi_context_open_requested: bool,
    pub wasapi_context_opened: bool,
    pub wasapi_context_closed: bool,
    pub audio_client_start_requested: bool,
    pub audio_client_started: bool,
    pub audio_client_stop_requested: bool,
    pub audio_client_stopped: bool,
}

pub(crate) fn build_real_output_thread_report(
    lifecycle: RealOutputThreadLifecycleReportFields,
    render_once: RenderSilenceOnceOutcome,
    stages: ResolvedThreadStageResults,
) -> RealOutputThreadReport {
    let worker_report = stages.loop_result.report;
    let render_loop = stages.render_loop_outcome;
    let padding_loop = stages.render_padding_loop_outcome;
    let ring_buffer_boundary = stages.render_ring_buffer_boundary_outcome;

    RealOutputThreadReport {
        thread_started: lifecycle.thread_started,
        owned_state_validated: lifecycle.owned_state_validated,
        shutdown_received: worker_report.stopped_by_close_transport,
        exited_cleanly: worker_report.final_state.is_terminal()
            || !stages.loop_result.stopped_early,
        commands_processed: worker_report.commands_handled,
        loop_result: Some(stages.loop_result),
        panicked: lifecycle.panicked,
        com_initialized: lifecycle.com_initialized,
        com_uninitialized: lifecycle.com_uninitialized,
        wasapi_context_open_requested: lifecycle.wasapi_context_open_requested,
        wasapi_context_opened: lifecycle.wasapi_context_opened,
        wasapi_context_closed: lifecycle.wasapi_context_closed,
        audio_client_start_requested: lifecycle.audio_client_start_requested,
        audio_client_started: lifecycle.audio_client_started,
        audio_client_stop_requested: lifecycle.audio_client_stop_requested,
        audio_client_stopped: lifecycle.audio_client_stopped,
        render_silence_once_requested: render_once.requested,
        render_silence_once_written: render_once.written,
        render_silence_once_frames_requested: render_once.frames_requested,
        render_silence_once_frames_written: render_once.frames_written,
        render_silence_once_used_silent_flag: render_once.used_silent_flag,
        render_silence_loop_requested: render_loop.requested,
        render_silence_loop_started: render_loop.started,
        render_silence_loop_completed: render_loop.completed,
        render_silence_loop_iterations_requested: render_loop.iterations_requested,
        render_silence_loop_iterations_completed: render_loop.iterations_completed,
        render_silence_loop_frames_per_write: render_loop.frames_per_write,
        render_silence_loop_frames_written_total: render_loop.frames_written_total,
        render_silence_loop_used_silent_flag: render_loop.used_silent_flag,
        render_padding_loop_requested: padding_loop.requested,
        render_padding_loop_started: padding_loop.started,
        render_padding_loop_completed: padding_loop.completed,
        render_padding_loop_iterations_requested: padding_loop.iterations_requested,
        render_padding_loop_iterations_completed: padding_loop.iterations_completed,
        render_padding_loop_iterations_skipped_no_available: padding_loop
            .iterations_skipped_no_available,
        render_padding_loop_max_frames_per_write: padding_loop.max_frames_per_write,
        render_padding_loop_frames_written_total: padding_loop.frames_written_total,
        render_padding_loop_last_capacity: padding_loop.last_capacity,
        render_padding_loop_last_padding: padding_loop.last_padding,
        render_padding_loop_last_available: padding_loop.last_available,
        render_padding_loop_used_silent_flag: padding_loop.used_silent_flag,
        render_ring_buffer_boundary_requested: ring_buffer_boundary.requested,
        render_ring_buffer_boundary_started: ring_buffer_boundary.started,
        render_ring_buffer_boundary_completed: ring_buffer_boundary.completed,
        render_ring_buffer_boundary_skipped_after_prior_failure: ring_buffer_boundary
            .skipped_after_prior_failure,
        render_ring_buffer_boundary_iterations_requested: ring_buffer_boundary.iterations_requested,
        render_ring_buffer_boundary_iterations_completed: ring_buffer_boundary.iterations_completed,
        render_ring_buffer_boundary_write_attempts: ring_buffer_boundary.write_attempts,
        render_ring_buffer_boundary_frames_written_total: ring_buffer_boundary.frames_written_total,
        render_ring_buffer_boundary_frames_committed_total: ring_buffer_boundary
            .frames_committed_total,
        render_ring_buffer_boundary_source_seeded_frames: ring_buffer_boundary
            .synthetic_zero_seed_frames,
        render_ring_buffer_boundary_source_remaining_frames: ring_buffer_boundary
            .source_remaining_frames,
        render_ring_buffer_boundary_used_silent_flag: ring_buffer_boundary.used_silent_flag,
    }
}
