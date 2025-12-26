<script lang="ts">
  import LogTailTabs from './LogTailTabs.svelte';
  import LogTailPane from './LogTailPane.svelte';
  import {
    logTailSessions,
    activeLogSessionId,
    hasActiveLogSessions,
  } from '$lib/stores/logs';
</script>

<div class="log-viewer">
  {#if $hasActiveLogSessions}
    <LogTailTabs />
    <div class="log-panes">
      {#each [...$logTailSessions.entries()] as [sessionId, state] (sessionId)}
        <LogTailPane
          session={state}
          active={$activeLogSessionId === sessionId}
        />
      {/each}
    </div>
  {:else}
    <div class="empty-state">
      <div class="empty-icon">
        <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
          <polyline points="14 2 14 8 20 8"></polyline>
          <line x1="16" y1="13" x2="8" y2="13"></line>
          <line x1="16" y1="17" x2="8" y2="17"></line>
        </svg>
      </div>
      <p class="empty-title">No active log tails</p>
      <p class="empty-hint">Click the tail button on a log group to start streaming logs</p>
    </div>
  {/if}
</div>

<style>
  .log-viewer {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background-color: var(--terminal-bg, #0d0d0d);
  }

  .log-panes {
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
    color: #f97316;
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
