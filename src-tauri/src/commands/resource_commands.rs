use crate::aws::{
    discover_ecs_resources, list_clusters, list_services, list_ssm_instances, list_tasks,
    Ec2Instance, EcsCluster, EcsResources, EcsService, EcsTask,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredResources {
    pub ecs: EcsResources,
    pub ec2_instances: Vec<Ec2Instance>,
}

/// Discover all AWS resources (ECS and EC2)
#[tauri::command]
pub async fn discover_resources(
    profile: String,
    region: String,
) -> Result<DiscoveredResources, String> {
    // Run ECS and EC2 discovery in parallel
    let ecs_future = discover_ecs_resources(&profile, &region);
    let ec2_future = list_ssm_instances(&profile, &region);

    let (ecs_result, ec2_result) = tokio::join!(ecs_future, ec2_future);

    Ok(DiscoveredResources {
        ecs: ecs_result?,
        ec2_instances: ec2_result?,
    })
}

/// List ECS clusters
#[tauri::command]
pub async fn list_ecs_clusters(
    profile: String,
    region: String,
) -> Result<Vec<EcsCluster>, String> {
    list_clusters(&profile, &region).await
}

/// List ECS services in a cluster
#[tauri::command]
pub async fn list_ecs_services(
    profile: String,
    region: String,
    cluster_arn: String,
) -> Result<Vec<EcsService>, String> {
    list_services(&profile, &region, &cluster_arn).await
}

/// List ECS tasks in a cluster/service
#[tauri::command]
pub async fn list_ecs_tasks(
    profile: String,
    region: String,
    cluster_arn: String,
    service_name: Option<String>,
) -> Result<Vec<EcsTask>, String> {
    list_tasks(
        &profile,
        &region,
        &cluster_arn,
        service_name.as_deref(),
    )
    .await
}

/// List SSM-enabled EC2 instances
#[tauri::command]
pub async fn list_ec2_instances(
    profile: String,
    region: String,
) -> Result<Vec<Ec2Instance>, String> {
    list_ssm_instances(&profile, &region).await
}
