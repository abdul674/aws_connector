<script lang="ts">
  interface Props {
    activeConnections?: number;
    profile?: string;
    region?: string;
  }

  let {
    activeConnections = 0,
    profile = 'default',
    region = 'us-east-1'
  }: Props = $props();
</script>

<footer class="status-bar">
  <div class="status-left">
    <div class="status-item">
      <span class="status-label">Profile:</span>
      <span class="status-value">{profile}</span>
    </div>
    <div class="status-divider"></div>
    <div class="status-item">
      <span class="status-label">Region:</span>
      <span class="status-value">{region}</span>
    </div>
  </div>

  <div class="status-right">
    <div class="status-item">
      <span class="status-dot" class:online={activeConnections > 0}></span>
      <span class="status-value">
        {activeConnections} active {activeConnections === 1 ? 'session' : 'sessions'}
      </span>
    </div>
  </div>
</footer>

<style>
  .status-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px 12px;
    background-color: var(--color-bg-tertiary);
    border-top: 1px solid var(--color-border);
    font-size: 11px;
    min-height: 24px;
  }

  .status-left,
  .status-right {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .status-item {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .status-label {
    color: var(--color-text-muted);
  }

  .status-value {
    color: var(--color-text-secondary);
  }

  .status-divider {
    width: 1px;
    height: 12px;
    background-color: var(--color-border);
  }

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background-color: var(--color-text-muted);
  }

  .status-dot.online {
    background-color: var(--color-success);
  }
</style>
