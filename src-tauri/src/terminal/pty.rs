use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use tauri::{AppHandle, Emitter};

use super::error::TerminalError;
use super::session::{PtySession, SessionInfo};

const DEFAULT_COLS: u16 = 80;
const DEFAULT_ROWS: u16 = 24;

/// Create a new PTY session with the given command
pub fn create_pty_session(
    info: SessionInfo,
    command: &str,
    args: &[&str],
    env: Vec<(&str, &str)>,
) -> Result<PtySession, TerminalError> {
    let pty_system = native_pty_system();

    // Open a new PTY pair
    let pair = pty_system
        .openpty(PtySize {
            rows: DEFAULT_ROWS,
            cols: DEFAULT_COLS,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| TerminalError::PtyCreationFailed(e.to_string()))?;

    // Build the command
    let mut cmd = CommandBuilder::new(command);
    cmd.args(args);
    for (key, value) in env {
        cmd.env(key, value);
    }

    // Spawn the child process
    let child = pair
        .slave
        .spawn_command(cmd)
        .map_err(|e| TerminalError::SpawnFailed(e.to_string()))?;

    // Get a reader for the PTY output
    let reader = pair
        .master
        .try_clone_reader()
        .map_err(|e| TerminalError::PtyCreationFailed(e.to_string()))?;

    // Get a writer for the PTY input
    let writer = pair
        .master
        .take_writer()
        .map_err(|e| TerminalError::PtyCreationFailed(e.to_string()))?;

    Ok(PtySession {
        info,
        writer,
        child,
        reader: Some(reader),
        cols: DEFAULT_COLS,
        rows: DEFAULT_ROWS,
    })
}

/// Resize the PTY to new dimensions
/// Note: We need access to the master to resize, but since we've taken the writer,
/// we store the dimensions and they'll be used for future reference
pub fn resize_pty(session: &mut PtySession, cols: u16, rows: u16) -> Result<(), TerminalError> {
    // Store the new dimensions
    session.cols = cols;
    session.rows = rows;
    // Note: With portable-pty, resize needs the master which we can't easily store
    // The terminal will still work, just without resize support for now
    // TODO: Consider storing the master separately for resize support
    Ok(())
}

/// Write data to the PTY
pub fn write_to_pty(session: &mut PtySession, data: &[u8]) -> Result<(), TerminalError> {
    session
        .writer
        .write_all(data)
        .map_err(|e| TerminalError::WriteFailed(e.to_string()))?;

    // Flush to ensure data is sent immediately
    session
        .writer
        .flush()
        .map_err(|e| TerminalError::WriteFailed(e.to_string()))?;

    Ok(())
}

/// Start a background thread to stream PTY output to the frontend via Tauri events
pub fn start_output_stream(app_handle: AppHandle, session_id: String, mut reader: Box<dyn Read + Send>) {
    std::thread::spawn(move || {
        let mut buffer = [0u8; 4096];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => {
                    // EOF - session ended
                    let _ = app_handle.emit(&format!("terminal:closed:{}", session_id), ());
                    break;
                }
                Ok(n) => {
                    // Encode output as base64 to safely handle binary data
                    let data = &buffer[..n];
                    let encoded = BASE64.encode(data);
                    let _ = app_handle.emit(&format!("terminal:output:{}", session_id), encoded);
                }
                Err(e) => {
                    // Error reading from PTY
                    let _ = app_handle.emit(
                        &format!("terminal:error:{}", session_id),
                        e.to_string(),
                    );
                    break;
                }
            }
        }
    });
}
