// Terminal session types matching the Rust backend

export type SessionType =
  | {
      type: 'ecs_exec';
      cluster: string;
      task: string;
      container: string;
      profile: string;
      region: string;
    }
  | {
      type: 'ssm_session';
      instance_id: string;
      profile: string;
      region: string;
    }
  | {
      type: 'ssm_port_forwarding';
      instance_id: string;
      local_port: number;
      remote_port: number;
      remote_host?: string;
      profile: string;
      region: string;
    }
  | {
      type: 'local';
    };

export type SessionStatus = 'starting' | 'running' | 'closing' | 'closed' | 'error';

export interface SessionInfo {
  id: string;
  title: string;
  session_type: SessionType;
  created_at: number;
  status: SessionStatus;
}

export interface TerminalSession extends SessionInfo {
  // Frontend-only state
  hasUnreadOutput: boolean;
}

export interface CreateSessionInput {
  session_type: SessionType;
  title?: string;
  shell?: string;
}

export interface CreateSessionOutput {
  session_id: string;
  info: SessionInfo;
}
