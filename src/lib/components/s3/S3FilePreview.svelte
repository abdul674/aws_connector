<script lang="ts">
  import { onMount } from 'svelte';
  import { getS3ObjectContent } from '$lib/stores/s3';
  import { formatFileSize, getFileExtension } from '$lib/api/s3';

  interface Props {
    key: string;
    onClose: () => void;
  }

  let { key, onClose }: Props = $props();

  let content = $state<string>('');
  let loading = $state(true);
  let error = $state<string | null>(null);
  let lineNumbers = $state<number[]>([]);

  const MAX_PREVIEW_BYTES = 512 * 1024; // 512KB max preview

  onMount(async () => {
    try {
      content = await getS3ObjectContent(key, MAX_PREVIEW_BYTES);
      lineNumbers = content.split('\n').map((_, i) => i + 1);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  });

  function getFileName(): string {
    return key.split('/').pop() || key;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      onClose();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="preview-overlay" onclick={onClose}>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="preview-modal" onclick={(e) => e.stopPropagation()}>
    <div class="preview-header">
      <div class="file-info">
        <span class="file-name">{getFileName()}</span>
        <span class="file-ext">.{getFileExtension(key)}</span>
      </div>
      <button class="close-btn" onclick={onClose}>
        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
    </div>

    <div class="preview-content">
      {#if loading}
        <div class="loading">
          <div class="spinner"></div>
          <span>Loading preview...</span>
        </div>
      {:else if error}
        <div class="error">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="12" y1="8" x2="12" y2="12"></line>
            <line x1="12" y1="16" x2="12.01" y2="16"></line>
          </svg>
          <p>{error}</p>
        </div>
      {:else}
        <div class="code-container">
          <div class="line-numbers">
            {#each lineNumbers as num}
              <span class="line-num">{num}</span>
            {/each}
          </div>
          <pre class="code-content">{content}</pre>
        </div>
      {/if}
    </div>

    {#if content.length >= MAX_PREVIEW_BYTES}
      <div class="preview-footer">
        <span class="truncation-notice">
          Preview truncated at {formatFileSize(MAX_PREVIEW_BYTES)}
        </span>
      </div>
    {/if}
  </div>
</div>

<style>
  .preview-overlay {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 32px;
  }

  .preview-modal {
    display: flex;
    flex-direction: column;
    width: 100%;
    max-width: 900px;
    max-height: 80vh;
    background-color: var(--color-bg-secondary);
    border-radius: 12px;
    border: 1px solid var(--color-border);
    overflow: hidden;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.4);
  }

  .preview-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    background-color: var(--color-bg-tertiary);
    border-bottom: 1px solid var(--color-border);
  }

  .file-info {
    display: flex;
    align-items: baseline;
    gap: 4px;
  }

  .file-name {
    font-size: 14px;
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .file-ext {
    font-size: 12px;
    color: var(--color-text-muted);
  }

  .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: 6px;
    color: var(--color-text-muted);
    transition: all 150ms ease;
  }

  .close-btn:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .preview-content {
    flex: 1;
    overflow: auto;
    background-color: var(--terminal-bg, #0d0d0d);
  }

  .loading,
  .error {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 48px;
    color: var(--color-text-muted);
  }

  .error {
    color: var(--color-error);
  }

  .spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .code-container {
    display: flex;
    font-family: 'SF Mono', 'Monaco', 'Consolas', monospace;
    font-size: 13px;
    line-height: 1.6;
  }

  .line-numbers {
    display: flex;
    flex-direction: column;
    padding: 12px 12px 12px 16px;
    text-align: right;
    background-color: rgba(255, 255, 255, 0.03);
    border-right: 1px solid var(--color-border);
    user-select: none;
  }

  .line-num {
    color: var(--color-text-muted);
    opacity: 0.5;
  }

  .code-content {
    flex: 1;
    margin: 0;
    padding: 12px 16px;
    color: #e0e0e0;
    white-space: pre;
    overflow-x: auto;
  }

  .preview-footer {
    padding: 8px 16px;
    background-color: var(--color-bg-tertiary);
    border-top: 1px solid var(--color-border);
  }

  .truncation-notice {
    font-size: 12px;
    color: var(--color-warning);
  }
</style>
