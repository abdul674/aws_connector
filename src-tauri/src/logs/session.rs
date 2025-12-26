use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::oneshot;

use crate::aws::cloudwatch::tail_log_events;

/// Information about a log tail session (serializable for frontend)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogTailSessionInfo {
    pub id: String,
    pub log_group_name: String,
    pub filter_pattern: Option<String>,
    pub profile: String,
    pub region: String,
    pub status: LogTailStatus,
    pub created_at: i64,
}

/// Status of a log tail session
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LogTailStatus {
    Running,
    Stopped,
    Error,
}

/// Internal log tail session state
pub struct LogTailSession {
    pub info: LogTailSessionInfo,
    pub stop_signal: Arc<AtomicBool>,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl LogTailSession {
    pub fn stop(&mut self) {
        self.stop_signal.store(true, Ordering::SeqCst);
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

/// Thread-safe registry of all active log tail sessions
pub struct LogTailRegistry {
    sessions: Mutex<HashMap<String, Arc<Mutex<LogTailSession>>>>,
}

impl LogTailRegistry {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    /// Create and start a new log tail session
    pub fn create_session(
        &self,
        app_handle: AppHandle,
        id: String,
        log_group_name: String,
        filter_pattern: Option<String>,
        profile: String,
        region: String,
    ) -> String {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let stop_signal = Arc::new(AtomicBool::new(false));

        let info = LogTailSessionInfo {
            id: id.clone(),
            log_group_name: log_group_name.clone(),
            filter_pattern: filter_pattern.clone(),
            profile: profile.clone(),
            region: region.clone(),
            status: LogTailStatus::Running,
            created_at: chrono::Utc::now().timestamp_millis(),
        };

        let session = LogTailSession {
            info,
            stop_signal: stop_signal.clone(),
            shutdown_tx: Some(shutdown_tx),
        };

        self.sessions
            .lock()
            .insert(id.clone(), Arc::new(Mutex::new(session)));

        // Spawn the tailing task
        let session_id = id.clone();
        let app = app_handle.clone();

        tokio::spawn(async move {
            run_log_tail(
                app,
                session_id,
                log_group_name,
                filter_pattern,
                profile,
                region,
                stop_signal,
                shutdown_rx,
            )
            .await;
        });

        id
    }

    /// Get a session by ID
    #[allow(dead_code)]
    pub fn get_session(&self, id: &str) -> Option<Arc<Mutex<LogTailSession>>> {
        self.sessions.lock().get(id).cloned()
    }

    /// Stop and remove a session
    pub fn stop_session(&self, id: &str) -> bool {
        if let Some(session) = self.sessions.lock().remove(id) {
            session.lock().stop();
            true
        } else {
            false
        }
    }

    /// List all session infos
    pub fn list_sessions(&self) -> Vec<LogTailSessionInfo> {
        self.sessions
            .lock()
            .values()
            .map(|s| s.lock().info.clone())
            .collect()
    }

    /// Update session status
    #[allow(dead_code)]
    pub fn update_status(&self, id: &str, status: LogTailStatus) {
        if let Some(session) = self.sessions.lock().get(id) {
            session.lock().info.status = status;
        }
    }
}

impl Default for LogTailRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Background task that polls CloudWatch and emits log events
async fn run_log_tail(
    app: AppHandle,
    session_id: String,
    log_group_name: String,
    filter_pattern: Option<String>,
    profile: String,
    region: String,
    stop_signal: Arc<AtomicBool>,
    mut shutdown_rx: oneshot::Receiver<()>,
) {
    // Start from now minus 30 seconds to catch recent logs
    let mut last_timestamp = chrono::Utc::now().timestamp_millis() - 30_000;
    let poll_interval = std::time::Duration::from_secs(2);

    loop {
        // Check if we should stop
        if stop_signal.load(Ordering::SeqCst) {
            break;
        }

        // Check for shutdown signal
        if shutdown_rx.try_recv().is_ok() {
            break;
        }

        // Poll for new events
        match tail_log_events(
            &profile,
            &region,
            &log_group_name,
            last_timestamp,
            filter_pattern.as_deref(),
        )
        .await
        {
            Ok((events, new_timestamp)) => {
                if !events.is_empty() {
                    // Emit events to frontend
                    let event_name = format!("logs:output:{}", session_id);
                    if let Err(e) = app.emit(&event_name, &events) {
                        tracing::error!("Failed to emit log events: {}", e);
                    }
                    last_timestamp = new_timestamp;
                }
            }
            Err(e) => {
                // Emit error to frontend
                let event_name = format!("logs:error:{}", session_id);
                if let Err(emit_err) = app.emit(&event_name, &e) {
                    tracing::error!("Failed to emit log error: {}", emit_err);
                }

                // Continue polling despite errors (might be temporary)
                tracing::warn!("Log tail error for {}: {}", session_id, e);
            }
        }

        // Wait before next poll
        tokio::select! {
            _ = tokio::time::sleep(poll_interval) => {},
            _ = &mut shutdown_rx => {
                break;
            }
        }
    }

    // Emit stopped event
    let event_name = format!("logs:stopped:{}", session_id);
    let _ = app.emit(&event_name, ());
}

// Global registry instance
use once_cell::sync::Lazy;

pub static LOG_TAIL_REGISTRY: Lazy<LogTailRegistry> = Lazy::new(LogTailRegistry::new);
