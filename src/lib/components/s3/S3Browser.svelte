<script lang="ts">
  import { fade } from 'svelte/transition';
  import {
    s3BrowserState,
    s3Breadcrumbs,
    s3NavigationHistory,
    s3NavigationIndex,
    enterFolder,
    navigateUp,
    navigateBack,
    navigateForward,
    navigateToPrefix,
    refreshS3Browser,
    closeS3Browser,
    deleteS3Object,
    getS3PresignedUrl,
  } from '$lib/stores/s3';
  import { formatFileSize, getFileIcon, isPreviewable } from '$lib/api/s3';
  import { success, error as showError } from '$lib/stores/notifications';
  import S3FilePreview from './S3FilePreview.svelte';

  let showPreview = $state(false);
  let previewKey = $state('');
  let contextMenu = $state<{ x: number; y: number; key: string } | null>(null);

  function handleFolderClick(folderName: string) {
    enterFolder(folderName);
  }

  function handleFileClick(key: string) {
    if (isPreviewable(key)) {
      previewKey = key;
      showPreview = true;
    }
  }

  function handleBreadcrumbClick(prefix: string) {
    navigateToPrefix($s3BrowserState.bucket, prefix);
  }

  function handleContextMenu(e: MouseEvent, key: string) {
    e.preventDefault();
    contextMenu = { x: e.clientX, y: e.clientY, key };
  }

  function closeContextMenu() {
    contextMenu = null;
  }

  async function handleCopyUrl(key: string) {
    try {
      const url = await getS3PresignedUrl(key, 3600);
      await navigator.clipboard.writeText(url);
      success('Presigned URL copied to clipboard (valid for 1 hour)');
    } catch (e) {
      showError(`Failed to generate URL: ${e}`);
    }
    closeContextMenu();
  }

  async function handleDelete(key: string) {
    if (!confirm(`Are you sure you want to delete "${key.split('/').pop()}"?`)) {
      closeContextMenu();
      return;
    }

    try {
      await deleteS3Object(key);
      success('Object deleted');
    } catch (e) {
      showError(`Failed to delete: ${e}`);
    }
    closeContextMenu();
  }

  function formatDate(timestamp: number | null): string {
    if (!timestamp) return '-';
    return new Date(timestamp).toLocaleString();
  }

  function getFileName(key: string): string {
    const parts = key.split('/');
    return parts[parts.length - 1] || key;
  }

  // Close context menu when clicking outside
  function handleWindowClick() {
    if (contextMenu) {
      contextMenu = null;
    }
  }

  $effect(() => {
    window.addEventListener('click', handleWindowClick);
    return () => window.removeEventListener('click', handleWindowClick);
  });
</script>

