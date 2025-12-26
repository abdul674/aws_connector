<script lang="ts">
  import type { Ec2Instance } from '$lib/types/aws';

  interface Props {
    open: boolean;
    instance: Ec2Instance | null;
    profile: string;
    region: string;
    onConnect: (config: {
      localPort: number;
      remotePort: number;
      remoteHost?: string;
    }) => void;
    onClose: () => void;
  }

  let { open, instance, profile, region, onConnect, onClose }: Props = $props();

  // Service presets
  const servicePresets = [
    { name: 'PostgreSQL', port: 5432 },
    { name: 'MySQL', port: 3306 },
    { name: 'Redis', port: 6379 },
    { name: 'MongoDB', port: 27017 },
    { name: 'SQL Server', port: 1433 },
    { name: 'Custom', port: 0 },
  ];

  let selectedService = $state('PostgreSQL');
  let localPort = $state(5432);
  let remotePort = $state(5432);
  let remoteHost = $state('');
  let useRemoteHost = $state(false);

  $effect(() => {
    if (open) {
      // Reset form when opening
      selectedService = 'PostgreSQL';
      localPort = 5432;
      remotePort = 5432;
      remoteHost = '';
      useRemoteHost = false;
    }
  });

  function handleServiceChange() {
    const preset = servicePresets.find((s) => s.name === selectedService);
    if (preset && preset.port > 0) {
      remotePort = preset.port;
      localPort = preset.port;
    }
  }

  function handleConnect() {
    onConnect({
      localPort,
      remotePort,
      remoteHost: useRemoteHost && remoteHost ? remoteHost : undefined,
    });
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
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open && instance}
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
        <h2>Port Forwarding</h2>
        <button class="close-btn" onclick={onClose} aria-label="Close">
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <div class="instance-info">
          <span class="instance-label">Instance:</span>
          <span class="instance-value">{instance.name ?? instance.instance_id}</span>
        </div>

        <div class="form-group">
          <label for="service-preset">Service Preset</label>
          <select
            id="service-preset"
            bind:value={selectedService}
            onchange={handleServiceChange}
          >
            {#each servicePresets as preset (preset.name)}
              <option value={preset.name}>{preset.name}{preset.port > 0 ? ` (${preset.port})` : ''}</option>
            {/each}
          </select>
        </div>

        <div class="port-row">
          <div class="form-group">
            <label for="local-port">Local Port</label>
            <input
              type="number"
              id="local-port"
              bind:value={localPort}
              min="1"
              max="65535"
            />
          </div>

          <div class="port-arrow">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="5" y1="12" x2="19" y2="12"></line>
              <polyline points="12 5 19 12 12 19"></polyline>
            </svg>
          </div>

          <div class="form-group">
            <label for="remote-port">Remote Port</label>
            <input
              type="number"
              id="remote-port"
              bind:value={remotePort}
              min="1"
              max="65535"
            />
          </div>
        </div>

        <div class="form-group checkbox-group">
          <label class="checkbox-label">
            <input type="checkbox" bind:checked={useRemoteHost} />
            <span>Forward to remote host (RDS, ElastiCache, etc.)</span>
          </label>
        </div>

        {#if useRemoteHost}
          <div class="form-group">
            <label for="remote-host">Remote Host</label>
            <input
              type="text"
              id="remote-host"
              bind:value={remoteHost}
              placeholder="mydb.xxxx.us-east-1.rds.amazonaws.com"
            />
            <span class="help-text">The endpoint to forward traffic to through this instance</span>
          </div>
        {/if}

        <div class="connection-summary">
          <span class="summary-label">Connection:</span>
          <code class="summary-value">
            localhost:{localPort} → {useRemoteHost && remoteHost ? remoteHost : instance.private_ip ?? instance.instance_id}:{remotePort}
          </code>
        </div>
      </div>

      <div class="modal-footer">
        <button type="button" class="btn-secondary" onclick={onClose}>
          Cancel
        </button>
        <button type="button" class="btn-primary" onclick={handleConnect}>
          Start Forwarding
        </button>
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
    max-width: 440px;
    max-height: 90vh;
    overflow-y: auto;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3);
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--color-border);
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

  .instance-info {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px;
    background-color: var(--color-bg-tertiary);
    border-radius: 6px;
    margin-bottom: 20px;
  }

  .instance-label {
    font-size: 13px;
    color: var(--color-text-secondary);
  }

  .instance-value {
    font-size: 13px;
    font-weight: 500;
    color: var(--color-text-primary);
    font-family: monospace;
  }

  .form-group {
    margin-bottom: 16px;
  }

  .form-group label {
    display: block;
    font-size: 13px;
    font-weight: 500;
    color: var(--color-text-secondary);
    margin-bottom: 6px;
  }

  .form-group input,
  .form-group select {
    width: 100%;
    padding: 10px 12px;
    font-size: 14px;
    color: var(--color-text-primary);
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    transition: border-color 150ms ease, box-shadow 150ms ease;
  }

  .form-group input::placeholder {
    color: var(--color-text-muted);
  }

  .form-group input:focus,
  .form-group select:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: 0 0 0 2px var(--color-accent-subtle);
  }

  .form-group select {
    appearance: none;
    -webkit-appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23a3a3a3' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 12px center;
    padding-right: 36px;
  }

  .port-row {
    display: flex;
    align-items: flex-end;
    gap: 12px;
    margin-bottom: 16px;
  }

  .port-row .form-group {
    flex: 1;
    margin-bottom: 0;
  }

  .port-arrow {
    display: flex;
    align-items: center;
    justify-content: center;
    padding-bottom: 10px;
    color: var(--color-text-muted);
  }

  .checkbox-group {
    margin-bottom: 16px;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
  }

  .checkbox-label input[type="checkbox"] {
    width: 16px;
    height: 16px;
    accent-color: var(--color-accent);
  }

  .checkbox-label span {
    font-size: 13px;
    color: var(--color-text-secondary);
  }

  .help-text {
    display: block;
    font-size: 12px;
    color: var(--color-text-muted);
    margin-top: 4px;
  }

  .connection-summary {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 12px;
    background-color: var(--color-bg-tertiary);
    border-radius: 6px;
    margin-top: 20px;
  }

  .summary-label {
    font-size: 12px;
    color: var(--color-text-muted);
  }

  .summary-value {
    font-size: 13px;
    color: var(--color-accent);
    background: none;
    padding: 0;
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
    padding: 10px 20px;
    font-size: 14px;
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
