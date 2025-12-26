use aws_config::BehaviorVersion;
use aws_sdk_cloudwatchlogs::Client as CloudWatchLogsClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogGroupInfo {
    pub name: String,
    pub arn: String,
    pub stored_bytes: i64,
    pub retention_days: Option<i32>,
    pub creation_time: i64,
    pub log_stream_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogStreamInfo {
    pub name: String,
    pub arn: Option<String>,
    pub creation_time: Option<i64>,
    pub last_event_time: Option<i64>,
    pub stored_bytes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    pub timestamp: i64,
    pub message: String,
    pub log_stream_name: String,
    pub ingestion_time: Option<i64>,
}

/// Create a CloudWatch Logs client with the specified profile and region
async fn create_cloudwatch_client(profile: &str, region: &str) -> Result<CloudWatchLogsClient, String> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .profile_name(profile)
        .region(aws_config::Region::new(region.to_string()))
        .load()
        .await;

    Ok(CloudWatchLogsClient::new(&config))
}

/// List all log groups, optionally filtered by prefix
pub async fn list_log_groups(
    profile: &str,
    region: &str,
    prefix: Option<&str>,
) -> Result<Vec<LogGroupInfo>, String> {
    let client = create_cloudwatch_client(profile, region).await?;

    let mut log_groups = Vec::new();
    let mut next_token: Option<String> = None;

    loop {
        let mut request = client.describe_log_groups();

        if let Some(p) = prefix {
            request = request.log_group_name_prefix(p);
        }

        if let Some(token) = next_token {
            request = request.next_token(token);
        }

        let result = request
            .send()
            .await
            .map_err(|e| format!("Failed to list log groups: {}", e))?;

        for lg in result.log_groups() {
            log_groups.push(LogGroupInfo {
                name: lg.log_group_name().unwrap_or_default().to_string(),
                arn: lg.arn().unwrap_or_default().to_string(),
                stored_bytes: lg.stored_bytes().unwrap_or(0),
                retention_days: lg.retention_in_days(),
                creation_time: lg.creation_time().unwrap_or(0),
                log_stream_count: None,
            });
        }

        next_token = result.next_token().map(|s| s.to_string());
        if next_token.is_none() {
            break;
        }
    }

    Ok(log_groups)
}

/// List log streams in a log group
pub async fn list_log_streams(
    profile: &str,
    region: &str,
    log_group_name: &str,
    limit: Option<i32>,
) -> Result<Vec<LogStreamInfo>, String> {
    let client = create_cloudwatch_client(profile, region).await?;

    let mut request = client
        .describe_log_streams()
        .log_group_name(log_group_name)
        .order_by(aws_sdk_cloudwatchlogs::types::OrderBy::LastEventTime)
        .descending(true);

    if let Some(l) = limit {
        request = request.limit(l);
    }

    let result = request
        .send()
        .await
        .map_err(|e| format!("Failed to list log streams: {}", e))?;

    let log_streams = result
        .log_streams()
        .iter()
        .map(|ls| LogStreamInfo {
            name: ls.log_stream_name().unwrap_or_default().to_string(),
            arn: ls.arn().map(|s| s.to_string()),
            creation_time: ls.creation_time(),
            last_event_time: ls.last_event_timestamp(),
            stored_bytes: None, // Deprecated by AWS
        })
        .collect();

    Ok(log_streams)
}

/// Get log events from a log group using filter
pub async fn get_log_events(
    profile: &str,
    region: &str,
    log_group_name: &str,
    start_time: Option<i64>,
    end_time: Option<i64>,
    filter_pattern: Option<&str>,
    limit: Option<i32>,
) -> Result<Vec<LogEvent>, String> {
    let client = create_cloudwatch_client(profile, region).await?;

    let mut request = client.filter_log_events().log_group_name(log_group_name);

    if let Some(st) = start_time {
        request = request.start_time(st);
    }

    if let Some(et) = end_time {
        request = request.end_time(et);
    }

    if let Some(pattern) = filter_pattern {
        if !pattern.is_empty() {
            request = request.filter_pattern(pattern);
        }
    }

    if let Some(l) = limit {
        request = request.limit(l);
    }

    let result = request
        .send()
        .await
        .map_err(|e| format!("Failed to get log events: {}", e))?;

    let events = result
        .events()
        .iter()
        .map(|e| LogEvent {
            timestamp: e.timestamp().unwrap_or(0),
            message: e.message().unwrap_or_default().to_string(),
            log_stream_name: e.log_stream_name().unwrap_or_default().to_string(),
            ingestion_time: e.ingestion_time(),
        })
        .collect();

    Ok(events)
}

/// Get log events from a specific log stream
#[allow(dead_code)]
pub async fn get_log_stream_events(
    profile: &str,
    region: &str,
    log_group_name: &str,
    log_stream_name: &str,
    start_time: Option<i64>,
    limit: Option<i32>,
    next_token: Option<&str>,
) -> Result<(Vec<LogEvent>, Option<String>), String> {
    let client = create_cloudwatch_client(profile, region).await?;

    let mut request = client
        .get_log_events()
        .log_group_name(log_group_name)
        .log_stream_name(log_stream_name)
        .start_from_head(false); // Start from most recent

    if let Some(st) = start_time {
        request = request.start_time(st);
    }

    if let Some(l) = limit {
        request = request.limit(l);
    }

    if let Some(token) = next_token {
        request = request.next_token(token);
    }

    let result = request
        .send()
        .await
        .map_err(|e| format!("Failed to get log stream events: {}", e))?;

    let events = result
        .events()
        .iter()
        .map(|e| LogEvent {
            timestamp: e.timestamp().unwrap_or(0),
            message: e.message().unwrap_or_default().to_string(),
            log_stream_name: log_stream_name.to_string(),
            ingestion_time: e.ingestion_time(),
        })
        .collect();

    let forward_token = result.next_forward_token().map(|s| s.to_string());

    Ok((events, forward_token))
}

/// Tail logs from a log group (get events since a timestamp)
/// Returns events and the timestamp to use for the next poll
pub async fn tail_log_events(
    profile: &str,
    region: &str,
    log_group_name: &str,
    since_timestamp: i64,
    filter_pattern: Option<&str>,
) -> Result<(Vec<LogEvent>, i64), String> {
    let client = create_cloudwatch_client(profile, region).await?;

    let mut request = client
        .filter_log_events()
        .log_group_name(log_group_name)
        .start_time(since_timestamp);

    if let Some(pattern) = filter_pattern {
        if !pattern.is_empty() {
            request = request.filter_pattern(pattern);
        }
    }

    let result = request
        .send()
        .await
        .map_err(|e| format!("Failed to tail log events: {}", e))?;

    let events: Vec<LogEvent> = result
        .events()
        .iter()
        .map(|e| LogEvent {
            timestamp: e.timestamp().unwrap_or(0),
            message: e.message().unwrap_or_default().to_string(),
            log_stream_name: e.log_stream_name().unwrap_or_default().to_string(),
            ingestion_time: e.ingestion_time(),
        })
        .collect();

    // Get the latest timestamp for the next poll
    let new_timestamp = events
        .iter()
        .map(|e| e.timestamp)
        .max()
        .map(|t| t + 1) // Add 1ms to avoid duplicates
        .unwrap_or(since_timestamp);

    Ok((events, new_timestamp))
}
