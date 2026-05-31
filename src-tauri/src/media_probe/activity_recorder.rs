use super::activity_log::MediaProbeActivityLogState;
use super::event::MediaProbeEvent;

pub struct MediaProbeActivityRecorder<'a> {
    activity_log: &'a MediaProbeActivityLogState,
    path: String,
    backend_name: String,
}

impl<'a> MediaProbeActivityRecorder<'a> {
    pub fn start(
        activity_log: &'a MediaProbeActivityLogState,
        path: impl Into<String>,
        backend_name: impl Into<String>,
    ) -> Self {
        let recorder = Self {
            activity_log,
            path: path.into(),
            backend_name: backend_name.into(),
        };

        recorder.activity_log.record(MediaProbeEvent::started(
            recorder.path.clone(),
            recorder.backend_name.clone(),
        ));

        recorder
    }

    pub fn succeeded(&self) {
        self.activity_log.record(MediaProbeEvent::succeeded(
            self.path.clone(),
            self.backend_name.clone(),
        ));
    }

    pub fn failed(&self, message: impl Into<String>) {
        self.activity_log.record(MediaProbeEvent::failed(
            self.path.clone(),
            self.backend_name.clone(),
            message,
        ));
    }
}
