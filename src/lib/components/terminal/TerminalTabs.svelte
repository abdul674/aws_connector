<script lang="ts">
  import { fade, fly } from 'svelte/transition';
  import { flip } from 'svelte/animate';
  import { sessionList, activeSessionId, switchToSession, closeSession } from '$lib/stores/terminals';

  function handleClose(e: Event, sessionId: string) {
    e.stopPropagation();
    closeSession(sessionId);
  }

  function getStatusIcon(status: string): string {
    switch (status) {
      case 'running':
        return 'status-running';
      case 'closed':
        return 'status-closed';
      case 'error':
        return 'status-error';
      default:
        return 'status-starting';
    }
  }

  function getTypeIcon(type: string): string {
    switch (type) {
      case 'ecs_exec':
        return 'icon-container';
      case 'ssm_session':
        return 'icon-server';
      default:
        return 'icon-terminal';
    }
  }
</script>

<div class="terminal-tabs">
  {#each $sessionList as session (session.id)}
    <div
      class="tab"
      class:active={$activeSessionId === session.id}
      class:unread={session.hasUnreadOutput}
      onclick={() => switchToSession(session.id)}
      onkeydown={(e) => e.key === 'Enter' && switchToSession(session.id)}
      role="tab"
      tabindex="0"
      title={session.title}
      in:fly={{ x: -20, duration: 200 }}
      out:fade={{ duration: 150 }}
      animate:flip={{ duration: 200 }}
    >
      <span class="type-icon {getTypeIcon(session.session_type.type)}"></span>
      <span class="status-dot {getStatusIcon(session.status)}"></span>
      <span class="tab-title">{session.title}</span>
      <button
        class="close-btn"
        onclick={(e) => handleClose(e, session.id)}
        title="Close terminal"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
    </div>
  {/each}
</div>

<style>
  .terminal-tabs {
    display: flex;
    gap: 2px;
    padding: 4px 8px;
    background-color: var(--color-bg-tertiary);
    border-bottom: 1px solid var(--color-border);
    overflow-x: auto;
    min-height: 36px;
  }

  .terminal-tabs::-webkit-scrollbar {
    height: 4px;
  }

  .terminal-tabs::-webkit-scrollbar-thumb {
    background-color: var(--color-border);
    border-radius: 2px;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    padding-right: 4px;
    border-radius: 4px;
    font-size: 12px;
    color: var(--color-text-secondary);
    background-color: transparent;
    border: 1px solid transparent;
    transition: all 150ms ease;
    white-space: nowrap;
    max-width: 200px;
    cursor: pointer;
  }

  .tab:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .tab.active {
    background-color: var(--color-bg-secondary);
    border-color: var(--color-border);
    color: var(--color-text-primary);
  }

  .tab.unread {
    color: var(--color-accent);
    background-color: rgba(59, 130, 246, 0.08);
  }

  .tab.unread .tab-title {
    font-weight: 600;
  }

  .tab.unread .status-dot {
    box-shadow: 0 0 6px var(--color-success);
  }

  .type-icon {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
  }

  .icon-container::before {
    content: '';
  }

  .icon-server::before {
    content: '';
  }

  .icon-terminal::before {
    content: '';
  }

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .status-running {
    background-color: var(--color-success);
  }

  .status-closed {
    background-color: var(--color-text-muted);
  }

  .status-error {
    background-color: var(--color-error);
  }

  .status-starting {
    background-color: var(--color-warning);
    animation: pulse 1.5s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.5; transform: scale(0.8); }
  }

  .tab-title {
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    border-radius: 4px;
    color: var(--color-text-muted);
    flex-shrink: 0;
    opacity: 0;
    transition: opacity 150ms ease, background-color 150ms ease;
  }

  .tab:hover .close-btn {
    opacity: 1;
  }

  .close-btn:hover {
    background-color: var(--color-bg-tertiary);
    color: var(--color-text-primary);
  }
</style>
