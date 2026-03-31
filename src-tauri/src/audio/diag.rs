use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::audio::errors::{AudioError, AudioResult};

/// Extremely simple file logger used by the smoke test.
///
/// We intentionally avoid tying this into any global logger, so it can be used
/// from background threads without touching app-wide logging config.
#[derive(Debug)]
pub struct DiagWriter {
    path: PathBuf,
    w: BufWriter<File>,
}

impl DiagWriter {
    pub fn create(dir: impl AsRef<Path>, filename_prefix: &str) -> AudioResult<Self> {
        let dir = dir.as_ref();
        fs::create_dir_all(dir).map_err(|e| AudioError::io("create_dir_all", e))?;

        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AudioError::io("system_time", std::io::Error::other(e)))?
            .as_millis();

        let path = dir.join(format!("{filename_prefix}_{ts}.log"));
        let file = File::create(&path).map_err(|e| AudioError::io("create_log_file", e))?;
        let mut w = BufWriter::new(file);
        writeln!(w, "# kivo-shell wasapi_shared smoke log")?;
        writeln!(w, "# file={}", path.display())?;
        Ok(Self { path, w })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn line(&mut self, msg: impl AsRef<str>) -> AudioResult<()> {
        writeln!(self.w, "{}", msg.as_ref())?;
        self.w.flush()?;
        Ok(())
    }
}
