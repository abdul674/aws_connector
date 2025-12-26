use parking_lot::Mutex;
use portable_pty::Child;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Arc;

/// Information about a terminal session (serializable for frontend)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub title: String,
    pub session_type: SessionType,
    pub created_at: i64,
    pub status: SessionStatus,
}

/// Type of terminal session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SessionType {
    EcsExec {
        cluster: String,
        task: String,
        container: String,
        profile: String,
        region: String,
    },
    SsmSession {
        instance_id: String,
        profile: String,
        region: String,
    },
    SsmPortForwarding {
        instance_id: String,
        local_port: u16,
        remote_port: u16,
        remote_host: Option<String>,
        profile: String,
        region: String,
    },
    Local,
}

/// Status of a terminal session
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Starting,
    Running,
    Closing,
    Closed,
    Error,
}

/// Internal PTY session state (not serializable)
/// Using separate writer handle for thread safety
pub struct PtySession {
    pub info: SessionInfo,
    pub writer: Box<dyn Write + Send>,
    pub child: Box<dyn Child + Send + Sync>,
    pub reader: Option<Box<dyn Read + Send>>,
    pub cols: u16,
    pub rows: u16,
}

/// Thread-safe registry of all active sessions
pub struct SessionRegistry {
    sessions: Mutex<HashMap<String, Arc<Mutex<PtySession>>>>,
}

impl SessionRegistry {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    /// Add a new session to the registry
    pub fn create_session(&self, session: PtySession) -> String {
        let id = session.info.id.clone();
        self.sessions
            .lock()
            .insert(id.clone(), Arc::new(Mutex::new(session)));
        id
    }

    /// Get a session by ID
    pub fn get_session(&self, id: &str) -> Option<Arc<Mutex<PtySession>>> {
        self.sessions.lock().get(id).cloned()
    }

    /// Remove a session from the registry
    pub fn remove_session(&self, id: &str) -> Option<Arc<Mutex<PtySession>>> {
        self.sessions.lock().remove(id)
    }

    /// List all session infos
    pub fn list_sessions(&self) -> Vec<SessionInfo> {
        self.sessions
            .lock()
            .values()
            .map(|s| s.lock().info.clone())
            .collect()
    }
}

impl Default for SessionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
