<script lang="ts">
  import TerminalTabs from './TerminalTabs.svelte';
  import TerminalPane from './TerminalPane.svelte';
  import { sessionList, activeSessionId, hasActiveSessions } from '$lib/stores/terminals';
</script>

<div class="terminal-container">
  {#if $hasActiveSessions}
    <TerminalTabs />
    <div class="terminal-panes">
      {#each $sessionList as session (session.id)}
        <TerminalPane
          {session}
          active={$activeSessionId === session.id}
        />
      {/each}
    </div>
  {:else}
    <div class="empty-state">
      <div class="empty-icon">
        <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <polyline points="4 17 10 11 4 5"></polyline>
          <line x1="12" y1="19" x2="20" y2="19"></line>
        </svg>
      </div>
      <p class="empty-title">No active terminal sessions</p>
      <p class="empty-hint">Connect to a container or instance from the sidebar to start a session</p>
    </div>
  {/if}
</div>

<style>
  .terminal-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background-color: var(--terminal-bg, #0d0d0d);
  }

  .terminal-panes {
    flex: 1;
    display: flex;
    overflow: hidden;
    position: relative;
  }

  .empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--color-text-muted);
    padding: 32px;
    text-align: center;
  }

  .empty-icon {
    margin-bottom: 16px;
    opacity: 0.5;
  }

  .empty-title {
    font-size: 16px;
    font-weight: 500;
    color: var(--color-text-secondary);
    margin: 0 0 8px;
  }

  .empty-hint {
    font-size: 13px;
    margin: 0;
    max-width: 300px;
  }
</style>
