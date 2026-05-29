#[derive(Debug, Clone)]
pub enum PlaybackError {
    BackendUnavailable,
    FileMissing,
    UnsupportedFormat,
    OutputDeviceUnavailable,
    Unknown(String),
}
