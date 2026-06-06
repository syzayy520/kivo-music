use super::super::render_loop::RenderSilenceLoopOutcome;
use super::super::render_padding_loop::RenderPaddingLoopOutcome;
use super::super::render_ring_buffer_boundary::RenderRingBufferBoundaryOutcome;
use super::super::thread_error::RealOutputThreadSkeletonError;
use crate::playback::output_wasapi::worker_loop::runner::OutputThreadWorkerLoopRunResult;

pub(crate) struct PostStartStageResults {
    pub render_loop_result: Result<RenderSilenceLoopOutcome, RealOutputThreadSkeletonError>,
    pub render_padding_loop_result: Result<RenderPaddingLoopOutcome, RealOutputThreadSkeletonError>,
    pub render_ring_buffer_boundary_result:
        Result<RenderRingBufferBoundaryOutcome, RealOutputThreadSkeletonError>,
}

impl PostStartStageResults {
    pub(crate) fn can_run_worker_loop(&self) -> bool {
        self.render_loop_result.is_ok()
            && self.render_padding_loop_result.is_ok()
            && self.render_ring_buffer_boundary_result.is_ok()
    }
}

#[derive(Debug)]
pub(crate) struct ResolvedThreadStageResults {
    pub render_loop_outcome: RenderSilenceLoopOutcome,
    pub render_padding_loop_outcome: RenderPaddingLoopOutcome,
    pub render_ring_buffer_boundary_outcome: RenderRingBufferBoundaryOutcome,
    pub loop_result: OutputThreadWorkerLoopRunResult,
}

pub(crate) fn resolve_thread_stage_results(
    stop_result: Result<(), String>,
    stage_results: PostStartStageResults,
    worker_loop_result: Option<OutputThreadWorkerLoopRunResult>,
) -> Result<ResolvedThreadStageResults, RealOutputThreadSkeletonError> {
    let PostStartStageResults {
        render_loop_result,
        render_padding_loop_result,
        render_ring_buffer_boundary_result,
    } = stage_results;

    stop_result.map_err(RealOutputThreadSkeletonError::AudioClientStopFailed)?;
    let render_loop_outcome = render_loop_result?;
    let render_padding_loop_outcome = render_padding_loop_result?;
    let render_ring_buffer_boundary_outcome = render_ring_buffer_boundary_result?;
    let loop_result = match worker_loop_result {
        Some(loop_result) => loop_result,
        None => return Err(RealOutputThreadSkeletonError::WorkerDidNotReport),
    };

    Ok(ResolvedThreadStageResults {
        render_loop_outcome,
        render_padding_loop_outcome,
        render_ring_buffer_boundary_outcome,
        loop_result,
    })
}
