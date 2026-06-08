use crate::playback::decoder_runtime_state::DecoderRuntimePhase;
use crate::playback::errors::PlaybackResult;
use crate::playback::native_pipeline::NativePipeline;

/// Result of a single `pump_once` step in the runtime loop.
///
/// This enum does NOT:
/// - represent real playback state
/// - represent audio device state
/// - track wall-clock time
/// - send events
/// - know about UI/manager/commands
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::playback) enum NativePipelineLoopStep {
    /// One frame was drained from buffer to output sink.
    /// Clock position was updated to the drained frame's position_ms.
    Drained,
    /// Decoder reached end-of-stream AND buffer is empty.
    /// This is not an error — it means the track is fully consumed.
    EndOfStream,
}

impl NativePipeline {
    /// Decode one frame from the decoder into the pipeline buffer.
    ///
    /// Wraps `schedule_decode_step()`. Does NOT drain, does NOT open output,
    /// does NOT send events, does NOT change public engine state.
    #[allow(dead_code)]
    pub(in crate::playback) fn decode_once_to_buffer(&mut self) -> PlaybackResult<()> {
        self.schedule_decode_step()
    }

    /// Drain one frame from the pipeline buffer to the output sink.
    ///
    /// Wraps `drain_next_frame_to_output()`. Does NOT decode,
    /// does NOT open output device, does NOT send events.
    /// Clock position is updated on success.
    #[allow(dead_code)]
    pub(in crate::playback) fn drain_once_to_output(&mut self) -> PlaybackResult<()> {
        self.drain_next_frame_to_output()
    }

    /// Execute one pump cycle: try to advance the decode→buffer→drain pipeline.
    ///
    /// Logic:
    /// 1. If buffer has frames → drain one → return `Drained`
    /// 2. If buffer empty + decoder is `Draining` (EOS) → return `EndOfStream`
    /// 3. If buffer empty + decoder is open → decode one frame
    ///    - If buffer now has frame → drain one → return `Drained`
    ///    - If decoder transitioned to Draining → return `EndOfStream`
    /// 4. If decoder not open → return `Backend` error
    ///
    /// Does NOT:
    /// - clone AudioOutputFrame unnecessarily
    /// - swallow errors
    /// - update public engine state
    /// - send events
    /// - open threads
    #[allow(dead_code)]
    pub(in crate::playback) fn pump_once(&mut self) -> PlaybackResult<NativePipelineLoopStep> {
        // 1. Buffer has frames → drain one
        if self.buffered_frame_count() > 0 {
            self.drain_next_frame_to_output()?;
            return Ok(NativePipelineLoopStep::Drained);
        }

        // 2. Buffer empty + decoder already Draining (EOS)
        if self.state.decoder_state.phase == DecoderRuntimePhase::Draining {
            return Ok(NativePipelineLoopStep::EndOfStream);
        }

        // 3. Buffer empty + decoder should be open → try decode
        self.schedule_decode_step()?;

        // Check if decode produced a frame
        if self.buffered_frame_count() > 0 {
            self.drain_next_frame_to_output()?;
            return Ok(NativePipelineLoopStep::Drained);
        }

        // Decode returned Ok but no frame → decoder entered Draining (EOS)
        Ok(NativePipelineLoopStep::EndOfStream)
    }

    /// Run `pump_once` in a loop up to `max_steps` times.
    ///
    /// Stops when:
    /// - `EndOfStream` is reached (returns `Ok(())`)
    /// - `max_steps` iterations completed (returns `Ok(())`)
    /// - An error occurs (returns `Err(...)`)
    ///
    /// `max_steps = 0` does nothing and returns `Ok(())`.
    ///
    /// Does NOT:
    /// - run infinitely
    /// - run in background
    /// - use async
    /// - open threads
    #[allow(dead_code)]
    pub(in crate::playback) fn run_until_blocked(
        &mut self,
        max_steps: usize,
    ) -> PlaybackResult<()> {
        for _ in 0..max_steps {
            match self.pump_once()? {
                NativePipelineLoopStep::Drained => {}
                NativePipelineLoopStep::EndOfStream => break,
            }
        }
        Ok(())
    }
}
