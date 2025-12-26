<script lang="ts">
  import { fade, fly } from 'svelte/transition';
  import { flip } from 'svelte/animate';
  import {
    logTailSessions,
    activeLogSessionId,
    switchToLogSession,
    closeLogTailSession,
  } from '$lib/stores/logs';

  function handleClose(e: Event, sessionId: string) {
    e.stopPropagation();
    closeLogTailSession(sessionId);
  }

  function getStatusClass(status: string): string {
    switch (status) {
      case 'running':
        return 'status-running';
      case 'stopped':
        return 'status-stopped';
      case 'error':
        return 'status-error';
      default:
        return 'status-running';
    }
  }

  function getLogGroupShortName(name: string): string {
    // Extract the last part of the log group name for display
    const parts = name.split('/');
    return parts[parts.length - 1] || name;
  }
</script>

<div class="log-tabs">
  {#each [...$logTailSessions.entries()] as [sessionId, state] (sessionId)}
    <div
      class="tab"
      class:active={$activeLogSessionId === sessionId}
      class:unread={state.hasUnreadOutput}
      onclick={() => switchToLogSession(sessionId)}
      onkeydown={(e) => e.key === 'Enter' && switchToLogSession(sessionId)}
      role="tab"
      tabindex="0"
      title={state.session.log_group_name}
      in:fly={{ x: -20, duration: 200 }}
      out:fade={{ duration: 150 }}
      animate:flip={{ duration: 200 }}
    >
      <span class="type-icon">
        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
          <polyline points="14 2 14 8 20 8"></polyline>
        </svg>
      </span>
      <span class="status-dot {getStatusClass(state.session.status)}"></span>
      <span class="tab-title">{getLogGroupShortName(state.session.log_group_name)}</span>
      <button
        class="close-btn"
        onclick={(e) => handleClose(e, sessionId)}
        title="Close log tail"
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
  .log-tabs {
    display: flex;
    gap: 2px;
    padding: 4px 8px;
    background-color: var(--color-bg-tertiary);
    border-bottom: 1px solid var(--color-border);
    overflow-x: auto;
    min-height: 36px;
  }

  .log-tabs::-webkit-scrollbar {
    height: 4px;
  }

  .log-tabs::-webkit-scrollbar-thumb {
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
    color: #f97316;
    background-color: rgba(249, 115, 22, 0.08);
  }

  .tab.unread .tab-title {
    font-weight: 600;
  }

  .type-icon {
    display: flex;
    align-items: center;
    color: #f97316;
  }

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .status-running {
    background-color: var(--color-success);
    animation: pulse 1.5s ease-in-out infinite;
  }

  .status-stopped {
    background-color: var(--color-text-muted);
  }

  .status-error {
    background-color: var(--color-error);
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
