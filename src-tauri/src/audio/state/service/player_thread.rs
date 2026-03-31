use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crossbeam_channel::Receiver;

use crate::audio::errors::{AudioError, AudioResult};

use super::lock_poison::lock_poison;
use super::types::{AudioSessionState, AudioSessionStatus, PlayStartOptions, ServiceCmd};

#[cfg(windows)]
use super::{session_loop, windows_device};

pub fn spawn_session_thread(
    session_id: u64,
    input_path: String,
    opt: PlayStartOptions,
    status: Arc<Mutex<AudioSessionStatus>>,
    cmd_rx: Receiver<ServiceCmd>,
) -> AudioResult<JoinHandle<()>> {
    let input_path = PathBuf::from(input_path);
    if !input_path.exists() {
        return Err(AudioError::InvalidInput(format!(
            "input_path not found: {}",
            input_path.display()
        )));
    }

    let t = thread::Builder::new()
        .name(format!("audio_session_{session_id}"))
        .spawn(move || {
            let r = run_session(session_id, &input_path, opt, status.clone(), cmd_rx);
            if let Err(e) = r {
                let mut s = lock_poison(&status);
                s.state = AudioSessionState::Error;
                s.set_last_error_from_audio_error(&e);
            }
        })
        .map_err(|e| AudioError::io("spawn audio session thread", e))?;

    Ok(t)
}

fn run_session(
    session_id: u64,
    input_path: &Path,
    opt: PlayStartOptions,
    status: Arc<Mutex<AudioSessionStatus>>,
    cmd_rx: Receiver<ServiceCmd>,
) -> AudioResult<()> {
    #[cfg(not(windows))]
    {
        let _ = (session_id, input_path, opt, cmd_rx);
        let mut s = lock_poison(&status);
        s.state = AudioSessionState::Error;
        s.set_last_error_from_audio_error(&AudioError::unsupported(
            "wasapi shared playback is Windows-only",
        ));
        return Ok(());
    }

    #[cfg(windows)]
    {
        use crate::audio::logging::AudioLog;
        use wasapi::initialize_mta;

        let init = initialize_mta();
        if init.is_err() {
            AudioLog::warn(&format!(
                "session {session_id}: initialize_mta failed: {:?}",
                init
            ));
        }

        let requested_device_id = opt.device_id.clone();
        let device = windows_device::WasapiSharedDevice::init(
            status.clone(),
            requested_device_id.as_deref(),
        )?;
        session_loop::run_session_loop(session_id, input_path, opt, status, cmd_rx, device)
    }
}
