<script lang="ts">
  import { notificationList, dismiss, type Notification } from '$lib/stores/notifications';
  import { fly, fade } from 'svelte/transition';

  function getIcon(type: Notification['type']): string {
    switch (type) {
      case 'success':
        return 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z';
      case 'error':
        return 'M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z';
      case 'warning':
        return 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z';
      case 'info':
      default:
        return 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z';
    }
  }
</script>

<div class="toast-container">
  {#each $notificationList as notification (notification.id)}
    <div
      class="toast toast-{notification.type}"
      in:fly={{ x: 100, duration: 200 }}
      out:fade={{ duration: 150 }}
    >
      <div class="toast-icon">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d={getIcon(notification.type)}></path>
        </svg>
      </div>
      <div class="toast-content">
        <p class="toast-message">{notification.message}</p>
      </div>
      <button class="toast-dismiss" onclick={() => dismiss(notification.id)} aria-label="Dismiss">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    bottom: 20px;
    right: 20px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    z-index: 2000;
    max-width: 400px;
  }

  .toast {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 12px 16px;
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    animation: slideIn 200ms ease-out;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateX(20px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }

  .toast-icon {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
  }

  .toast-success .toast-icon {
    color: var(--color-success);
  }

  .toast-error .toast-icon {
    color: var(--color-error);
  }

  .toast-warning .toast-icon {
    color: var(--color-warning);
  }

  .toast-info .toast-icon {
    color: var(--color-accent);
  }

  .toast-content {
    flex: 1;
    min-width: 0;
  }

  .toast-message {
    font-size: 13px;
    color: var(--color-text-primary);
    margin: 0;
    line-height: 1.4;
  }

  .toast-dismiss {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    color: var(--color-text-muted);
    border-radius: 4px;
    transition: color 150ms ease, background-color 150ms ease;
  }

  .toast-dismiss:hover {
    color: var(--color-text-primary);
    background-color: var(--color-bg-hover);
  }

  /* Border accents by type */
  .toast-success {
    border-left: 3px solid var(--color-success);
  }

  .toast-error {
    border-left: 3px solid var(--color-error);
  }

  .toast-warning {
    border-left: 3px solid var(--color-warning);
  }

  .toast-info {
    border-left: 3px solid var(--color-accent);
  }
</style>
