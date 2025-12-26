<script lang="ts">
  import { fade } from 'svelte/transition';
  import { TerminalContainer } from '$lib/components/terminal';
  import { LogViewer } from '$lib/components/logs';
  import { S3Browser } from '$lib/components/s3';
  import { hasActiveSessions } from '$lib/stores/terminals';
  import { hasActiveLogSessions } from '$lib/stores/logs';
  import { s3BrowserActive } from '$lib/stores/s3';
  import { resources, resourcesLoading } from '$lib/stores/resources';
  import { info } from '$lib/stores/notifications';

  interface Props {
    onRefresh?: () => void;
  }

  let { onRefresh }: Props = $props();

  // View mode: 'terminals' | 'logs' | 's3' | 'welcome'
  let viewMode = $derived.by(() => {
    if ($hasActiveSessions) return 'terminals';
    if ($hasActiveLogSessions) return 'logs';
    if ($s3BrowserActive) return 's3';
    return 'welcome';
  });

  function handleCardClick(type: 'ecs' | 'ec2' | 'portforward') {
    // If no resources loaded yet, trigger refresh
    if (!$resources && !$resourcesLoading) {
      onRefresh?.();
      info('Refreshing resources... Check the sidebar after loading completes.');
    } else if ($resources) {
      // Resources are loaded, just show a hint
      const hints: Record<string, string> = {
        ecs: 'Expand an ECS cluster in the sidebar, then click the terminal icon on a container.',
        ec2: 'Click the terminal icon on an EC2 instance in the sidebar to connect.',
        portforward: 'Click the purple tunnel icon on an EC2 instance to set up port forwarding.',
      };
      info(hints[type]);
    }
  }
</script>

<main class="main-content">
  {#if viewMode === 'terminals'}
    <TerminalContainer />
  {:else if viewMode === 'logs'}
    <LogViewer />
  {:else if viewMode === 's3'}
    <S3Browser />
  {:else}
    <!-- Welcome/empty state -->
    <div class="welcome-state" in:fade={{ duration: 300 }}>
      <div class="welcome-icon">
        <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <line x1="3" y1="9" x2="21" y2="9"></line>
          <line x1="9" y1="21" x2="9" y2="9"></line>
        </svg>
      </div>
      <h2 class="welcome-title">Welcome to AWS Connector</h2>
      <p class="welcome-text">
        Select a resource from the sidebar to start a terminal session
      </p>
      <div class="quick-actions">
        <div class="action-card" onclick={() => handleCardClick('ecs')} role="button" tabindex="0" onkeydown={(e) => e.key === 'Enter' && handleCardClick('ecs')}>
          <div class="action-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
              <line x1="8" y1="21" x2="16" y2="21"></line>
              <line x1="12" y1="17" x2="12" y2="21"></line>
            </svg>
          </div>
          <h3>ECS Containers</h3>
          <p>Connect to running containers in your ECS clusters</p>
        </div>
        <div class="action-card" onclick={() => handleCardClick('ec2')} role="button" tabindex="0" onkeydown={(e) => e.key === 'Enter' && handleCardClick('ec2')}>
          <div class="action-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="4" y="4" width="16" height="16" rx="2" ry="2"></rect>
              <rect x="9" y="9" width="6" height="6"></rect>
              <line x1="9" y1="1" x2="9" y2="4"></line>
              <line x1="15" y1="1" x2="15" y2="4"></line>
              <line x1="9" y1="20" x2="9" y2="23"></line>
              <line x1="15" y1="20" x2="15" y2="23"></line>
              <line x1="20" y1="9" x2="23" y2="9"></line>
              <line x1="20" y1="14" x2="23" y2="14"></line>
              <line x1="1" y1="9" x2="4" y2="9"></line>
              <line x1="1" y1="14" x2="4" y2="14"></line>
            </svg>
          </div>
          <h3>EC2 Instances</h3>
          <p>SSH into EC2 instances via SSM Session Manager</p>
        </div>
        <div class="action-card" onclick={() => handleCardClick('portforward')} role="button" tabindex="0" onkeydown={(e) => e.key === 'Enter' && handleCardClick('portforward')}>
          <div class="action-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
              <polyline points="15 3 21 3 21 9"></polyline>
              <line x1="10" y1="14" x2="21" y2="3"></line>
            </svg>
          </div>
          <h3>Port Forwarding</h3>
          <p>Create tunnels to RDS, ElastiCache, and other services</p>
        </div>
      </div>
    </div>
  {/if}
</main>

<style>
  .main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    background-color: var(--color-bg-primary);
    overflow: hidden;
  }

  .welcome-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px;
    text-align: center;
  }

  .welcome-icon {
    color: var(--color-text-muted);
    margin-bottom: 24px;
    opacity: 0.5;
  }

  .welcome-title {
    font-size: 24px;
    font-weight: 600;
    color: var(--color-text-primary);
    margin: 0 0 8px 0;
  }

  .welcome-text {
    font-size: 14px;
    color: var(--color-text-secondary);
    margin: 0 0 40px 0;
    max-width: 400px;
  }

  .quick-actions {
    display: flex;
    gap: 20px;
    flex-wrap: wrap;
    justify-content: center;
  }

  .action-card {
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 12px;
    padding: 24px;
    width: 200px;
    text-align: center;
    transition: border-color 200ms ease, transform 200ms ease, box-shadow 200ms ease;
    cursor: pointer;
    animation: cardFadeIn 400ms ease-out backwards;
  }

  .action-card:nth-child(1) { animation-delay: 100ms; }
  .action-card:nth-child(2) { animation-delay: 200ms; }
  .action-card:nth-child(3) { animation-delay: 300ms; }

  @keyframes cardFadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .action-card:hover {
    border-color: var(--color-accent);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  }

  .action-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    margin: 0 auto 16px;
    background-color: var(--color-accent-subtle);
    border-radius: 12px;
    color: var(--color-accent);
  }

  .action-card h3 {
    font-size: 14px;
    font-weight: 600;
    color: var(--color-text-primary);
    margin: 0 0 8px 0;
  }

  .action-card p {
    font-size: 12px;
    color: var(--color-text-secondary);
    margin: 0;
    line-height: 1.4;
  }
</style>
