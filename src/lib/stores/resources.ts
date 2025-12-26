import { writable, derived, get } from 'svelte/store';
import type { DiscoveredResources, EcsCluster, EcsService, EcsTask, Ec2Instance } from '$lib/types/aws';
import { discoverResources } from '$lib/api/resources';
import { selectedProfile, selectedRegion } from './profiles';

// Store for discovered resources
export const resources = writable<DiscoveredResources | null>(null);
export const resourcesLoading = writable<boolean>(false);
export const resourcesError = writable<string | null>(null);

// Expanded state for tree nodes
export const expandedNodes = writable<Set<string>>(new Set());

// Derived stores for easier access
export const ecsClusters = derived(resources, ($resources) => {
  return $resources?.ecs.clusters ?? [];
});

export const ec2Instances = derived(resources, ($resources) => {
  return $resources?.ec2_instances ?? [];
});

/**
 * Get services for a cluster
 */
export function getServicesForCluster(clusterArn: string): EcsService[] {
  const $resources = get(resources);
  return $resources?.ecs.services[clusterArn] ?? [];
}

/**
 * Get tasks for a service
 */
export function getTasksForService(clusterArn: string, serviceName: string): EcsTask[] {
  const $resources = get(resources);
  const key = `${clusterArn}:${serviceName}`;
  return $resources?.ecs.tasks[key] ?? [];
}

/**
 * Load AWS resources
 */
export async function loadResources(): Promise<void> {
  const profile = get(selectedProfile);
  const region = get(selectedRegion);

  if (!profile || !region) {
    return;
  }

  resourcesLoading.set(true);
  resourcesError.set(null);

  try {
    const discovered = await discoverResources(profile, region);
    resources.set(discovered);

    // Auto-expand clusters with running tasks
    const toExpand = new Set<string>();
    for (const cluster of discovered.ecs.clusters) {
      if (cluster.running_tasks_count > 0) {
        toExpand.add(cluster.arn);
      }
    }
    expandedNodes.set(toExpand);
  } catch (error) {
    resourcesError.set(String(error));
    console.error('Failed to load AWS resources:', error);
  } finally {
    resourcesLoading.set(false);
  }
}

/**
 * Toggle node expansion
 */
export function toggleNode(nodeId: string): void {
  expandedNodes.update((nodes) => {
    const newNodes = new Set(nodes);
    if (newNodes.has(nodeId)) {
      newNodes.delete(nodeId);
    } else {
      newNodes.add(nodeId);
    }
    return newNodes;
  });
}

/**
 * Check if a node is expanded
 */
export function isNodeExpanded(nodeId: string): boolean {
  return get(expandedNodes).has(nodeId);
}

/**
 * Clear resources (e.g., when changing profile/region)
 */
export function clearResources(): void {
  resources.set(null);
  expandedNodes.set(new Set());
  resourcesError.set(null);
}
