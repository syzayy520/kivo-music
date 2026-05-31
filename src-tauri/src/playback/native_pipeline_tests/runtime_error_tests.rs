use super::*;

#[test]
fn runtime_operations_are_typed_unsupported() {
    let mut pipeline = NativePipeline::new();

    assert_unsupported(pipeline.start(), "start");
    assert_unsupported(pipeline.schedule_decode_step(), "schedule_decode_step");
    assert_unsupported(pipeline.submit(), "submit");
    assert_unsupported(
        pipeline.schedule_output_submit_step(),
        "schedule_output_submit_step",
    );
    assert_unsupported(pipeline.shutdown(), "shutdown");
}

#[test]
fn worker_commands_are_typed_unsupported() {
    let mut pipeline = NativePipeline::new();
    let commands = vec![
        (
            PlaybackWorkerCommand::Load {
                track: worker_track(),
            },
            "load",
        ),
        (PlaybackWorkerCommand::Play, "play"),
        (PlaybackWorkerCommand::Pause, "pause"),
        (PlaybackWorkerCommand::Resume, "resume"),
        (PlaybackWorkerCommand::Stop, "stop"),
        (PlaybackWorkerCommand::Seek { position_ms: 1_000 }, "seek"),
        (
            PlaybackWorkerCommand::SetVolume { level: 0.8 },
            "set_volume",
        ),
        (PlaybackWorkerCommand::SetMuted { muted: true }, "set_muted"),
        (PlaybackWorkerCommand::Shutdown, "shutdown"),
    ];

    for (command, operation) in commands {
        assert_worker_unsupported(pipeline.handle_worker_command(&command), operation);
    }
}
