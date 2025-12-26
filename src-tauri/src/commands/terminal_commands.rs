use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use uuid::Uuid;

use crate::terminal::{
    create_pty_session, resize_pty, start_output_stream, write_to_pty, SessionInfo,
    SessionRegistry, SessionStatus, SessionType,
};

/// Global session registry
static SESSIONS: Lazy<SessionRegistry> = Lazy::new(SessionRegistry::new);

/// Input for creating a new terminal session
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSessionInput {
    pub session_type: SessionType,
    pub title: Option<String>,
    pub shell: Option<String>,
}

/// Output from creating a new terminal session
#[derive(Debug, Clone, Serialize)]
pub struct CreateSessionOutput {
    pub session_id: String,
    pub info: SessionInfo,
}

/// Create a new terminal session
#[tauri::command]
pub async fn terminal_create_session(
    app_handle: AppHandle,
    input: CreateSessionInput,
) -> Result<CreateSessionOutput, String> {
    let session_id = Uuid::new_v4().to_string();
    let shell = input.shell.unwrap_or_else(|| "/bin/sh".to_string());

    // Generate title if not provided
    let title = input.title.unwrap_or_else(|| match &input.session_type {
        SessionType::EcsExec { container, .. } => format!("ECS: {}", container),
        SessionType::SsmSession { instance_id, .. } => format!("EC2: {}", instance_id),
        SessionType::SsmPortForwarding {
            local_port,
            remote_port,
            ..
        } => format!("Port Forward: {} -> {}", local_port, remote_port),
        SessionType::Local => "Local Shell".to_string(),
    });

    let info = SessionInfo {
        id: session_id.clone(),
        title,
        session_type: input.session_type.clone(),
        created_at: chrono::Utc::now().timestamp(),
        status: SessionStatus::Starting,
    };

    // Build command based on session type
    let (command, args): (&str, Vec<String>) = match &input.session_type {
        SessionType::EcsExec {
            cluster,
            task,
            container,
            profile,
            region,
        } => (
            "aws",
            vec![
                "ecs".to_string(),
                "execute-command".to_string(),
                "--cluster".to_string(),
                cluster.clone(),
                "--task".to_string(),
                task.clone(),
                "--container".to_string(),
                container.clone(),
                "--interactive".to_string(),
                "--command".to_string(),
                shell.clone(),
                "--profile".to_string(),
                profile.clone(),
                "--region".to_string(),
                region.clone(),
            ],
        ),
        SessionType::SsmSession {
            instance_id,
            profile,
            region,
        } => (
            "aws",
            vec![
                "ssm".to_string(),
                "start-session".to_string(),
                "--target".to_string(),
                instance_id.clone(),
                "--profile".to_string(),
                profile.clone(),
                "--region".to_string(),
                region.clone(),
            ],
        ),
        SessionType::SsmPortForwarding {
            instance_id,
            local_port,
            remote_port,
            remote_host,
            profile,
            region,
        } => {
            let mut args = vec![
                "ssm".to_string(),
                "start-session".to_string(),
                "--target".to_string(),
                instance_id.clone(),
            ];

            // Choose document based on whether remote_host is specified
            if let Some(host) = remote_host {
                // Remote host forwarding (for RDS/ElastiCache endpoints)
                args.push("--document-name".to_string());
                args.push("AWS-StartPortForwardingSessionToRemoteHost".to_string());
                args.push("--parameters".to_string());
                args.push(format!(
                    "host={},portNumber={},localPortNumber={}",
                    host, remote_port, local_port
                ));
            } else {
                // Local port forwarding to the instance itself
                args.push("--document-name".to_string());
                args.push("AWS-StartPortForwardingSession".to_string());
                args.push("--parameters".to_string());
                args.push(format!(
                    "portNumber={},localPortNumber={}",
                    remote_port, local_port
                ));
            }

            args.push("--profile".to_string());
            args.push(profile.clone());
            args.push("--region".to_string());
            args.push(region.clone());

            ("aws", args)
        }
        SessionType::Local => (&shell, vec![]),
    };

    // Convert args to references
    let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    // Create the PTY session
    let mut session = create_pty_session(info.clone(), command, &args_refs, vec![])
        .map_err(|e| e.to_string())?;

    // Take the reader and start streaming output
    if let Some(reader) = session.reader.take() {
        start_output_stream(app_handle, session_id.clone(), reader);
    }

    // Update status to running
    session.info.status = SessionStatus::Running;
    let final_info = session.info.clone();

    // Store the session
    SESSIONS.create_session(session);

    Ok(CreateSessionOutput {
        session_id,
        info: final_info,
    })
}

/// Write data to a terminal session (data should be base64 encoded)
#[tauri::command]
pub async fn terminal_write(session_id: String, data: String) -> Result<(), String> {
    let session = SESSIONS
        .get_session(&session_id)
        .ok_or_else(|| format!("Session not found: {}", session_id))?;

    // Decode base64 input
    let bytes = BASE64
        .decode(&data)
        .map_err(|e| format!("Failed to decode input: {}", e))?;

    let mut session = session.lock();
    write_to_pty(&mut session, &bytes).map_err(|e| e.to_string())
}

/// Resize a terminal session
#[tauri::command]
pub async fn terminal_resize(session_id: String, cols: u16, rows: u16) -> Result<(), String> {
    let session = SESSIONS
        .get_session(&session_id)
        .ok_or_else(|| format!("Session not found: {}", session_id))?;

    let mut session = session.lock();
    resize_pty(&mut session, cols, rows).map_err(|e| e.to_string())
}

/// Close a terminal session
#[tauri::command]
pub async fn terminal_close(session_id: String) -> Result<(), String> {
    if let Some(session) = SESSIONS.remove_session(&session_id) {
        let mut session = session.lock();
        session.info.status = SessionStatus::Closed;
        // Kill the child process - it will be cleaned up when dropped
        let _ = session.child.kill();
    }
    Ok(())
}

/// List all active terminal sessions
#[tauri::command]
pub async fn terminal_list_sessions() -> Vec<SessionInfo> {
    SESSIONS.list_sessions()
}

/// Get info for a specific session
#[tauri::command]
pub async fn terminal_get_session(session_id: String) -> Result<SessionInfo, String> {
    let session = SESSIONS
        .get_session(&session_id)
        .ok_or_else(|| format!("Session not found: {}", session_id))?;

    let info = session.lock().info.clone();
    Ok(info)
}
