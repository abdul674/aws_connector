import { writable, derived, get } from 'svelte/store';
import type { TerminalSession, CreateSessionInput } from '$lib/types/terminal';
import {
  createTerminalSession,
  closeTerminal,
  listTerminalSessions,
} from '$lib/api/terminal';

// All terminal sessions
export const sessions = writable<Map<string, TerminalSession>>(new Map());

// Currently active tab
export const activeSessionId = writable<string | null>(null);

// Derived: ordered list of sessions
export const sessionList = derived(sessions, ($sessions) => {
  return Array.from($sessions.values()).sort((a, b) => a.created_at - b.created_at);
});

// Derived: active session
export const activeSession = derived(
  [sessions, activeSessionId],
  ([$sessions, $activeId]) => {
    if (!$activeId) return null;
    return $sessions.get($activeId) ?? null;
  }
);

// Derived: has any sessions
export const hasActiveSessions = derived(sessions, ($sessions) => $sessions.size > 0);

// Derived: session count
export const sessionCount = derived(sessions, ($sessions) => $sessions.size);

/**
 * Create a new terminal session
 */
export async function createSession(input: CreateSessionInput): Promise<string> {
  const result = await createTerminalSession(input);

  const session: TerminalSession = {
    ...result.info,
    hasUnreadOutput: false,
  };

  sessions.update((s) => {
    s.set(result.session_id, session);
    return new Map(s);
  });

  // Set as active
  activeSessionId.set(result.session_id);

  return result.session_id;
}

/**
 * Close a terminal session
 */
export async function closeSession(sessionId: string): Promise<void> {
  await closeTerminal(sessionId);

  sessions.update((s) => {
    s.delete(sessionId);
    return new Map(s);
  });

  // If this was active, switch to another
  const currentActive = get(activeSessionId);
  if (currentActive === sessionId) {
    const remaining = get(sessionList);
    activeSessionId.set(remaining.length > 0 ? remaining[0].id : null);
  }
}

/**
 * Switch to a session tab
 */
export function switchToSession(sessionId: string): void {
  activeSessionId.set(sessionId);

  // Mark as read
  sessions.update((s) => {
    const session = s.get(sessionId);
    if (session) {
      session.hasUnreadOutput = false;
    }
    return new Map(s);
  });
}

/**
 * Mark session as having unread output (when not active)
 */
export function markUnread(sessionId: string): void {
  const currentActive = get(activeSessionId);
  if (currentActive === sessionId) return;

  sessions.update((s) => {
    const session = s.get(sessionId);
    if (session) {
      session.hasUnreadOutput = true;
    }
    return new Map(s);
  });
}

/**
 * Update session status
 */
export function updateSessionStatus(
  sessionId: string,
  status: TerminalSession['status']
): void {
  sessions.update((s) => {
    const session = s.get(sessionId);
    if (session) {
      session.status = status;
    }
    return new Map(s);
  });
}

/**
 * Load existing sessions on startup
 */
export async function loadSessions(): Promise<void> {
  try {
    const existingSessions = await listTerminalSessions();

    sessions.update((s) => {
      for (const info of existingSessions) {
        s.set(info.id, {
          ...info,
          hasUnreadOutput: false,
        });
      }
      return new Map(s);
    });

    // Set first as active if any exist and none selected
    if (existingSessions.length > 0 && get(activeSessionId) === null) {
      activeSessionId.set(existingSessions[0].id);
    }
  } catch (e) {
    console.error('Failed to load terminal sessions:', e);
  }
}
