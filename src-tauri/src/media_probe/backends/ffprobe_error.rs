pub fn command_error_message(stderr: &[u8], fallback: &str) -> String {
    let message = String::from_utf8_lossy(stderr).trim().to_string();

    if message.is_empty() {
        fallback.to_string()
    } else {
        message
    }
}

pub fn spawn_error_message(error: impl ToString) -> String {
    let message = error.to_string();

    if message.is_empty() {
        "ffprobe command is unavailable".to_string()
    } else {
        message
    }
}
