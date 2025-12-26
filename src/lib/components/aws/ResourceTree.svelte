<script lang="ts">
  import { slide } from 'svelte/transition';
  import {
    resources,
    resourcesLoading,
    resourcesError,
    expandedNodes,
    toggleNode,
    getServicesForCluster,
    getTasksForService,
  } from '$lib/stores/resources';
  import {
    logGroups,
    logGroupsLoading,
  } from '$lib/stores/logs';
  import {
    s3Buckets,
    s3BucketsLoading,
  } from '$lib/stores/s3';
  import type { EcsCluster, EcsService, EcsTask, EcsContainer, Ec2Instance } from '$lib/types/aws';
  import type { LogGroup } from '$lib/types/logs';
  import type { S3Bucket } from '$lib/types/s3';

  interface Props {
    onConnect?: (type: 'ecs' | 'ec2', data: unknown) => void;
    onPortForward?: (instance: Ec2Instance) => void;
    onLogTail?: (logGroup: LogGroup) => void;
    onS3Browse?: (bucket: S3Bucket) => void;
  }

  let { onConnect, onPortForward, onLogTail, onS3Browse }: Props = $props();

  function handleConnect(type: 'ecs' | 'ec2', data: unknown) {
    onConnect?.(type, data);
  }

  function handlePortForward(instance: Ec2Instance) {
    onPortForward?.(instance);
  }

  function handleLogTail(logGroup: LogGroup) {
    onLogTail?.(logGroup);
  }

  function handleS3Browse(bucket: S3Bucket) {
    onS3Browse?.(bucket);
  }

  function getTaskId(taskArn: string): string {
    return taskArn.split('/').pop() ?? taskArn;
  }

  function getStatusColor(status: string): string {
    const s = status.toUpperCase();
    if (s === 'RUNNING' || s === 'ACTIVE' || s === 'ONLINE') return 'var(--color-success)';
    if (s === 'PENDING' || s === 'PROVISIONING') return 'var(--color-warning)';
    if (s === 'STOPPED' || s === 'INACTIVE' || s === 'OFFLINE') return 'var(--color-error)';
    return 'var(--color-text-muted)';
  }
</script>

