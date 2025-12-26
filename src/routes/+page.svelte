<script lang="ts">
  import { onMount } from 'svelte';
  import { Sidebar, Header, MainContent, StatusBar, SettingsModal, ToastContainer } from '$lib/components/layout';
  import PortForwardingModal from '$lib/components/aws/PortForwardingModal.svelte';
  import {
    initializeAws,
    loadProfiles,
    selectedProfile,
    selectedRegion,
  } from '$lib/stores/profiles';
  import { loadResources, clearResources } from '$lib/stores/resources';
  import {
    createSession,
    sessionCount,
    sessionList,
    activeSessionId,
    closeSession,
    switchToSession,
  } from '$lib/stores/terminals';
  import {
    loadLogGroups,
    clearLogGroups,
    createLogTailSession,
    hasActiveLogSessions,
  } from '$lib/stores/logs';
  import {
    loadS3Buckets,
    clearS3Buckets,
    openS3Bucket,
  } from '$lib/stores/s3';
  import { settings } from '$lib/stores/settings';
  import { error as showError, success as showSuccess } from '$lib/stores/notifications';
  import type { EcsCluster, EcsService, EcsTask, EcsContainer, Ec2Instance } from '$lib/types/aws';
  import type { LogGroup } from '$lib/types/logs';
  import type { S3Bucket } from '$lib/types/s3';
  import type { SessionType } from '$lib/types/terminal';

  // State
  let sidebarCollapsed = $state(false);
  let sidebarWidth = $state($settings.sidebarWidth);
  let showShellSelector = $state(false);
  let pendingEcsConnect: { cluster: string; task: string; container: string } | null = $state(null);
  let selectedShell = $state($settings.defaultShell);

  // Port forwarding state
  let showPortForwardingModal = $state(false);
  let pendingPortForwardInstance: Ec2Instance | null = $state(null);

  // Settings state
  let showSettingsModal = $state(false);

  onMount(async () => {
    await initializeAws();
  });

  // Keyboard shortcuts
  function handleGlobalKeydown(e: KeyboardEvent) {
    const isMod = e.metaKey || e.ctrlKey;

    // Don't handle shortcuts when typing in inputs
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
      return;
    }

    // Cmd/Ctrl + W: Close active terminal
    if (isMod && e.key === 'w') {
      e.preventDefault();
      const activeId = $activeSessionId;
      if (activeId) {
        closeSession(activeId);
      }
    }

    // Cmd/Ctrl + R: Refresh resources
    if (isMod && e.key === 'r') {
      e.preventDefault();
      handleRefresh();
    }

    // Cmd/Ctrl + [: Previous terminal
    if (isMod && e.key === '[') {
      e.preventDefault();
      navigateTerminal(-1);
    }

    // Cmd/Ctrl + ]: Next terminal
    if (isMod && e.key === ']') {
      e.preventDefault();
      navigateTerminal(1);
    }

    // Cmd/Ctrl + 1-9: Switch to terminal tab
    if (isMod && e.key >= '1' && e.key <= '9') {
      e.preventDefault();
      const index = parseInt(e.key) - 1;
      const sessions = $sessionList;
      if (index < sessions.length) {
        switchToSession(sessions[index].id);
      }
    }

    // Cmd/Ctrl + ,: Open settings
    if (isMod && e.key === ',') {
      e.preventDefault();
      showSettingsModal = true;
    }
  }

  function navigateTerminal(direction: number) {
    const sessions = $sessionList;
    const activeId = $activeSessionId;
    if (sessions.length === 0) return;

    const currentIndex = sessions.findIndex((s) => s.id === activeId);
    let newIndex = currentIndex + direction;

    // Wrap around
    if (newIndex < 0) newIndex = sessions.length - 1;
    if (newIndex >= sessions.length) newIndex = 0;

    switchToSession(sessions[newIndex].id);
  }

  // Save sidebar width to settings when resizing stops
  function handleSidebarResize(width: number) {
    sidebarWidth = width;
    settings.setSetting('sidebarWidth', width);
  }

  // Reload resources when profile or region changes
  $effect(() => {
    const profile = $selectedProfile;
    const region = $selectedRegion;

    // Clear existing resources when profile/region changes
    clearResources();
    clearLogGroups();
    clearS3Buckets();
  });

  async function handleRefresh() {
    console.log('Refreshing resources...');
    await loadProfiles();
    await Promise.all([loadResources(), loadLogGroups(), loadS3Buckets()]);
  }

  function handleConnect(type: 'ecs' | 'ec2', data: unknown) {
    if (type === 'ecs') {
      const { cluster, task, container } = data as {
        cluster: EcsCluster;
        service: EcsService;
        task: EcsTask;
        container: EcsContainer;
      };

      // Store pending connection and show shell selector
      pendingEcsConnect = {
        cluster: cluster.name,
        task: task.arn.split('/').pop() ?? task.arn,
        container: container.name,
      };
      showShellSelector = true;
    } else if (type === 'ec2') {
      const instance = data as Ec2Instance;
      connectToEc2(instance);
    }
  }

  async function connectToEcs() {
    if (!pendingEcsConnect) return;

    const sessionType: SessionType = {
      type: 'ecs_exec',
      cluster: pendingEcsConnect.cluster,
      task: pendingEcsConnect.task,
      container: pendingEcsConnect.container,
      profile: $selectedProfile,
      region: $selectedRegion,
    };

    try {
      await createSession({
        session_type: sessionType,
        shell: selectedShell,
      });
      showSuccess('Connected to ECS container');
    } catch (e) {
      console.error('Failed to create ECS session:', e);
      showError(`Failed to connect: ${e}`);
    }

    // Reset state
    showShellSelector = false;
    pendingEcsConnect = null;
    selectedShell = '/bin/sh';
  }

  async function connectToEc2(instance: Ec2Instance) {
    const sessionType: SessionType = {
      type: 'ssm_session',
      instance_id: instance.instance_id,
      profile: $selectedProfile,
      region: $selectedRegion,
    };

    try {
      await createSession({
        session_type: sessionType,
        title: instance.name ? `EC2: ${instance.name}` : undefined,
      });
      showSuccess('Connected to EC2 instance');
    } catch (e) {
      console.error('Failed to create SSM session:', e);
      showError(`Failed to connect: ${e}`);
    }
  }

  function cancelShellSelector() {
    showShellSelector = false;
    pendingEcsConnect = null;
    selectedShell = '/bin/sh';
  }

  // Port forwarding handlers
  function handlePortForward(instance: Ec2Instance) {
    pendingPortForwardInstance = instance;
    showPortForwardingModal = true;
  }

  async function startPortForwarding(config: {
    localPort: number;
    remotePort: number;
    remoteHost?: string;
  }) {
    if (!pendingPortForwardInstance) return;

    const sessionType: SessionType = {
      type: 'ssm_port_forwarding',
      instance_id: pendingPortForwardInstance.instance_id,
      local_port: config.localPort,
      remote_port: config.remotePort,
      remote_host: config.remoteHost,
      profile: $selectedProfile,
      region: $selectedRegion,
    };

    try {
      await createSession({
        session_type: sessionType,
        title: config.remoteHost
          ? `Port ${config.localPort} → ${config.remoteHost}:${config.remotePort}`
          : `Port ${config.localPort} → ${config.remotePort}`,
      });
      showSuccess(`Port forwarding started on localhost:${config.localPort}`);
    } catch (e) {
      console.error('Failed to create port forwarding session:', e);
      showError(`Failed to start port forwarding: ${e}`);
    }

    // Reset state
    showPortForwardingModal = false;
    pendingPortForwardInstance = null;
  }

  function cancelPortForwarding() {
    showPortForwardingModal = false;
    pendingPortForwardInstance = null;
  }

  // Settings handlers
  function handleOpenSettings() {
    showSettingsModal = true;
  }

  function handleCloseSettings() {
    showSettingsModal = false;
  }

  // Log tailing handler
  async function handleLogTail(logGroup: LogGroup) {
    try {
      await createLogTailSession(logGroup.name);
      showSuccess(`Started tailing ${logGroup.name}`);
    } catch (e) {
      console.error('Failed to start log tail:', e);
      showError(`Failed to start log tail: ${e}`);
    }
  }

  // S3 browse handler
  async function handleS3Browse(bucket: S3Bucket) {
    try {
      await openS3Bucket(bucket.name);
    } catch (e) {
      console.error('Failed to open S3 bucket:', e);
      showError(`Failed to open bucket: ${e}`);
    }
  }
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<div class="app-layout">
  <Sidebar
    collapsed={sidebarCollapsed}
    width={sidebarWidth}
    onConnect={handleConnect}
    onPortForward={handlePortForward}
    onLogTail={handleLogTail}
    onS3Browse={handleS3Browse}
    onResize={handleSidebarResize}
    onSettings={handleOpenSettings}
  />

  <div class="main-area">
    <Header onRefresh={handleRefresh} />

    <MainContent onRefresh={handleRefresh} />

    <StatusBar
      activeConnections={$sessionCount}
      profile={$selectedProfile}
      region={$selectedRegion}
    />
  </div>
