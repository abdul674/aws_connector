<script lang="ts">
  import { onMount } from 'svelte';
  import type { LogTailSessionState } from '$lib/stores/logs';

  interface Props {
    session: LogTailSessionState;
    active: boolean;
  }

  let { session, active }: Props = $props();

  let containerEl: HTMLDivElement;
  let autoScroll = $state(true);

  // Auto-scroll to bottom when new logs arrive
  $effect(() => {
    if (active && autoScroll && session.logs.length > 0 && containerEl) {
      containerEl.scrollTop = containerEl.scrollHeight;
    }
  });

  function handleScroll() {
    if (!containerEl) return;
    const isAtBottom = containerEl.scrollHeight - containerEl.scrollTop <= containerEl.clientHeight + 50;
    autoScroll = isAtBottom;
  }

  function formatTimestamp(ts: number): string {
    const date = new Date(ts);
    return date.toISOString().replace('T', ' ').replace('Z', '');
  }

  function getLogLevelColor(message: string): string {
    const upper = message.toUpperCase();
    if (upper.includes('ERROR') || upper.includes('FATAL') || upper.includes('CRITICAL')) {
      return 'var(--color-error)';
    }
    if (upper.includes('WARN')) {
      return 'var(--color-warning)';
    }
    if (upper.includes('DEBUG') || upper.includes('TRACE')) {
      return 'var(--color-text-muted)';
    }
    return 'var(--color-text-primary)';
  }
</script>

<div
  class="log-pane"
  class:active
  bind:this={containerEl}
  onscroll={handleScroll}
>
  {#if session.logs.length === 0}
    <div class="empty-state">
      <div class="spinner"></div>
      <span>Waiting for logs...</span>
    </div>
  {:else}
    <div class="log-content">
      {#each session.logs as event (event.timestamp + event.message.slice(0, 50))}
        <div class="log-line">
          <span class="log-timestamp">{formatTimestamp(event.timestamp)}</span>
          <span class="log-stream" title={event.log_stream_name}>
            {event.log_stream_name.split('/').pop() ?? event.log_stream_name}
          </span>
          <span class="log-message" style="color: {getLogLevelColor(event.message)}">
            {event.message}
          </span>
        </div>
      {/each}
    </div>
  {/if}

  {#if !autoScroll && session.logs.length > 0}
    <button
      class="scroll-to-bottom"
      onclick={() => {
        autoScroll = true;
        if (containerEl) containerEl.scrollTop = containerEl.scrollHeight;
      }}
    >
      Scroll to bottom
    </button>
  {/if}
</div>

<style>
  .log-pane {
    flex: 1;
    display: none;
    background-color: #0d0d0d;
    overflow-y: auto;
    overflow-x: hidden;
    position: relative;
  }

  .log-pane.active {
    display: block;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 12px;
    color: var(--color-text-muted);
  }

  .spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .log-content {
    padding: 8px;
    font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
    font-size: 12px;
    line-height: 1.5;
  }

  .log-line {
    display: flex;
    gap: 8px;
    padding: 2px 4px;
    border-radius: 2px;
  }

  .log-line:hover {
    background-color: rgba(255, 255, 255, 0.05);
  }

  .log-timestamp {
    color: var(--color-text-muted);
    white-space: nowrap;
    flex-shrink: 0;
    font-size: 11px;
  }

  .log-stream {
    color: var(--color-accent);
    white-space: nowrap;
    flex-shrink: 0;
    max-width: 150px;
    overflow: hidden;
    text-overflow: ellipsis;
    font-size: 11px;
  }

  .log-message {
    white-space: pre-wrap;
    word-break: break-all;
    flex: 1;
  }

  .scroll-to-bottom {
    position: absolute;
    bottom: 16px;
    left: 50%;
    transform: translateX(-50%);
    padding: 6px 16px;
    background-color: var(--color-accent);
    color: white;
    border-radius: 16px;
    font-size: 12px;
    font-weight: 500;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
    transition: background-color 150ms ease;
  }

  .scroll-to-bottom:hover {
    background-color: var(--color-accent-hover);
  }
</style>
