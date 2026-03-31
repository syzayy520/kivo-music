use std::path::Path;

/// Minimal logging helpers for the audio subsystem.
///
/// We intentionally keep this dependency-free (no `log`/`tracing`) so the shell stays small.
pub struct AudioLog;

impl AudioLog {
    pub fn info(msg: &str) {
        eprintln!("[audio][info] {msg}");
    }

    pub fn warn(msg: &str) {
        eprintln!("[audio][warn] {msg}");
    }

    pub fn error(msg: &str) {
        eprintln!("[audio][error] {msg}");
    }

    pub fn ffmpeg_dir_hint(dll_dir: Option<&Path>) -> String {
        match dll_dir {
            Some(p) => format!("ffmpeg_dir={}", p.display()),
            None => "ffmpeg_dir=<none>; you can set env KIVO_FFMPEG_DIR".to_string(),
        }
    }
}