</div>

<!-- Shell Selector Modal -->
{#if showShellSelector && pendingEcsConnect}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="modal-backdrop"
    onclick={cancelShellSelector}
    onkeydown={(e) => e.key === 'Escape' && cancelShellSelector()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div class="modal" onclick={(e) => e.stopPropagation()} role="document">
      <div class="modal-header">
        <h3>Select Shell</h3>
        <button class="close-btn" onclick={cancelShellSelector} aria-label="Close">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>
      <div class="modal-body">
        <p class="connect-info">
          Connecting to <strong>{pendingEcsConnect.container}</strong>
        </p>
        <div class="shell-options">
          <label class="shell-option">
            <input type="radio" name="shell" value="/bin/sh" bind:group={selectedShell} />
            <span class="shell-label">/bin/sh</span>
            <span class="shell-desc">Most compatible</span>
          </label>
          <label class="shell-option">
            <input type="radio" name="shell" value="/bin/bash" bind:group={selectedShell} />
            <span class="shell-label">/bin/bash</span>
            <span class="shell-desc">More features</span>
          </label>
          <label class="shell-option custom">
            <input type="radio" name="shell" value="custom" bind:group={selectedShell} />
            <span class="shell-label">Custom</span>
            {#if selectedShell === 'custom'}
              <input
                type="text"
                class="custom-shell-input"
                placeholder="/bin/zsh"
                bind:value={selectedShell}
              />
            {/if}
          </label>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn-secondary" onclick={cancelShellSelector}>Cancel</button>
        <button class="btn-primary" onclick={connectToEcs}>Connect</button>
      </div>
    </div>
  </div>
{/if}

<!-- Port Forwarding Modal -->
<PortForwardingModal
  open={showPortForwardingModal}
  instance={pendingPortForwardInstance}
  profile={$selectedProfile}
  region={$selectedRegion}
  onConnect={startPortForwarding}
  onClose={cancelPortForwarding}
/>

<!-- Settings Modal -->
<SettingsModal
  open={showSettingsModal}
  onClose={handleCloseSettings}
/>

<!-- Toast Notifications -->
<ToastContainer />

<style>
  .app-layout {
    display: flex;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }

  .main-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
    overflow: hidden;
  }

  /* Shell Selector Modal */
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
    max-width: 360px;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3);
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--color-border);
  }

  .modal-header h3 {
    font-size: 16px;
    font-weight: 600;
    color: var(--color-text-primary);
    margin: 0;
  }

  .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
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

  .connect-info {
    font-size: 13px;
    color: var(--color-text-secondary);
    margin: 0 0 16px;
  }

  .connect-info strong {
    color: var(--color-text-primary);
  }

  .shell-options {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .shell-option {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px;
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    cursor: pointer;
    transition: border-color 150ms ease;
  }

  .shell-option:hover {
    border-color: var(--color-accent);
  }

  .shell-option input[type="radio"] {
    accent-color: var(--color-accent);
  }

  .shell-label {
    font-size: 14px;
    font-weight: 500;
    color: var(--color-text-primary);
    font-family: monospace;
  }

  .shell-desc {
    font-size: 12px;
    color: var(--color-text-muted);
    margin-left: auto;
  }

  .shell-option.custom {
    flex-wrap: wrap;
  }

  .custom-shell-input {
    width: 100%;
    margin-top: 8px;
    padding: 8px 10px;
    font-size: 13px;
    font-family: monospace;
    color: var(--color-text-primary);
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
  }

  .custom-shell-input:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  .modal-footer {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
    padding: 16px 20px;
    border-top: 1px solid var(--color-border);
  }

  .btn-secondary,
  .btn-primary {
    padding: 8px 16px;
    font-size: 13px;
    font-weight: 500;
    border-radius: 6px;
    transition: all 150ms ease;
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

  .btn-primary:hover {
    background-color: var(--color-accent-hover);
  }
</style>