<div class="resource-tree">
  {#if $resourcesLoading}
    <div class="loading-state">
      <div class="spinner"></div>
      <span>Discovering resources...</span>
    </div>
  {:else if $resourcesError}
    <div class="error-state">
      <span class="error-icon">!</span>
      <span class="error-text">{$resourcesError}</span>
    </div>
  {:else if !$resources}
    <div class="empty-state">
      <p>Click refresh to discover resources</p>
    </div>
  {:else}
    <!-- ECS Section -->
    <div class="section">
      <div class="section-header">
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
          <line x1="8" y1="21" x2="16" y2="21"></line>
          <line x1="12" y1="17" x2="12" y2="21"></line>
        </svg>
        <span>ECS Clusters</span>
        <span class="count">{$resources.ecs.clusters.length}</span>
      </div>

      {#if $resources.ecs.clusters.length === 0}
        <div class="no-items">No clusters found</div>
      {:else}
        {#each $resources.ecs.clusters as cluster (cluster.arn)}
          {@const isExpanded = $expandedNodes.has(cluster.arn)}
          {@const services = getServicesForCluster(cluster.arn)}

          <div class="tree-node cluster">
            <button
              class="node-header"
              onclick={() => toggleNode(cluster.arn)}
            >
              <span class="chevron" class:expanded={isExpanded}>
                <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="9 18 15 12 9 6"></polyline>
                </svg>
              </span>
              <span class="node-icon cluster-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="2" y="2" width="20" height="8" rx="2" ry="2"></rect>
                  <rect x="2" y="14" width="20" height="8" rx="2" ry="2"></rect>
                  <line x1="6" y1="6" x2="6.01" y2="6"></line>
                  <line x1="6" y1="18" x2="6.01" y2="18"></line>
                </svg>
              </span>
              <span class="node-name" title={cluster.name}>{cluster.name}</span>
              <span class="node-badge">{cluster.running_tasks_count} tasks</span>
              <span class="status-indicator" style="background-color: {getStatusColor(cluster.status)}"></span>
            </button>

            {#if isExpanded}
              <div class="node-children" transition:slide={{ duration: 150 }}>
                {#if services.length === 0}
                  <div class="no-items nested">No services</div>
                {:else}
                  {#each services as service (service.arn)}
                    {@const serviceExpanded = $expandedNodes.has(service.arn)}
                    {@const tasks = getTasksForService(cluster.arn, service.name)}

                    <div class="tree-node service">
                      <button
                        class="node-header"
                        onclick={() => toggleNode(service.arn)}
                      >
                        <span class="chevron" class:expanded={serviceExpanded}>
                          <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <polyline points="9 18 15 12 9 6"></polyline>
                          </svg>
                        </span>
                        <span class="node-icon service-icon">
                          <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
                            <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
                          </svg>
                        </span>
                        <span class="node-name" title={service.name}>{service.name}</span>
                        <span class="node-badge">{service.running_count}/{service.desired_count}</span>
                        <span class="status-indicator" style="background-color: {getStatusColor(service.status)}"></span>
                      </button>

                      {#if serviceExpanded}
                        <div class="node-children" transition:slide={{ duration: 150 }}>
                          {#if tasks.length === 0}
                            <div class="no-items nested">No running tasks</div>
                          {:else}
                            {#each tasks as task (task.arn)}
                              {@const taskExpanded = $expandedNodes.has(task.arn)}

                              <div class="tree-node task">
                                <button
                                  class="node-header"
                                  onclick={() => toggleNode(task.arn)}
                                >
                                  <span class="chevron" class:expanded={taskExpanded}>
                                    <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                      <polyline points="9 18 15 12 9 6"></polyline>
                                    </svg>
                                  </span>
                                  <span class="node-icon task-icon">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                      <polyline points="4 17 10 11 4 5"></polyline>
                                      <line x1="12" y1="19" x2="20" y2="19"></line>
                                    </svg>
                                  </span>
                                  <span class="node-name" title={getTaskId(task.arn)}>{getTaskId(task.arn)}</span>
                                  {#if task.enable_execute_command}
                                    <span class="exec-badge">exec</span>
                                  {/if}
                                  <span class="status-indicator" style="background-color: {getStatusColor(task.last_status)}"></span>
                                </button>

                                {#if taskExpanded}
                                  <div class="node-children" transition:slide={{ duration: 150 }}>
                                    {#each task.containers as container (container.name)}
                                      <div class="tree-node container">
                                        <div class="node-header container-node">
                                          <span class="node-icon container-icon">
                                            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                              <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
                                            </svg>
                                          </span>
                                          <span class="node-name" title={container.name}>{container.name}</span>
                                          <span class="status-indicator" style="background-color: {getStatusColor(container.last_status)}"></span>
                                          {#if task.enable_execute_command && container.runtime_id}
                                            <button
                                              class="connect-btn"
                                              onclick={() => handleConnect('ecs', { cluster, service, task, container })}
                                              title="Connect to container"
                                            >
                                              <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <polyline points="4 17 10 11 4 5"></polyline>
                                                <line x1="12" y1="19" x2="20" y2="19"></line>
                                              </svg>
                                            </button>
                                          {/if}
                                        </div>
                                      </div>
                                    {/each}
                                  </div>
                                {/if}
                              </div>
                            {/each}
                          {/if}
                        </div>
                      {/if}
                    </div>
                  {/each}
                {/if}
              </div>
            {/if}
          </div>
        {/each}
      {/if}
    </div>

    <!-- EC2 Section -->
    <div class="section">
      <div class="section-header">
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
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
        <span>EC2 Instances</span>
        <span class="count">{$resources.ec2_instances.length}</span>
      </div>

      {#if $resources.ec2_instances.length === 0}
        <div class="no-items">No SSM-enabled instances</div>
      {:else}
        {#each $resources.ec2_instances as instance (instance.instance_id)}
          <div class="tree-node instance">
            <div class="node-header instance-node">
              <span class="node-icon instance-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
                  <line x1="8" y1="21" x2="16" y2="21"></line>
                  <line x1="12" y1="17" x2="12" y2="21"></line>
                </svg>
              </span>
              <span class="node-name" title={instance.name ?? instance.instance_id}>{instance.name ?? instance.instance_id}</span>
              {#if instance.name}
                <span class="instance-id">{instance.instance_id}</span>
              {/if}
              <span class="status-indicator" style="background-color: {getStatusColor(instance.ssm_ping_status ?? 'offline')}"></span>
              {#if instance.ssm_ping_status === 'Online'}
                <button
                  class="connect-btn"
                  onclick={() => handleConnect('ec2', instance)}
                  title="SSH Terminal"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="4 17 10 11 4 5"></polyline>
                    <line x1="12" y1="19" x2="20" y2="19"></line>
                  </svg>
                </button>
                <button
                  class="port-forward-btn"
                  onclick={() => handlePortForward(instance)}
                  title="Port Forwarding"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"></path>
                  </svg>
                </button>
              {/if}
            </div>
          </div>
        {/each}
      {/if}
    </div>

    <!-- CloudWatch Logs Section -->
    <div class="section">
      <div class="section-header">
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
          <polyline points="14 2 14 8 20 8"></polyline>
          <line x1="16" y1="13" x2="8" y2="13"></line>
          <line x1="16" y1="17" x2="8" y2="17"></line>
          <polyline points="10 9 9 9 8 9"></polyline>
        </svg>
        <span>CloudWatch Logs</span>
        {#if $logGroupsLoading}
          <div class="spinner small"></div>
        {:else}
          <span class="count">{$logGroups.length}</span>
        {/if}
      </div>

      {#if $logGroups.length === 0 && !$logGroupsLoading}
        <div class="no-items">No log groups found</div>
      {:else}
        {#each $logGroups as logGroup (logGroup.arn)}
          <div class="tree-node log-group">
            <div class="node-header log-group-node">
              <span class="node-icon log-group-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                  <polyline points="14 2 14 8 20 8"></polyline>
                </svg>
              </span>
              <span class="node-name" title={logGroup.name}>{logGroup.name}</span>
              {#if logGroup.retention_days}
                <span class="retention-badge">{logGroup.retention_days}d</span>
              {/if}
              <button
                class="tail-btn"
                onclick={() => handleLogTail(logGroup)}
                title="Tail logs"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="4 17 10 11 4 5"></polyline>
                  <line x1="12" y1="19" x2="20" y2="19"></line>
                </svg>
              </button>
            </div>
          </div>
        {/each}
      {/if}
    </div>

    <!-- S3 Buckets Section -->
    <div class="section">
      <div class="section-header">
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
        </svg>
        <span>S3 Buckets</span>
        {#if $s3BucketsLoading}
          <div class="spinner small"></div>
        {:else}
          <span class="count">{$s3Buckets.length}</span>
        {/if}
      </div>

      {#if $s3Buckets.length === 0 && !$s3BucketsLoading}
        <div class="no-items">No S3 buckets found</div>
      {:else}
        {#each $s3Buckets as bucket (bucket.name)}
          <div class="tree-node s3-bucket">
            <div class="node-header s3-bucket-node">
              <span class="node-icon s3-bucket-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
                </svg>
              </span>
              <span class="node-name" title={bucket.name}>{bucket.name}</span>
              <button
                class="browse-btn"
                onclick={() => handleS3Browse(bucket)}
                title="Browse bucket"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
                  <circle cx="12" cy="12" r="3"></circle>
                </svg>
              </button>
            </div>
          </div>
        {/each}
      {/if}
    </div>
  {/if}
</div>

<style>
  .resource-tree {
    font-size: 13px;
  }

  .loading-state,
  .error-state,
  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 24px 16px;
    color: var(--color-text-secondary);
    text-align: center;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.5; transform: scale(0.9); }
  }

  .error-state {
    color: var(--color-error);
    flex-direction: column;
  }

  .error-icon {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--color-error);
    color: white;
    border-radius: 50%;
    font-weight: bold;
    font-size: 14px;
  }

  .error-text {
    font-size: 12px;
    max-width: 200px;
    word-break: break-word;
  }

  .section {
    margin-bottom: 16px;
  }

  .section-header {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--color-text-muted);
    letter-spacing: 0.5px;
  }

  .section-header .count {
    margin-left: auto;
    background-color: var(--color-bg-tertiary);
    padding: 2px 6px;
    border-radius: 10px;
    font-size: 10px;
  }

  .no-items {
    padding: 12px 16px;
    color: var(--color-text-muted);
    font-size: 12px;
  }

  .no-items.nested {
    padding-left: 40px;
  }

  .tree-node {
    margin: 0;
  }

  .node-header {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    padding: 6px 12px;
    text-align: left;
    color: var(--color-text-primary);
    border-radius: 4px;
    transition: background-color 100ms ease;
  }

  .node-header:hover {
    background-color: var(--color-bg-hover);
  }

  .container-node,
  .instance-node {
    cursor: default;
  }

  .chevron {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    color: var(--color-text-muted);
    transition: transform 150ms ease;
  }

  .chevron.expanded {
    transform: rotate(90deg);
  }

  .node-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    color: var(--color-text-secondary);
  }

  .cluster-icon { color: var(--color-accent); }
  .service-icon { color: #a855f7; }
  .task-icon { color: #22c55e; }
  .container-icon { color: #f59e0b; }
  .instance-icon { color: #06b6d4; }

  .node-name {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .node-badge {
    font-size: 11px;
    color: var(--color-text-muted);
    background-color: var(--color-bg-tertiary);
    padding: 1px 6px;
    border-radius: 4px;
  }

  .exec-badge {
    font-size: 9px;
    color: var(--color-success);
    background-color: rgba(34, 197, 94, 0.1);
    padding: 1px 4px;
    border-radius: 3px;
    font-weight: 600;
    text-transform: uppercase;
  }

  .instance-id {
    font-size: 11px;
    color: var(--color-text-muted);
    font-family: monospace;
  }

  .status-indicator {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .connect-btn,
  .port-forward-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    border-radius: 4px;
    transition: background-color 100ms ease;
  }

  .connect-btn {
    color: var(--color-accent);
    background-color: var(--color-accent-subtle);
  }

  .connect-btn:hover {
    background-color: var(--color-accent);
    color: white;
  }

  .port-forward-btn {
    color: #a855f7;
    background-color: rgba(168, 85, 247, 0.1);
  }

  .port-forward-btn:hover {
    background-color: #a855f7;
    color: white;
  }

  .node-children {
    padding-left: 20px;
  }

  .service .node-children {
    padding-left: 16px;
  }

  .task .node-children {
    padding-left: 12px;
  }

  .log-group-node {
    cursor: default;
  }

  .log-group-icon { color: #f97316; }

  .spinner.small {
    width: 12px;
    height: 12px;
    border-width: 1.5px;
    margin-left: auto;
  }

  .retention-badge {
    font-size: 10px;
    color: var(--color-text-muted);
    background-color: var(--color-bg-tertiary);
    padding: 1px 4px;
    border-radius: 3px;
    font-family: monospace;
  }

  .tail-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    border-radius: 4px;
    color: #f97316;
    background-color: rgba(249, 115, 22, 0.1);
    transition: background-color 100ms ease;
  }

  .tail-btn:hover {
    background-color: #f97316;
    color: white;
  }

  .s3-bucket-node {
    cursor: default;
  }

  .s3-bucket-icon { color: #22c55e; }

  .browse-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    border-radius: 4px;
    color: #22c55e;
    background-color: rgba(34, 197, 94, 0.1);
    transition: background-color 100ms ease;
  }

  .browse-btn:hover {
    background-color: #22c55e;
    color: white;
  }
</style>
