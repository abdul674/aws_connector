<script lang="ts">
  import ResourceTree from '$lib/components/aws/ResourceTree.svelte';
  import { resources } from '$lib/stores/resources';
  import type { Ec2Instance } from '$lib/types/aws';
  import type { LogGroup } from '$lib/types/logs';
  import type { S3Bucket } from '$lib/types/s3';

  interface Props {
    collapsed?: boolean;
    width?: number;
    onConnect?: (type: 'ecs' | 'ec2', data: unknown) => void;
    onPortForward?: (instance: Ec2Instance) => void;
    onLogTail?: (logGroup: LogGroup) => void;
    onS3Browse?: (bucket: S3Bucket) => void;
    onResize?: (width: number) => void;
    onSettings?: () => void;
  }

  let { collapsed = false, width = 280, onConnect, onPortForward, onLogTail, onS3Browse, onResize, onSettings }: Props = $props();

  let isResizing = $state(false);

  function handleMouseDown(e: MouseEvent) {
    e.preventDefault();
    isResizing = true;
    document.body.classList.add('no-select');

    const handleMouseMove = (e: MouseEvent) => {
      const newWidth = Math.max(200, Math.min(500, e.clientX));
      onResize?.(newWidth);
    };

    const handleMouseUp = () => {
      isResizing = false;
      document.body.classList.remove('no-select');
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
  }
</script>

<aside class="sidebar" class:collapsed class:resizing={isResizing} style="width: {collapsed ? 'var(--sidebar-collapsed-width)' : `${width}px`}">
  <div class="sidebar-header">
    <div class="logo">
      {#if !collapsed}
        <span class="logo-text">AWS Connector</span>
      {:else}
        <span class="logo-icon">AC</span>
      {/if}
    </div>
  </div>

  <div class="sidebar-content">
    {#if collapsed}
      <div class="collapsed-hint">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
          <line x1="8" y1="21" x2="16" y2="21"></line>
          <line x1="12" y1="17" x2="12" y2="21"></line>
        </svg>
      </div>
    {:else}
      <ResourceTree {onConnect} {onPortForward} {onLogTail} {onS3Browse} />
    {/if}
  </div>

  <div class="sidebar-footer">
    <button class="sidebar-btn" title="Settings" onclick={() => onSettings?.()}>
      <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="3"></circle>
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
      </svg>
      {#if !collapsed}
        <span>Settings</span>
      {/if}
    </button>
  </div>

  {#if !collapsed}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="resize-handle"
      onmousedown={handleMouseDown}
    ></div>
  {/if}
</aside>

<style>
  .sidebar {
    position: relative;
    display: flex;
    flex-direction: column;
    height: 100%;
    background-color: var(--color-bg-secondary);
    border-right: 1px solid var(--color-border);
    flex-shrink: 0;
  }

  .sidebar:not(.resizing) {
    transition: width 200ms ease;
  }

  .resize-handle {
    position: absolute;
    top: 0;
    right: -3px;
    width: 6px;
    height: 100%;
    cursor: col-resize;
    z-index: 10;
  }

  .resize-handle:hover,
  .sidebar.resizing .resize-handle {
    background-color: var(--color-accent);
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    padding: 16px;
    border-bottom: 1px solid var(--color-border);
    min-height: 56px;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .logo-text {
    font-size: 16px;
    font-weight: 600;
    color: var(--color-text-primary);
    white-space: nowrap;
  }

  .logo-icon {
    font-size: 14px;
    font-weight: 700;
    color: var(--color-accent);
  }

  .sidebar-content {
    flex: 1;
    overflow-y: auto;
    padding: 8px 0;
  }

  .collapsed-hint {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    color: var(--color-text-muted);
  }

  .sidebar-footer {
    padding: 12px;
    border-top: 1px solid var(--color-border);
  }

  .sidebar-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 12px;
    border-radius: 6px;
    color: var(--color-text-secondary);
    transition: background-color 150ms ease, color 150ms ease;
  }

  .sidebar-btn:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .collapsed .sidebar-btn {
    justify-content: center;
    padding: 8px;
  }

  .collapsed .sidebar-btn span {
    display: none;
  }
</style>
