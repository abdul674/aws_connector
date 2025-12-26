<script lang="ts">
  import { settings, type AppSettings } from '$lib/stores/settings';
  import { listAwsRegions } from '$lib/api/aws';
  import type { AwsRegion } from '$lib/types/aws';

  interface Props {
    open: boolean;
    onClose: () => void;
  }

  let { open, onClose }: Props = $props();

  // Local state for form
  let localSettings = $state<AppSettings>({ ...$settings });
  let regions = $state<AwsRegion[]>([]);
  let hasChanges = $state(false);

  $effect(() => {
    if (open) {
      // Reset local state when opening
      localSettings = { ...$settings };
      hasChanges = false;
      loadRegions();
    }
  });

  // Track changes
  $effect(() => {
    const current = $settings;
    hasChanges = JSON.stringify(localSettings) !== JSON.stringify(current);
  });

  async function loadRegions() {
    try {
      regions = await listAwsRegions();
    } catch (e) {
      console.error('Failed to load regions:', e);
    }
  }

  function handleSave() {
    settings.setSettings(localSettings);
    onClose();
  }

  function handleReset() {
    settings.reset();
    localSettings = { ...$settings };
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      onClose();
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      onClose();
    }
  }

  const shellOptions = [
    { value: '/bin/sh', label: '/bin/sh (Most compatible)' },
    { value: '/bin/bash', label: '/bin/bash (More features)' },
    { value: '/bin/zsh', label: '/bin/zsh' },
  ];

  const fontSizeOptions = [10, 11, 12, 13, 14, 15, 16, 18, 20];

  const refreshIntervalOptions = [
    { value: 0, label: 'Disabled' },
    { value: 30, label: '30 seconds' },
    { value: 60, label: '1 minute' },
    { value: 300, label: '5 minutes' },
    { value: 600, label: '10 minutes' },
  ];
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="modal-backdrop"
    onclick={handleBackdropClick}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div class="modal" role="document">
      <div class="modal-header">
        <h2>Settings</h2>
        <button class="close-btn" onclick={onClose} aria-label="Close">
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <!-- Appearance Section -->
        <section class="settings-section">
          <h3 class="section-title">Appearance</h3>

          <div class="setting-row">
            <div class="setting-info">
              <label for="terminal-font-size">Terminal Font Size</label>
              <span class="setting-desc">Size of text in terminal windows</span>
            </div>
            <select id="terminal-font-size" bind:value={localSettings.terminalFontSize}>
              {#each fontSizeOptions as size}
                <option value={size}>{size}px</option>
              {/each}
            </select>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <label for="sidebar-width">Default Sidebar Width</label>
              <span class="setting-desc">Width of the sidebar in pixels</span>
            </div>
            <input
              type="number"
              id="sidebar-width"
              bind:value={localSettings.sidebarWidth}
              min="200"
              max="500"
              step="10"
            />
          </div>
        </section>

        <!-- Defaults Section -->
        <section class="settings-section">
          <h3 class="section-title">Defaults</h3>

          <div class="setting-row">
            <div class="setting-info">
              <label for="default-region">Default AWS Region</label>
              <span class="setting-desc">Region to use when no profile is selected</span>
            </div>
            <select id="default-region" bind:value={localSettings.defaultRegion}>
              {#each regions as region (region.code)}
                <option value={region.code}>{region.code}</option>
              {/each}
            </select>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <label for="default-shell">Default Shell</label>
              <span class="setting-desc">Shell to use for ECS Exec connections</span>
            </div>
            <select id="default-shell" bind:value={localSettings.defaultShell}>
              {#each shellOptions as shell}
                <option value={shell.value}>{shell.label}</option>
              {/each}
            </select>
          </div>
        </section>

        <!-- Behavior Section -->
        <section class="settings-section">
          <h3 class="section-title">Behavior</h3>

          <div class="setting-row">
            <div class="setting-info">
              <label for="confirm-close">Confirm Before Closing</label>
              <span class="setting-desc">Ask before closing terminal sessions</span>
            </div>
            <label class="toggle">
              <input type="checkbox" bind:checked={localSettings.confirmBeforeClose} />
              <span class="toggle-slider"></span>
            </label>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <label for="auto-refresh">Auto Refresh Resources</label>
              <span class="setting-desc">Automatically refresh resource list</span>
            </div>
            <select id="auto-refresh" bind:value={localSettings.autoRefreshInterval}>
              {#each refreshIntervalOptions as option}
                <option value={option.value}>{option.label}</option>
              {/each}
            </select>
          </div>
        </section>

        <!-- Keyboard Shortcuts Info -->
        <section class="settings-section">
          <h3 class="section-title">Keyboard Shortcuts</h3>
          <div class="shortcuts-list">
            <div class="shortcut-item">
              <span class="shortcut-keys"><kbd>Cmd</kbd> + <kbd>W</kbd></span>
              <span class="shortcut-desc">Close active terminal</span>
            </div>
            <div class="shortcut-item">
              <span class="shortcut-keys"><kbd>Cmd</kbd> + <kbd>1-9</kbd></span>
              <span class="shortcut-desc">Switch to terminal tab</span>
            </div>
            <div class="shortcut-item">
              <span class="shortcut-keys"><kbd>Cmd</kbd> + <kbd>[</kbd></span>
              <span class="shortcut-desc">Previous terminal</span>
            </div>
            <div class="shortcut-item">
              <span class="shortcut-keys"><kbd>Cmd</kbd> + <kbd>]</kbd></span>
              <span class="shortcut-desc">Next terminal</span>
            </div>
            <div class="shortcut-item">
              <span class="shortcut-keys"><kbd>Cmd</kbd> + <kbd>R</kbd></span>
              <span class="shortcut-desc">Refresh resources</span>
            </div>
          </div>
        </section>
      </div>

      <div class="modal-footer">
        <button type="button" class="btn-reset" onclick={handleReset}>
          Reset to Defaults
        </button>
        <div class="footer-right">
          <button type="button" class="btn-secondary" onclick={onClose}>
            Cancel
          </button>
          <button type="button" class="btn-primary" onclick={handleSave} disabled={!hasChanges}>
            Save Changes
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background-color: var(--color-bg-secondary);
    border-radius: 12px;
    border: 1px solid var(--color-border);
    width: 100%;
    max-width: 520px;
    max-height: 85vh;
    overflow-y: auto;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3);
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--color-border);
    position: sticky;
    top: 0;
    background-color: var(--color-bg-secondary);
    z-index: 1;
  }

  .modal-header h2 {
    font-size: 18px;
    font-weight: 600;
    color: var(--color-text-primary);
    margin: 0;
  }

  .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: 6px;
    color: var(--color-text-muted);
    transition: background-color 150ms ease, color 150ms ease;
  }

  .close-btn:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .modal-body {
    padding: 20px;
  }

  .settings-section {
    margin-bottom: 24px;
  }

  .settings-section:last-child {
    margin-bottom: 0;
  }

  .section-title {
    font-size: 12px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--color-text-muted);
    margin: 0 0 12px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--color-border-subtle);
  }

  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 12px 0;
  }

  .setting-row:not(:last-child) {
    border-bottom: 1px solid var(--color-border-subtle);
  }

  .setting-info {
    flex: 1;
  }

  .setting-info label {
    display: block;
    font-size: 14px;
    font-weight: 500;
    color: var(--color-text-primary);
    margin-bottom: 2px;
  }

  .setting-desc {
    font-size: 12px;
    color: var(--color-text-muted);
  }

  .setting-row select,
  .setting-row input[type="number"] {
    width: 140px;
    padding: 8px 12px;
    font-size: 13px;
    color: var(--color-text-primary);
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
  }

  .setting-row select {
    appearance: none;
    -webkit-appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23a3a3a3' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 10px center;
    padding-right: 32px;
  }

  .setting-row input:focus,
  .setting-row select:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  /* Toggle switch */
  .toggle {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
    cursor: pointer;
  }

  .toggle input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .toggle-slider {
    position: absolute;
    inset: 0;
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: 12px;
    transition: all 150ms ease;
  }

  .toggle-slider::before {
    content: '';
    position: absolute;
    height: 18px;
    width: 18px;
    left: 2px;
    bottom: 2px;
    background-color: var(--color-text-muted);
    border-radius: 50%;
    transition: all 150ms ease;
  }

  .toggle input:checked + .toggle-slider {
    background-color: var(--color-accent);
    border-color: var(--color-accent);
  }

  .toggle input:checked + .toggle-slider::before {
    transform: translateX(20px);
    background-color: white;
  }

  /* Keyboard shortcuts */
  .shortcuts-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .shortcut-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    background-color: var(--color-bg-tertiary);
    border-radius: 6px;
  }

  .shortcut-keys {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  kbd {
    display: inline-block;
    padding: 2px 6px;
    font-size: 11px;
    font-family: inherit;
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 4px;
    color: var(--color-text-secondary);
  }

  .shortcut-desc {
    font-size: 13px;
    color: var(--color-text-secondary);
  }

  .modal-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-top: 1px solid var(--color-border);
    position: sticky;
    bottom: 0;
    background-color: var(--color-bg-secondary);
  }

  .footer-right {
    display: flex;
    gap: 12px;
  }

  .btn-reset,
  .btn-secondary,
  .btn-primary {
    padding: 10px 16px;
    font-size: 13px;
    font-weight: 500;
    border-radius: 6px;
    transition: all 150ms ease;
  }

  .btn-reset {
    color: var(--color-text-muted);
    background: none;
  }

  .btn-reset:hover {
    color: var(--color-text-primary);
  }

  .btn-secondary {
    color: var(--color-text-secondary);
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
  }

  .btn-secondary:hover {
    color: var(--color-text-primary);
    background-color: var(--color-bg-hover);
  }

  .btn-primary {
    color: white;
    background-color: var(--color-accent);
    border: 1px solid var(--color-accent);
  }

  .btn-primary:hover:not(:disabled) {
    background-color: var(--color-accent-hover);
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
