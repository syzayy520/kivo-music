use super::output_thread_worker_shutdown::*;

#[test]
fn none_does_not_request_stop() {
    assert!(!OutputThreadWorkerShutdownRequest::None.requests_stop());
}

#[test]
fn request_stop_requests_stop() {
    assert!(OutputThreadWorkerShutdownRequest::RequestStop.requests_stop());
}

#[test]
fn close_transport_closes_transport() {
    assert!(OutputThreadWorkerShutdownRequest::CloseTransport.closes_transport());
}

#[test]
fn no_worker_outcome_requires_no_worker() {
    assert!(!OutputThreadWorkerShutdownOutcome::NoWorker.requires_worker());
}

#[test]
fn stop_marked_is_not_terminal() {
    assert!(!OutputThreadWorkerShutdownOutcome::StopMarked.is_terminal());
}

#[test]
fn already_stopped_is_terminal() {
    assert!(OutputThreadWorkerShutdownOutcome::AlreadyStopped.is_terminal());
}

#[test]
fn transport_closed_is_terminal() {
    assert!(OutputThreadWorkerShutdownOutcome::TransportClosed.is_terminal());
}

#[test]
fn unsupported_is_terminal() {
    assert!(OutputThreadWorkerShutdownOutcome::Unsupported.is_terminal());
}

#[test]
fn stop_marked_requires_worker() {
    assert!(OutputThreadWorkerShutdownOutcome::StopMarked.requires_worker());
}

#[test]
fn close_transport_does_not_request_stop() {
    assert!(!OutputThreadWorkerShutdownRequest::CloseTransport.requests_stop());
}