<div class="s3-browser" in:fade={{ duration: 200 }}>
  <!-- Toolbar -->
  <div class="toolbar">
    <div class="nav-buttons">
      <button
        class="nav-btn"
        onclick={navigateBack}
        disabled={$s3NavigationIndex <= 0}
        title="Back"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="15 18 9 12 15 6"></polyline>
        </svg>
      </button>
      <button
        class="nav-btn"
        onclick={navigateForward}
        disabled={$s3NavigationIndex >= $s3NavigationHistory.length - 1}
        title="Forward"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="9 18 15 12 9 6"></polyline>
        </svg>
      </button>
      <button
        class="nav-btn"
        onclick={navigateUp}
        disabled={!$s3BrowserState.prefix}
        title="Up"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="18 15 12 9 6 15"></polyline>
        </svg>
      </button>
      <button
        class="nav-btn"
        onclick={refreshS3Browser}
        disabled={$s3BrowserState.loading}
        title="Refresh"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class:spinning={$s3BrowserState.loading}>
          <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.3"/>
        </svg>
      </button>
    </div>

    <!-- Breadcrumbs -->
    <div class="breadcrumbs">
      {#each $s3Breadcrumbs as crumb, i}
        {#if i > 0}
          <span class="separator">/</span>
        {/if}
        <button
          class="breadcrumb"
          class:active={i === $s3Breadcrumbs.length - 1}
          onclick={() => handleBreadcrumbClick(crumb.prefix)}
        >
          {#if i === 0}
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
            </svg>
          {/if}
          {crumb.name}
        </button>
      {/each}
    </div>

    <button class="close-btn" onclick={closeS3Browser} title="Close">
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="18" y1="6" x2="6" y2="18"></line>
        <line x1="6" y1="6" x2="18" y2="18"></line>
      </svg>
    </button>
  </div>

  <!-- Content -->
  <div class="content">
    {#if $s3BrowserState.loading}
      <div class="loading">
        <div class="spinner"></div>
        <span>Loading...</span>
      </div>
    {:else if $s3BrowserState.error}
      <div class="error">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"></circle>
          <line x1="12" y1="8" x2="12" y2="12"></line>
          <line x1="12" y1="16" x2="12.01" y2="16"></line>
        </svg>
        <p>{$s3BrowserState.error}</p>
        <button onclick={refreshS3Browser}>Retry</button>
      </div>
    {:else if $s3BrowserState.folders.length === 0 && $s3BrowserState.objects.length === 0}
      <div class="empty">
        <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
        </svg>
        <p>This folder is empty</p>
      </div>
    {:else}
      <table class="file-list">
        <thead>
          <tr>
            <th class="name-col">Name</th>
            <th class="size-col">Size</th>
            <th class="date-col">Last Modified</th>
          </tr>
        </thead>
        <tbody>
          <!-- Folders first -->
          {#each $s3BrowserState.folders as folder}
            <tr
              class="folder-row"
              ondblclick={() => handleFolderClick(folder)}
              role="button"
              tabindex="0"
              onkeydown={(e) => e.key === 'Enter' && handleFolderClick(folder)}
            >
              <td class="name-cell">
                <div class="file-icon folder">
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="1">
                    <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
                  </svg>
                </div>
                <span class="file-name">{folder}</span>
              </td>
              <td class="size-cell">-</td>
              <td class="date-cell">-</td>
            </tr>
          {/each}

          <!-- Files -->
          {#each $s3BrowserState.objects as obj}
            <tr
              class="file-row"
              ondblclick={() => handleFileClick(obj.key)}
              oncontextmenu={(e) => handleContextMenu(e, obj.key)}
              role="button"
              tabindex="0"
              onkeydown={(e) => e.key === 'Enter' && handleFileClick(obj.key)}
            >
              <td class="name-cell">
                <div class="file-icon {getFileIcon(obj.key, false)}">
                  {#if getFileIcon(obj.key, false) === 'image'}
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
                      <circle cx="8.5" cy="8.5" r="1.5"></circle>
                      <polyline points="21 15 16 10 5 21"></polyline>
                    </svg>
                  {:else if getFileIcon(obj.key, false) === 'code'}
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <polyline points="16 18 22 12 16 6"></polyline>
                      <polyline points="8 6 2 12 8 18"></polyline>
                    </svg>
                  {:else if getFileIcon(obj.key, false) === 'archive'}
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <polyline points="21 8 21 21 3 21 3 8"></polyline>
                      <rect x="1" y="3" width="22" height="5"></rect>
                      <line x1="10" y1="12" x2="14" y2="12"></line>
                    </svg>
                  {:else}
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                      <polyline points="14 2 14 8 20 8"></polyline>
                    </svg>
                  {/if}
                </div>
                <span class="file-name">{getFileName(obj.key)}</span>
              </td>
              <td class="size-cell">{formatFileSize(obj.size)}</td>
              <td class="date-cell">{formatDate(obj.last_modified)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>

  <!-- Context Menu -->
  {#if contextMenu}
    <div
      class="context-menu"
      style="left: {contextMenu.x}px; top: {contextMenu.y}px"
      onclick={(e) => e.stopPropagation()}
      role="menu"
    >
      {#if isPreviewable(contextMenu.key)}
        <button class="menu-item" onclick={() => { handleFileClick(contextMenu!.key); closeContextMenu(); }}>
          <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
            <circle cx="12" cy="12" r="3"></circle>
          </svg>
          Preview
        </button>
      {/if}
      <button class="menu-item" onclick={() => handleCopyUrl(contextMenu!.key)}>
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
          <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
        </svg>
        Copy Presigned URL
      </button>
      <div class="menu-divider"></div>
      <button class="menu-item danger" onclick={() => handleDelete(contextMenu!.key)}>
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="3 6 5 6 21 6"></polyline>
          <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
        </svg>
        Delete
      </button>
    </div>
  {/if}

  <!-- File Preview Modal -->
  {#if showPreview}
    <S3FilePreview
      key={previewKey}
      onClose={() => { showPreview = false; previewKey = ''; }}
    />
  {/if}
</div>

<style>
  .s3-browser {
    flex: 1;
    display: flex;
    flex-direction: column;
    background-color: var(--color-bg-primary);
    overflow: hidden;
  }

  .toolbar {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 12px;
    background-color: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
  }

  .nav-buttons {
    display: flex;
    gap: 4px;
  }

  .nav-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: 6px;
    color: var(--color-text-secondary);
    transition: all 150ms ease;
  }

  .nav-btn:hover:not(:disabled) {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .nav-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .spinning {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .breadcrumbs {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 4px;
    overflow-x: auto;
    padding: 4px 8px;
    background-color: var(--color-bg-tertiary);
    border-radius: 6px;
  }

  .separator {
    color: var(--color-text-muted);
    font-size: 12px;
  }

  .breadcrumb {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 13px;
    color: var(--color-text-secondary);
    white-space: nowrap;
    transition: all 150ms ease;
  }

  .breadcrumb:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .breadcrumb.active {
    color: var(--color-text-primary);
    font-weight: 500;
  }

  .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: 6px;
    color: var(--color-text-muted);
    transition: all 150ms ease;
  }

  .close-btn:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .content {
    flex: 1;
    overflow: auto;
  }

  .loading,
  .error,
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 48px;
    color: var(--color-text-muted);
    text-align: center;
  }

  .error {
    color: var(--color-error);
  }

  .error button {
    padding: 6px 16px;
    border-radius: 6px;
    background-color: var(--color-accent);
    color: white;
    font-size: 13px;
    transition: background-color 150ms ease;
  }

  .error button:hover {
    background-color: var(--color-accent-hover);
  }

  .spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  .file-list {
    width: 100%;
    border-collapse: collapse;
    font-size: 13px;
  }

  .file-list th {
    position: sticky;
    top: 0;
    padding: 8px 12px;
    text-align: left;
    font-weight: 500;
    color: var(--color-text-secondary);
    background-color: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
  }

  .file-list td {
    padding: 8px 12px;
    border-bottom: 1px solid var(--color-border-subtle);
  }

  .folder-row,
  .file-row {
    cursor: pointer;
    transition: background-color 150ms ease;
  }

  .folder-row:hover,
  .file-row:hover {
    background-color: var(--color-bg-hover);
  }

  .name-col { width: 50%; }
  .size-col { width: 15%; }
  .date-col { width: 35%; }

  .name-cell {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .file-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    color: var(--color-text-muted);
  }

  .file-icon.folder { color: #f59e0b; }
  .file-icon.image { color: #ec4899; }
  .file-icon.code { color: #10b981; }
  .file-icon.archive { color: #8b5cf6; }
  .file-icon.data { color: #3b82f6; }
  .file-icon.config { color: #6b7280; }

  .file-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .size-cell,
  .date-cell {
    color: var(--color-text-secondary);
    white-space: nowrap;
  }

  .context-menu {
    position: fixed;
    min-width: 180px;
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    padding: 4px;
    z-index: 1000;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 12px;
    font-size: 13px;
    color: var(--color-text-primary);
    border-radius: 4px;
    text-align: left;
    transition: background-color 150ms ease;
  }

  .menu-item:hover {
    background-color: var(--color-bg-hover);
  }

  .menu-item.danger {
    color: var(--color-error);
  }

  .menu-divider {
    height: 1px;
    background-color: var(--color-border);
    margin: 4px 0;
  }
</style>
