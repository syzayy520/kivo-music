use super::super::request::OutputFlushRequest;

#[test]
fn request_seek_is_seek() {
    let req = OutputFlushRequest::Seek { position_ms: 1000 };
    assert!(req.is_seek());
}

#[test]
fn request_pause_is_not_seek() {
    let req = OutputFlushRequest::Pause;
    assert!(!req.is_seek());
}

#[test]
fn request_stop_is_not_seek() {
    let req = OutputFlushRequest::Stop;
    assert!(!req.is_seek());
}

#[test]
fn request_manual_is_not_seek() {
    let req = OutputFlushRequest::Manual;
    assert!(!req.is_seek());
}
