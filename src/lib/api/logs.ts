import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { LogGroup, LogStream, LogEvent, LogTailSession } from '$lib/types/logs';

/**
 * List CloudWatch log groups
 */
export async function listLogGroups(
  profile: string,
  region: string,
  prefix?: string
): Promise<LogGroup[]> {
  return invoke<LogGroup[]>('list_cloudwatch_log_groups', {
    profile,
    region,
    prefix: prefix ?? null,
  });
}

/**
 * List log streams in a log group
 */
export async function listLogStreams(
  profile: string,
  region: string,
  logGroupName: string,
  limit?: number
): Promise<LogStream[]> {
  return invoke<LogStream[]>('list_cloudwatch_log_streams', {
    profile,
    region,
    logGroupName,
    limit: limit ?? null,
  });
}

/**
 * Get log events from a log group
 */
export async function getLogEvents(
  profile: string,
  region: string,
  logGroupName: string,
  options?: {
    startTime?: number;
    endTime?: number;
    filterPattern?: string;
    limit?: number;
  }
): Promise<LogEvent[]> {
  return invoke<LogEvent[]>('get_cloudwatch_log_events', {
    profile,
    region,
    logGroupName,
    startTime: options?.startTime ?? null,
    endTime: options?.endTime ?? null,
    filterPattern: options?.filterPattern ?? null,
    limit: options?.limit ?? null,
  });
}

/**
 * Start a log tail session
 */
export async function startLogTail(
  profile: string,
  region: string,
  logGroupName: string,
  filterPattern?: string
): Promise<string> {
  return invoke<string>('start_log_tail', {
    profile,
    region,
    logGroupName,
    filterPattern: filterPattern ?? null,
  });
}

/**
 * Stop a log tail session
 */
export async function stopLogTail(sessionId: string): Promise<void> {
  return invoke<void>('stop_log_tail', { sessionId });
}

/**
 * List active log tail sessions
 */
export async function listLogTailSessions(): Promise<LogTailSession[]> {
  return invoke<LogTailSession[]>('list_log_tail_sessions');
}

/**
 * Listen for log output events
 */
export async function onLogOutput(
  sessionId: string,
  callback: (events: LogEvent[]) => void
): Promise<UnlistenFn> {
  return listen<LogEvent[]>(`logs:output:${sessionId}`, (event) => {
    callback(event.payload);
  });
}

/**
 * Listen for log errors
 */
export async function onLogError(
  sessionId: string,
  callback: (error: string) => void
): Promise<UnlistenFn> {
  return listen<string>(`logs:error:${sessionId}`, (event) => {
    callback(event.payload);
  });
}

/**
 * Listen for log session stopped
 */
export async function onLogStopped(
  sessionId: string,
  callback: () => void
): Promise<UnlistenFn> {
  return listen(`logs:stopped:${sessionId}`, () => {
    callback();
  });
}
