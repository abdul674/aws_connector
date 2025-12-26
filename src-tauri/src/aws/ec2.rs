use aws_config::BehaviorVersion;
use aws_sdk_ec2::Client as Ec2Client;
use aws_sdk_ssm::Client as SsmClient;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ec2Instance {
    pub instance_id: String,
    pub name: Option<String>,
    pub instance_type: String,
    pub state: String,
    pub private_ip: Option<String>,
    pub public_ip: Option<String>,
    pub platform: Option<String>,
    pub ssm_enabled: bool,
    pub ssm_ping_status: Option<String>,
}

/// Create an EC2 client with the specified profile and region
async fn create_ec2_client(profile: &str, region: &str) -> Result<Ec2Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .profile_name(profile)
        .region(aws_config::Region::new(region.to_string()))
        .load()
        .await;

    Ok(Ec2Client::new(&config))
}

/// Create an SSM client with the specified profile and region
async fn create_ssm_client(profile: &str, region: &str) -> Result<SsmClient, String> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .profile_name(profile)
        .region(aws_config::Region::new(region.to_string()))
        .load()
        .await;

    Ok(SsmClient::new(&config))
}

/// Get all SSM-managed instance IDs
async fn get_ssm_managed_instances(
    profile: &str,
    region: &str,
) -> Result<HashSet<String>, String> {
    let client = create_ssm_client(profile, region).await?;

    let mut instance_ids = HashSet::new();
    let mut next_token: Option<String> = None;

    loop {
        let mut request = client.describe_instance_information();

        if let Some(token) = next_token {
            request = request.next_token(token);
        }

        let result = request
            .send()
            .await
            .map_err(|e| format!("Failed to get SSM instances: {}", e))?;

        for info in result.instance_information_list() {
            if let Some(id) = info.instance_id() {
                instance_ids.insert(id.to_string());
            }
        }

        next_token = result.next_token().map(|s| s.to_string());
        if next_token.is_none() {
            break;
        }
    }

    Ok(instance_ids)
}

/// Get SSM ping status for instances
async fn get_ssm_ping_status(
    profile: &str,
    region: &str,
) -> Result<std::collections::HashMap<String, String>, String> {
    let client = create_ssm_client(profile, region).await?;

    let mut status_map = std::collections::HashMap::new();
    let mut next_token: Option<String> = None;

    loop {
        let mut request = client.describe_instance_information();

        if let Some(token) = next_token {
            request = request.next_token(token);
        }

        let result = request
            .send()
            .await
            .map_err(|e| format!("Failed to get SSM status: {}", e))?;

        for info in result.instance_information_list() {
            if let (Some(id), Some(status)) = (info.instance_id(), info.ping_status()) {
                status_map.insert(id.to_string(), status.as_str().to_string());
            }
        }

        next_token = result.next_token().map(|s| s.to_string());
        if next_token.is_none() {
            break;
        }
    }

    Ok(status_map)
}

/// List all EC2 instances with SSM status
pub async fn list_instances(
    profile: &str,
    region: &str,
    ssm_only: bool,
) -> Result<Vec<Ec2Instance>, String> {
    let ec2_client = create_ec2_client(profile, region).await?;

    // Get SSM-managed instances and their status
    let ssm_instances = get_ssm_managed_instances(profile, region).await?;
    let ssm_status = get_ssm_ping_status(profile, region).await?;

    // List EC2 instances
    let mut instances = Vec::new();
    let mut next_token: Option<String> = None;

    loop {
        let mut request = ec2_client.describe_instances();

        // Only get running instances
        request = request.filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("instance-state-name")
                .values("running")
                .build(),
        );

        if let Some(token) = next_token {
            request = request.next_token(token);
        }

        let result = request
            .send()
            .await
            .map_err(|e| format!("Failed to list EC2 instances: {}", e))?;

        for reservation in result.reservations() {
            for instance in reservation.instances() {
                let instance_id = instance.instance_id().unwrap_or_default().to_string();
                let ssm_enabled = ssm_instances.contains(&instance_id);

                // Skip non-SSM instances if ssm_only is true
                if ssm_only && !ssm_enabled {
                    continue;
                }

                // Get instance name from tags
                let name = instance
                    .tags()
                    .iter()
                    .find(|t| t.key() == Some("Name"))
                    .and_then(|t| t.value())
                    .map(|s| s.to_string());

                instances.push(Ec2Instance {
                    instance_id: instance_id.clone(),
                    name,
                    instance_type: instance
                        .instance_type()
                        .map(|t| t.as_str().to_string())
                        .unwrap_or_default(),
                    state: instance
                        .state()
                        .and_then(|s| s.name())
                        .map(|n| n.as_str().to_string())
                        .unwrap_or_default(),
                    private_ip: instance.private_ip_address().map(|s| s.to_string()),
                    public_ip: instance.public_ip_address().map(|s| s.to_string()),
                    platform: instance
                        .platform_details()
                        .map(|s| s.to_string()),
                    ssm_enabled,
                    ssm_ping_status: ssm_status.get(&instance_id).cloned(),
                });
            }
        }

        next_token = result.next_token().map(|s| s.to_string());
        if next_token.is_none() {
            break;
        }
    }

    // Sort by name (instances with names first, then by instance ID)
    instances.sort_by(|a, b| {
        match (&a.name, &b.name) {
            (Some(name_a), Some(name_b)) => name_a.cmp(name_b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.instance_id.cmp(&b.instance_id),
        }
    });

    Ok(instances)
}

/// List only SSM-enabled instances
pub async fn list_ssm_instances(
    profile: &str,
    region: &str,
) -> Result<Vec<Ec2Instance>, String> {
    list_instances(profile, region, true).await
}
