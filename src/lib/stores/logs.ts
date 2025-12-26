import { writable, derived, get } from 'svelte/store';
import type { LogGroup, LogEvent, LogTailSession } from '$lib/types/logs';
import {
  listLogGroups as apiListLogGroups,
  getLogEvents as apiGetLogEvents,
  startLogTail as apiStartLogTail,
  stopLogTail as apiStopLogTail,
  onLogOutput,
  onLogError,
  onLogStopped,
} from '$lib/api/logs';
import { selectedProfile, selectedRegion } from './profiles';

// Store for log groups
export const logGroups = writable<LogGroup[]>([]);
export const logGroupsLoading = writable<boolean>(false);
export const logGroupsError = writable<string | null>(null);

// Store for log tail sessions
export const logTailSessions = writable<Map<string, LogTailSessionState>>(new Map());
export const activeLogSessionId = writable<string | null>(null);

// Extended session state with logs buffer
export interface LogTailSessionState {
  session: LogTailSession;
  logs: LogEvent[];
  unlistenFns: (() => void)[];
  hasUnreadOutput: boolean;
}

// Derived stores
export const logTailSessionList = derived(logTailSessions, ($sessions) => {
  return Array.from($sessions.values()).map((s) => s.session);
});

export const hasActiveLogSessions = derived(logTailSessions, ($sessions) => {
  return $sessions.size > 0;
});

export const activeLogSession = derived(
  [logTailSessions, activeLogSessionId],
  ([$sessions, $activeId]) => {
    if (!$activeId) return null;
    return $sessions.get($activeId) ?? null;
  }
);

/**
 * Load CloudWatch log groups
 */
export async function loadLogGroups(): Promise<void> {
  const profile = get(selectedProfile);
  const region = get(selectedRegion);

  if (!profile || !region) {
    return;
  }

  logGroupsLoading.set(true);
  logGroupsError.set(null);

  try {
    const groups = await apiListLogGroups(profile, region);
    logGroups.set(groups);
  } catch (error) {
    logGroupsError.set(String(error));
    console.error('Failed to load log groups:', error);
  } finally {
    logGroupsLoading.set(false);
  }
}

/**
 * Get log events from a log group
 */
export async function fetchLogEvents(
  logGroupName: string,
  options?: {
    startTime?: number;
    endTime?: number;
    filterPattern?: string;
    limit?: number;
  }
): Promise<LogEvent[]> {
  const profile = get(selectedProfile);
  const region = get(selectedRegion);

  if (!profile || !region) {
    throw new Error('Profile and region are required');
  }

  return apiGetLogEvents(profile, region, logGroupName, options);
}

/**
 * Start a log tail session
 */
export async function createLogTailSession(
  logGroupName: string,
  filterPattern?: string
): Promise<string> {
  const profile = get(selectedProfile);
  const region = get(selectedRegion);

  if (!profile || !region) {
    throw new Error('Profile and region are required');
  }

  const sessionId = await apiStartLogTail(profile, region, logGroupName, filterPattern);

  // Create session state
  const session: LogTailSession = {
    id: sessionId,
    log_group_name: logGroupName,
    filter_pattern: filterPattern ?? null,
    profile,
    region,
    status: 'running',
    created_at: Date.now(),
  };

  const unlistenFns: (() => void)[] = [];

  // Listen for log output
  const unlistenOutput = await onLogOutput(sessionId, (events) => {
    logTailSessions.update((sessions) => {
      const state = sessions.get(sessionId);
      if (state) {
        // Append new events, keep last 5000
        const newLogs = [...state.logs, ...events].slice(-5000);
        const isActive = get(activeLogSessionId) === sessionId;
        sessions.set(sessionId, {
          ...state,
          logs: newLogs,
          hasUnreadOutput: !isActive,
        });
      }
      return new Map(sessions);
    });
  });
  unlistenFns.push(unlistenOutput);

  // Listen for errors
  const unlistenError = await onLogError(sessionId, (error) => {
    console.error(`Log tail error for ${sessionId}:`, error);
    logTailSessions.update((sessions) => {
      const state = sessions.get(sessionId);
      if (state) {
        sessions.set(sessionId, {
          ...state,
          session: { ...state.session, status: 'error' },
        });
      }
      return new Map(sessions);
    });
  });
  unlistenFns.push(unlistenError);

  // Listen for stopped
  const unlistenStopped = await onLogStopped(sessionId, () => {
    logTailSessions.update((sessions) => {
      const state = sessions.get(sessionId);
      if (state) {
        sessions.set(sessionId, {
          ...state,
          session: { ...state.session, status: 'stopped' },
        });
      }
      return new Map(sessions);
    });
  });
  unlistenFns.push(unlistenStopped);

  // Add to store
  logTailSessions.update((sessions) => {
    sessions.set(sessionId, {
      session,
      logs: [],
      unlistenFns,
      hasUnreadOutput: false,
    });
    return new Map(sessions);
  });

  // Set as active
  activeLogSessionId.set(sessionId);

  return sessionId;
}

/**
 * Stop and remove a log tail session
 */
export async function closeLogTailSession(sessionId: string): Promise<void> {
  const sessions = get(logTailSessions);
  const state = sessions.get(sessionId);

  if (state) {
    // Unsubscribe from events
    state.unlistenFns.forEach((fn) => fn());

    // Stop the backend session
    try {
      await apiStopLogTail(sessionId);
    } catch (e) {
      console.error('Failed to stop log tail:', e);
    }

    // Remove from store
    logTailSessions.update((s) => {
      s.delete(sessionId);
      return new Map(s);
    });

    // Update active session if needed
    if (get(activeLogSessionId) === sessionId) {
      const remaining = get(logTailSessions);
      const first = remaining.keys().next().value;
      activeLogSessionId.set(first ?? null);
    }
  }
}

/**
 * Switch to a log session
 */
export function switchToLogSession(sessionId: string): void {
  activeLogSessionId.set(sessionId);

  // Mark as read
  logTailSessions.update((sessions) => {
    const state = sessions.get(sessionId);
    if (state) {
      sessions.set(sessionId, { ...state, hasUnreadOutput: false });
    }
    return new Map(sessions);
  });
}

/**
 * Clear log groups (e.g., when changing profile/region)
 */
export function clearLogGroups(): void {
  logGroups.set([]);
  logGroupsError.set(null);
}
