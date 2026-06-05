use super::command::*;
use super::super::output_thread_runtime_intent::OutputThreadRuntimeIntent;

#[test]
fn runtime_intent_preserves_intent() {
    let cmd = OutputThreadRealTransportCommand::runtime_intent(OutputThreadRuntimeIntent::Start);
    match cmd {
        OutputThreadRealTransportCommand::RuntimeIntent(i) => assert_eq!(i, OutputThreadRuntimeIntent::Start),
        _ => panic!("expected RuntimeIntent"),
    }
}

#[test]
fn close_transport_is_close() {
    let cmd = OutputThreadRealTransportCommand::close_transport();
    assert!(cmd.is_close());
}

#[test]
fn runtime_intent_is_not_close() {
    let cmd = OutputThreadRealTransportCommand::runtime_intent(OutputThreadRuntimeIntent::Stop);
    assert!(!cmd.is_close());
}

#[test]
fn as_runtime_intent_returns_some_for_runtime_intent() {
    let cmd = OutputThreadRealTransportCommand::runtime_intent(OutputThreadRuntimeIntent::Flush);
    assert_eq!(cmd.as_runtime_intent(), Some(OutputThreadRuntimeIntent::Flush));
}

#[test]
fn as_runtime_intent_returns_none_for_close() {
    let cmd = OutputThreadRealTransportCommand::close_transport();
    assert_eq!(cmd.as_runtime_intent(), None);
}

#[test]
fn close_command_debug_shows_variant() {
    let cmd = OutputThreadRealTransportCommand::close_transport();
    let debug = format!("{:?}", cmd);
    assert!(debug.contains("CloseTransport"));
}

#[test]
fn command_is_copy_and_clone() {
    let a = OutputThreadRealTransportCommand::close_transport();
    let b = a;
    let c = a.clone();
    assert_eq!(a, b);
    assert_eq!(a, c);
}
