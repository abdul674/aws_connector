export interface LogGroup {
  name: string;
  arn: string;
  stored_bytes: number;
  retention_days: number | null;
  creation_time: number;
  log_stream_count: number | null;
}

export interface LogStream {
  name: string;
  arn: string | null;
  creation_time: number | null;
  last_event_time: number | null;
  stored_bytes: number | null;
}

export interface LogEvent {
  timestamp: number;
  message: string;
  log_stream_name: string;
  ingestion_time: number | null;
}

export interface LogTailSession {
  id: string;
  log_group_name: string;
  filter_pattern: string | null;
  profile: string;
  region: string;
  status: 'running' | 'stopped' | 'error';
  created_at: number;
}
