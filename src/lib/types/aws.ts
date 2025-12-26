export type ProfileSource = 'Credentials' | 'Config' | 'Both';

export interface AwsProfile {
  name: string;
  region: string | null;
  source: ProfileSource;
  sso_start_url?: string;
  sso_region?: string;
  role_arn?: string;
}

export interface AwsRegion {
  code: string;
  name: string;
}

export interface PrerequisiteStatus {
  awsCli: {
    installed: boolean;
    version?: string;
    error?: string;
  };
  ssmPlugin: {
    installed: boolean;
    version?: string;
    error?: string;
  };
}

export interface AddProfileInput {
  name: string;
  accessKeyId: string;
  secretAccessKey: string;
  region: string;
  sessionToken?: string;
}

export interface AddSsoProfileInput {
  name: string;
  ssoStartUrl: string;
  ssoRegion: string;
  ssoAccountId: string;
  ssoRoleName: string;
  region: string;
}

// ECS Types
export interface EcsCluster {
  arn: string;
  name: string;
  status: string;
  running_tasks_count: number;
  services_count: number;
}

export interface EcsService {
  arn: string;
  name: string;
  cluster_arn: string;
  status: string;
  desired_count: number;
  running_count: number;
  launch_type: string | null;
}

export interface EcsContainer {
  name: string;
  runtime_id: string | null;
  last_status: string;
  health_status: string | null;
}

export interface EcsTask {
  arn: string;
  task_definition_arn: string;
  cluster_arn: string;
  last_status: string;
  desired_status: string;
  launch_type: string | null;
  containers: EcsContainer[];
  enable_execute_command: boolean;
}

export interface EcsResources {
  clusters: EcsCluster[];
  services: Record<string, EcsService[]>;
  tasks: Record<string, EcsTask[]>;
}

// EC2 Types
export interface Ec2Instance {
  instance_id: string;
  name: string | null;
  instance_type: string;
  state: string;
  private_ip: string | null;
  public_ip: string | null;
  platform: string | null;
  ssm_enabled: boolean;
  ssm_ping_status: string | null;
}

// Combined Resources
export interface DiscoveredResources {
  ecs: EcsResources;
  ec2_instances: Ec2Instance[];
}

// Tree Node Types for UI
export type ResourceType = 'cluster' | 'service' | 'task' | 'container' | 'instance';

export interface ResourceNode {
  id: string;
  type: ResourceType;
  name: string;
  status: string;
  data: EcsCluster | EcsService | EcsTask | EcsContainer | Ec2Instance;
  children?: ResourceNode[];
  expanded?: boolean;
}
