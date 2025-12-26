use tauri::AppHandle;

use crate::aws::cloudwatch::{
    get_log_events, list_log_groups, list_log_streams, LogEvent, LogGroupInfo, LogStreamInfo,
};
use crate::logs::session::{LogTailSessionInfo, LOG_TAIL_REGISTRY};

/// List all CloudWatch log groups
#[tauri::command]
pub async fn list_cloudwatch_log_groups(
    profile: String,
    region: String,
    prefix: Option<String>,
) -> Result<Vec<LogGroupInfo>, String> {
    list_log_groups(&profile, &region, prefix.as_deref()).await
}

/// List log streams in a log group
#[tauri::command]
pub async fn list_cloudwatch_log_streams(
    profile: String,
    region: String,
    log_group_name: String,
    limit: Option<i32>,
) -> Result<Vec<LogStreamInfo>, String> {
    list_log_streams(&profile, &region, &log_group_name, limit).await
}

/// Get log events from a log group
#[tauri::command]
pub async fn get_cloudwatch_log_events(
    profile: String,
    region: String,
    log_group_name: String,
    start_time: Option<i64>,
    end_time: Option<i64>,
    filter_pattern: Option<String>,
    limit: Option<i32>,
) -> Result<Vec<LogEvent>, String> {
    get_log_events(
        &profile,
        &region,
        &log_group_name,
        start_time,
        end_time,
        filter_pattern.as_deref(),
        limit,
    )
    .await
}

/// Start a log tail session
#[tauri::command]
pub async fn start_log_tail(
    app_handle: AppHandle,
    profile: String,
    region: String,
    log_group_name: String,
    filter_pattern: Option<String>,
) -> Result<String, String> {
    let id = uuid::Uuid::new_v4().to_string();

    LOG_TAIL_REGISTRY.create_session(
        app_handle,
        id.clone(),
        log_group_name,
        filter_pattern,
        profile,
        region,
    );

    Ok(id)
}

/// Stop a log tail session
#[tauri::command]
pub async fn stop_log_tail(session_id: String) -> Result<(), String> {
    if LOG_TAIL_REGISTRY.stop_session(&session_id) {
        Ok(())
    } else {
        Err(format!("Session not found: {}", session_id))
    }
}

/// List all active log tail sessions
#[tauri::command]
pub async fn list_log_tail_sessions() -> Result<Vec<LogTailSessionInfo>, String> {
    Ok(LOG_TAIL_REGISTRY.list_sessions())
}
