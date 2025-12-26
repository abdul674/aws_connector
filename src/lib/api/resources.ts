import { invoke } from '@tauri-apps/api/core';
import type {
  DiscoveredResources,
  EcsCluster,
  EcsService,
  EcsTask,
  Ec2Instance,
} from '$lib/types/aws';

/**
 * Discover all AWS resources (ECS and EC2)
 */
export async function discoverResources(
  profile: string,
  region: string
): Promise<DiscoveredResources> {
  return invoke<DiscoveredResources>('discover_resources', { profile, region });
}

/**
 * List ECS clusters
 */
export async function listEcsClusters(
  profile: string,
  region: string
): Promise<EcsCluster[]> {
  return invoke<EcsCluster[]>('list_ecs_clusters', { profile, region });
}

/**
 * List ECS services in a cluster
 */
export async function listEcsServices(
  profile: string,
  region: string,
  clusterArn: string
): Promise<EcsService[]> {
  return invoke<EcsService[]>('list_ecs_services', {
    profile,
    region,
    clusterArn,
  });
}

/**
 * List ECS tasks in a cluster/service
 */
export async function listEcsTasks(
  profile: string,
  region: string,
  clusterArn: string,
  serviceName?: string
): Promise<EcsTask[]> {
  return invoke<EcsTask[]>('list_ecs_tasks', {
    profile,
    region,
    clusterArn,
    serviceName,
  });
}

/**
 * List SSM-enabled EC2 instances
 */
export async function listEc2Instances(
  profile: string,
  region: string
): Promise<Ec2Instance[]> {
  return invoke<Ec2Instance[]>('list_ec2_instances', { profile, region });
}
