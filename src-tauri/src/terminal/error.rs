use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum TerminalError {
    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Failed to create PTY: {0}")]
    PtyCreationFailed(String),

    #[error("Failed to spawn process: {0}")]
    SpawnFailed(String),

    #[error("Failed to write to PTY: {0}")]
    WriteFailed(String),

    #[error("Failed to resize PTY: {0}")]
    ResizeFailed(String),

    #[error("Session already exists: {0}")]
    SessionAlreadyExists(String),

    #[error("Failed to decode input: {0}")]
    DecodeError(String),
}

impl From<TerminalError> for String {
    fn from(err: TerminalError) -> String {
        err.to_string()
    }
}
