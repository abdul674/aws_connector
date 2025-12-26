<script lang="ts">
  import {
    profiles,
    regions,
    selectedProfile,
    selectedRegion,
    profilesLoading,
    selectProfile,
  } from '$lib/stores/profiles';
  import AddProfileModal from '$lib/components/aws/AddProfileModal.svelte';

  interface Props {
    onRefresh?: () => void;
  }

  let { onRefresh }: Props = $props();

  let isRefreshing = $state(false);
  let showAddProfileModal = $state(false);

  async function handleRefresh() {
    if (isRefreshing || !onRefresh) return;
    isRefreshing = true;
    try {
      await onRefresh();
    } finally {
      isRefreshing = false;
    }
  }

  async function handleProfileChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    await selectProfile(target.value);
  }

  function handleRegionChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    selectedRegion.set(target.value);
  }
</script>

<header class="header">
  <div class="header-left">
    <div class="profile-selector">
      <label class="selector-label" for="profile-select">Profile</label>
      <select
        id="profile-select"
        class="selector"
        value={$selectedProfile}
        onchange={handleProfileChange}
        disabled={$profilesLoading}
      >
        {#if $profiles.length === 0}
          <option value="">No profiles found</option>
        {:else}
          {#each $profiles as profile (profile.name)}
            <option value={profile.name}>
              {profile.name}
              {#if profile.region}({profile.region}){/if}
            </option>
          {/each}
        {/if}
      </select>
      <button
        class="add-profile-btn"
        title="Add new profile"
        onclick={() => showAddProfileModal = true}
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="12" y1="5" x2="12" y2="19"></line>
          <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
      </button>
    </div>

    <div class="region-selector">
      <label class="selector-label" for="region-select">Region</label>
      <select
        id="region-select"
        class="selector"
        value={$selectedRegion}
        onchange={handleRegionChange}
      >
        {#if $regions.length === 0}
          <option value="us-east-1">us-east-1</option>
          <option value="us-west-2">us-west-2</option>
          <option value="eu-west-1">eu-west-1</option>
        {:else}
          {#each $regions as region (region.code)}
            <option value={region.code}>{region.code}</option>
          {/each}
        {/if}
      </select>
    </div>
  </div>

  <div class="header-right">
    <button
      class="header-btn"
      title="Refresh resources"
      onclick={handleRefresh}
      disabled={isRefreshing}
    >
      <svg
        class="refresh-icon"
        class:spinning={isRefreshing}
        xmlns="http://www.w3.org/2000/svg"
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
        <path d="M3 3v5h5"></path>
        <path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"></path>
        <path d="M16 21h5v-5"></path>
      </svg>
    </button>

    <div class="connection-status">
      {#if $profilesLoading}
        <span class="status-dot connecting"></span>
        <span class="status-text">Loading...</span>
      {:else if $profiles.length > 0}
        <span class="status-dot online"></span>
        <span class="status-text">Ready</span>
      {:else}
        <span class="status-dot offline"></span>
        <span class="status-text">No profiles</span>
      {/if}
    </div>
  </div>
</header>

<AddProfileModal
  open={showAddProfileModal}
  onClose={() => showAddProfileModal = false}
/>

<style>
  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    background-color: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
    min-height: 48px;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .header-right {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .profile-selector,
  .region-selector {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .selector-label {
    font-size: 12px;
    color: var(--color-text-muted);
    font-weight: 500;
  }

  .selector {
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    padding: 6px 32px 6px 12px;
    font-size: 13px;
    color: var(--color-text-primary);
    cursor: pointer;
    min-width: 130px;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23a3a3a3' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 10px center;
    transition: border-color 150ms ease, background-color 150ms ease;
  }

  .selector:hover:not(:disabled) {
    border-color: var(--color-text-muted);
    background-color: var(--color-bg-hover);
  }

  .selector:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: 0 0 0 2px var(--color-accent-subtle);
  }

  .selector:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .selector option {
    background-color: var(--color-bg-tertiary);
    color: var(--color-text-primary);
    padding: 8px 12px;
  }

  .add-profile-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: 6px;
    color: var(--color-text-secondary);
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    transition: all 150ms ease;
  }

  .add-profile-btn:hover {
    background-color: var(--color-accent);
    border-color: var(--color-accent);
    color: white;
  }

  .header-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: 6px;
    color: var(--color-text-secondary);
    transition: background-color 150ms ease, color 150ms ease;
  }

  .header-btn:hover:not(:disabled) {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .header-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .refresh-icon {
    transition: transform 150ms ease;
  }

  .refresh-icon.spinning {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .connection-status {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    background-color: var(--color-bg-tertiary);
    border-radius: 4px;
  }

  .status-text {
    font-size: 12px;
    color: var(--color-text-secondary);
  }
</style>
