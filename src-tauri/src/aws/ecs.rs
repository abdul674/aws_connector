use aws_config::BehaviorVersion;
use aws_sdk_ecs::Client as EcsClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsCluster {
    pub arn: String,
    pub name: String,
    pub status: String,
    pub running_tasks_count: i32,
    pub services_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsService {
    pub arn: String,
    pub name: String,
    pub cluster_arn: String,
    pub status: String,
    pub desired_count: i32,
    pub running_count: i32,
    pub launch_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsTask {
    pub arn: String,
    pub task_definition_arn: String,
    pub cluster_arn: String,
    pub last_status: String,
    pub desired_status: String,
    pub launch_type: Option<String>,
    pub containers: Vec<EcsContainer>,
    pub enable_execute_command: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsContainer {
    pub name: String,
    pub runtime_id: Option<String>,
    pub last_status: String,
    pub health_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsResources {
    pub clusters: Vec<EcsCluster>,
    pub services: HashMap<String, Vec<EcsService>>,
    pub tasks: HashMap<String, Vec<EcsTask>>,
}

/// Create an ECS client with the specified profile and region
async fn create_ecs_client(profile: &str, region: &str) -> Result<EcsClient, String> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .profile_name(profile)
        .region(aws_config::Region::new(region.to_string()))
        .load()
        .await;

    Ok(EcsClient::new(&config))
}

/// List all ECS clusters
pub async fn list_clusters(profile: &str, region: &str) -> Result<Vec<EcsCluster>, String> {
    let client = create_ecs_client(profile, region).await?;

    // First, list cluster ARNs
    let list_result = client
        .list_clusters()
        .send()
        .await
        .map_err(|e| format!("Failed to list clusters: {}", e))?;

    let cluster_arns = list_result.cluster_arns();

    if cluster_arns.is_empty() {
        return Ok(vec![]);
    }

    // Then, describe the clusters to get details
    let describe_result = client
        .describe_clusters()
        .set_clusters(Some(cluster_arns.to_vec()))
        .send()
        .await
        .map_err(|e| format!("Failed to describe clusters: {}", e))?;

    let clusters = describe_result
        .clusters()
        .iter()
        .map(|c| EcsCluster {
            arn: c.cluster_arn().unwrap_or_default().to_string(),
            name: c.cluster_name().unwrap_or_default().to_string(),
            status: c.status().unwrap_or_default().to_string(),
            running_tasks_count: c.running_tasks_count(),
            services_count: c.active_services_count(),
        })
        .collect();

    Ok(clusters)
}

/// List services in a cluster
pub async fn list_services(
    profile: &str,
    region: &str,
    cluster_arn: &str,
) -> Result<Vec<EcsService>, String> {
    let client = create_ecs_client(profile, region).await?;

    // List service ARNs
    let mut service_arns = Vec::new();
    let mut next_token: Option<String> = None;

    loop {
        let mut request = client.list_services().cluster(cluster_arn);

        if let Some(token) = next_token {
            request = request.next_token(token);
        }

        let result = request
            .send()
            .await
            .map_err(|e| format!("Failed to list services: {}", e))?;

        service_arns.extend(result.service_arns().to_vec());

        next_token = result.next_token().map(|s| s.to_string());
        if next_token.is_none() {
            break;
        }
    }

    if service_arns.is_empty() {
        return Ok(vec![]);
    }

    // Describe services (max 10 at a time)
    let mut services = Vec::new();

    for chunk in service_arns.chunks(10) {
        let describe_result = client
            .describe_services()
            .cluster(cluster_arn)
            .set_services(Some(chunk.to_vec()))
            .send()
            .await
            .map_err(|e| format!("Failed to describe services: {}", e))?;

        for s in describe_result.services() {
            services.push(EcsService {
                arn: s.service_arn().unwrap_or_default().to_string(),
                name: s.service_name().unwrap_or_default().to_string(),
                cluster_arn: s.cluster_arn().unwrap_or_default().to_string(),
                status: s.status().unwrap_or_default().to_string(),
                desired_count: s.desired_count(),
                running_count: s.running_count(),
                launch_type: s.launch_type().map(|lt| lt.as_str().to_string()),
            });
        }
    }

    Ok(services)
}

/// List tasks in a cluster (optionally filtered by service)
pub async fn list_tasks(
    profile: &str,
    region: &str,
    cluster_arn: &str,
    service_name: Option<&str>,
) -> Result<Vec<EcsTask>, String> {
    let client = create_ecs_client(profile, region).await?;

    // List task ARNs
    let mut request = client
        .list_tasks()
        .cluster(cluster_arn)
        .desired_status(aws_sdk_ecs::types::DesiredStatus::Running);

    if let Some(service) = service_name {
        request = request.service_name(service);
    }

    let list_result = request
        .send()
        .await
        .map_err(|e| format!("Failed to list tasks: {}", e))?;

    let task_arns = list_result.task_arns();

    if task_arns.is_empty() {
        return Ok(vec![]);
    }

    // Describe tasks
    let describe_result = client
        .describe_tasks()
        .cluster(cluster_arn)
        .set_tasks(Some(task_arns.to_vec()))
        .send()
        .await
        .map_err(|e| format!("Failed to describe tasks: {}", e))?;

    let tasks = describe_result
        .tasks()
        .iter()
        .map(|t| {
            let containers = t
                .containers()
                .iter()
                .map(|c| EcsContainer {
                    name: c.name().unwrap_or_default().to_string(),
                    runtime_id: c.runtime_id().map(|s| s.to_string()),
                    last_status: c.last_status().unwrap_or_default().to_string(),
                    health_status: c.health_status().map(|h| h.as_str().to_string()),
                })
                .collect();

            EcsTask {
                arn: t.task_arn().unwrap_or_default().to_string(),
                task_definition_arn: t.task_definition_arn().unwrap_or_default().to_string(),
                cluster_arn: t.cluster_arn().unwrap_or_default().to_string(),
                last_status: t.last_status().unwrap_or_default().to_string(),
                desired_status: t.desired_status().unwrap_or_default().to_string(),
                launch_type: t.launch_type().map(|lt| lt.as_str().to_string()),
                containers,
                enable_execute_command: t.enable_execute_command(),
            }
        })
        .collect();

    Ok(tasks)
}

/// Discover all ECS resources (clusters, services, tasks)
pub async fn discover_ecs_resources(
    profile: &str,
    region: &str,
) -> Result<EcsResources, String> {
    let clusters = list_clusters(profile, region).await?;

    let mut services: HashMap<String, Vec<EcsService>> = HashMap::new();
    let mut tasks: HashMap<String, Vec<EcsTask>> = HashMap::new();

    for cluster in &clusters {
        // Get services for this cluster
        let cluster_services = list_services(profile, region, &cluster.arn).await?;
        services.insert(cluster.arn.clone(), cluster_services.clone());

        // Get tasks for each service
        for service in &cluster_services {
            let service_tasks =
                list_tasks(profile, region, &cluster.arn, Some(&service.name)).await?;
            let key = format!("{}:{}", cluster.arn, service.name);
            tasks.insert(key, service_tasks);
        }
    }

    Ok(EcsResources {
        clusters,
        services,
        tasks,
    })
}
