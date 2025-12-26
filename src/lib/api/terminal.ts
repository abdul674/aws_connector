import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type {
  CreateSessionInput,
  CreateSessionOutput,
  SessionInfo,
} from '$lib/types/terminal';

// Encode string to base64 for transport
function encodeData(data: string): string {
  // Use TextEncoder for proper UTF-8 handling
  const encoder = new TextEncoder();
  const bytes = encoder.encode(data);
  let binary = '';
  bytes.forEach((byte) => {
    binary += String.fromCharCode(byte);
  });
  return btoa(binary);
}

// Decode base64 data from backend
function decodeData(data: string): Uint8Array {
  const binary = atob(data);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) {
    bytes[i] = binary.charCodeAt(i);
  }
  return bytes;
}

/**
 * Create a new terminal session
 */
export async function createTerminalSession(
  input: CreateSessionInput
): Promise<CreateSessionOutput> {
  return invoke<CreateSessionOutput>('terminal_create_session', { input });
}

/**
 * Write data to a terminal session
 */
export async function writeToTerminal(sessionId: string, data: string): Promise<void> {
  const encoded = encodeData(data);
  return invoke('terminal_write', { sessionId, data: encoded });
}

/**
 * Resize a terminal session
 */
export async function resizeTerminal(
  sessionId: string,
  cols: number,
  rows: number
): Promise<void> {
  return invoke('terminal_resize', { sessionId, cols, rows });
}

/**
 * Close a terminal session
 */
export async function closeTerminal(sessionId: string): Promise<void> {
  return invoke('terminal_close', { sessionId });
}

/**
 * List all active terminal sessions
 */
export async function listTerminalSessions(): Promise<SessionInfo[]> {
  return invoke<SessionInfo[]>('terminal_list_sessions');
}

/**
 * Get info for a specific session
 */
export async function getTerminalSession(sessionId: string): Promise<SessionInfo> {
  return invoke<SessionInfo>('terminal_get_session', { sessionId });
}

/**
 * Listen for terminal output events
 */
export function onTerminalOutput(
  sessionId: string,
  callback: (data: Uint8Array) => void
): Promise<UnlistenFn> {
  return listen<string>(`terminal:output:${sessionId}`, (event) => {
    const decoded = decodeData(event.payload);
    callback(decoded);
  });
}

/**
 * Listen for terminal closed events
 */
export function onTerminalClosed(
  sessionId: string,
  callback: () => void
): Promise<UnlistenFn> {
  return listen(`terminal:closed:${sessionId}`, () => {
    callback();
  });
}

/**
 * Listen for terminal error events
 */
export function onTerminalError(
  sessionId: string,
  callback: (error: string) => void
): Promise<UnlistenFn> {
  return listen<string>(`terminal:error:${sessionId}`, (event) => {
    callback(event.payload);
  });
}
