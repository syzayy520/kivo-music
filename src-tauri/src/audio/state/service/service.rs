use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread::JoinHandle,
    time::Duration,
};

use crossbeam_channel::{bounded, Sender};

use crate::audio::errors::{AudioError, AudioResult};

use super::lock_poison::lock_poison;

use super::player_thread;
use super::types::{AudioSessionStatus, PlayStartOptions, ServiceCmd};

use super::queue_state::QueueState;
pub(super) struct SessionHandle {
    pub(super) cmd_tx: Sender<ServiceCmd>,
    pub(super) status: Arc<Mutex<AudioSessionStatus>>,
    pub(super) join: Option<JoinHandle<()>>,
}

#[derive(Default)]
pub struct AudioService {
    next_id: AtomicU64,
    pub(super) sessions: Mutex<HashMap<u64, SessionHandle>>,
    pub(super) queue: Mutex<QueueState>,
}

impl AudioService {
    pub(super) fn clear_active_session_if_matches(&self, session_id: u64) {
        let mut q = lock_poison(&self.queue);
        if q.active_session_id == Some(session_id) {
            q.active_session_id = None;
        }
    }

    pub(super) fn dispatch_cmd(
        &self,
        session_id: u64,
        action_name: &str,
        cmd: ServiceCmd,
        timeout: Duration,
    ) -> AudioResult<()> {
        let cmd_tx = self.cmd_tx_for_session(session_id)?;
        match cmd_tx.send_timeout(cmd, timeout) {
            Ok(()) => Ok(()),
            Err(crossbeam_channel::SendTimeoutError::Timeout(_)) => Err(AudioError::unsupported(
                format!("command queue busy: session_id={session_id} action={action_name}"),
            )),
            Err(crossbeam_channel::SendTimeoutError::Disconnected(_)) => Err(
                AudioError::InvalidInput(format!("session {session_id} is dead")),
            ),
        }
    }

    fn cmd_tx_for_session(&self, session_id: u64) -> AudioResult<Sender<ServiceCmd>> {
        let mut map = lock_poison(&self.sessions);
        Self::cleanup_finished_locked(&mut map);
        map.get(&session_id)
            .map(|h| h.cmd_tx.clone())
            .ok_or_else(|| AudioError::InvalidInput(format!("unknown session_id: {session_id}")))
    }

    fn status_arc_for_session(
        &self,
        session_id: u64,
    ) -> AudioResult<Arc<Mutex<AudioSessionStatus>>> {
        let mut map = lock_poison(&self.sessions);
        Self::cleanup_finished_locked(&mut map);
        map.get(&session_id)
            .map(|h| h.status.clone())
            .ok_or_else(|| AudioError::InvalidInput(format!("unknown session_id: {session_id}")))
    }

    pub fn play_start(&self, input_path: String, opt: PlayStartOptions) -> AudioResult<u64> {
        let input_path = input_path.trim().to_string();
        if input_path.is_empty() {
            return Err(AudioError::InvalidInput("input_path is empty".into()));
        }

        let pb = std::path::PathBuf::from(&input_path);
        if !pb.exists() {
            return Err(AudioError::InvalidInput(format!(
                "input_path not found: {}",
                pb.display()
            )));
        }

        if let Some(s) = opt.start_seconds {
            if !s.is_finite() || s < 0.0 {
                return Err(AudioError::InvalidInput(format!(
                    "start_seconds must be >= 0 and finite: {s}"
                )));
            }
        }
        if let Some(m) = opt.max_seconds {
            if !m.is_finite() || m <= 0.0 {
                return Err(AudioError::InvalidInput(format!(
                    "max_seconds must be > 0 and finite: {m}"
                )));
            }
        }

        let id = self.next_id.fetch_add(1, Ordering::Relaxed).max(1);
        let status = Arc::new(Mutex::new(AudioSessionStatus::new(id, input_path.clone())));
        // Use bounded channel to prevent unbounded memory growth and provide backpressure
        // Limit to 32 pending commands per session (reasonable for interactive audio control)
        let (cmd_tx, cmd_rx) = bounded::<ServiceCmd>(32);

        let mut map = lock_poison(&self.sessions);
        Self::cleanup_finished_locked(&mut map);

        // Prevent resource exhaustion by limiting concurrent sessions.
        // Check before thread spawn so we do not create detached sessions on rejection.
        if map.len() >= 256 {
            return Err(AudioError::unsupported(
                "too many concurrent sessions (max 256)",
            ));
        }

        let opt_for_state = opt.clone();
        let join = player_thread::spawn_session_thread(
            id,
            input_path.clone(),
            opt,
            status.clone(),
            cmd_rx,
        )
        .map_err(|e| AudioError::unsupported(format!("failed to spawn player thread: {e}")))?;

        map.insert(
            id,
            SessionHandle {
                cmd_tx,
                status,
                join: Some(join),
            },
        );
        drop(map);

        // Best-effort: keep the queue state aware of the latest started session.
        // This does NOT change playback behavior; it only improves Next/Prev consistency.
        {
            let mut q = lock_poison(&self.queue);
            q.note_play_started(id, &input_path, &opt_for_state);
        }
        Ok(id)
    }

    pub fn stop(&self, session_id: u64) -> AudioResult<()> {
        self.dispatch_cmd(
            session_id,
            "stop",
            ServiceCmd::Stop,
            Duration::from_millis(100),
        )
    }

    pub fn pause(&self, session_id: u64, paused: bool) -> AudioResult<()> {
        self.dispatch_cmd(
            session_id,
            "pause",
            ServiceCmd::Pause(paused),
            Duration::from_millis(100),
        )
    }

    pub fn seek(&self, session_id: u64, seconds: f64) -> AudioResult<()> {
        if !seconds.is_finite() || seconds < 0.0 {
            return Err(AudioError::InvalidInput(format!(
                "seek seconds invalid: {seconds}"
            )));
        }

        let cmd_tx = self.cmd_tx_for_session(session_id)?;

        // Handle bounded channel backpressure
        match cmd_tx.try_send(ServiceCmd::Seek {
            seconds: seconds.max(0.0),
        }) {
            Ok(()) => Ok(()),
            Err(crossbeam_channel::TrySendError::Full(_)) => {
                crate::audio::logging::AudioLog::warn(&format!(
                    "command queue full for session {session_id}, dropping seek command"
                ));
                Ok(())
            }
            Err(crossbeam_channel::TrySendError::Disconnected(_)) => Err(AudioError::InvalidInput(
                format!("session {session_id} is dead"),
            )),
        }
    }

    pub fn status(&self, session_id: u64) -> AudioResult<AudioSessionStatus> {
        let status = self.status_arc_for_session(session_id)?;
        let out = lock_poison(&status).clone();
        Ok(out)
    }
}
