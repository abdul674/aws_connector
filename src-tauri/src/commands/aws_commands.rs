use crate::aws::{
    add_profile, add_sso_profile, delete_profile, get_regions as get_aws_regions,
    list_profiles as get_profiles, profile_exists, AddProfileInput, AddSsoProfileInput,
    AwsProfile, AwsRegion,
};

/// List all available AWS profiles from credentials and config files
#[tauri::command]
pub async fn list_aws_profiles() -> Result<Vec<AwsProfile>, String> {
    get_profiles().map_err(|e| e.to_string())
}

/// Get list of AWS regions
#[tauri::command]
pub async fn list_aws_regions() -> Vec<AwsRegion> {
    get_aws_regions()
}

/// Check if AWS CLI is installed and accessible
#[tauri::command]
pub async fn check_aws_cli() -> Result<String, String> {
    let output = std::process::Command::new("aws")
        .arg("--version")
        .output()
        .map_err(|e| format!("Failed to execute AWS CLI: {}", e))?;

    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(version.trim().to_string())
    } else {
        Err("AWS CLI not found or not working properly".to_string())
    }
}

/// Check if session-manager-plugin is installed
#[tauri::command]
pub async fn check_ssm_plugin() -> Result<String, String> {
    let output = std::process::Command::new("session-manager-plugin")
        .arg("--version")
        .output()
        .map_err(|_| "session-manager-plugin not found. Please install it from: https://docs.aws.amazon.com/systems-manager/latest/userguide/session-manager-working-with-install-plugin.html".to_string())?;

    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(version.trim().to_string())
    } else {
        Err("session-manager-plugin not working properly".to_string())
    }
}

/// Get the default region for a profile
#[tauri::command]
pub async fn get_profile_region(profile_name: String) -> Option<String> {
    let profiles = get_profiles().ok()?;
    profiles
        .into_iter()
        .find(|p| p.name == profile_name)
        .and_then(|p| p.region)
}

/// Check if a profile name already exists
#[tauri::command]
pub async fn check_profile_exists(name: String) -> Result<bool, String> {
    profile_exists(&name).map_err(|e| e.to_string())
}

/// Add a new AWS profile with access key credentials
#[tauri::command]
pub async fn add_aws_profile(
    name: String,
    access_key_id: String,
    secret_access_key: String,
    region: String,
    session_token: Option<String>,
) -> Result<(), String> {
    add_profile(AddProfileInput {
        name,
        access_key_id,
        secret_access_key,
        region,
        session_token,
    })
    .map_err(|e| e.to_string())
}

/// Add a new AWS SSO profile
#[tauri::command]
pub async fn add_aws_sso_profile(
    name: String,
    sso_start_url: String,
    sso_region: String,
    sso_account_id: String,
    sso_role_name: String,
    region: String,
) -> Result<(), String> {
    add_sso_profile(AddSsoProfileInput {
        name,
        sso_start_url,
        sso_region,
        sso_account_id,
        sso_role_name,
        region,
    })
    .map_err(|e| e.to_string())
}

/// Delete an AWS profile
#[tauri::command]
pub async fn delete_aws_profile(name: String) -> Result<(), String> {
    delete_profile(&name).map_err(|e| e.to_string())
}

/// Run AWS SSO login for a profile
#[tauri::command]
pub async fn sso_login(profile_name: String) -> Result<String, String> {
    let output = std::process::Command::new("aws")
        .args(["sso", "login", "--profile", &profile_name])
        .output()
        .map_err(|e| format!("Failed to run SSO login: {}", e))?;

    if output.status.success() {
        Ok("SSO login successful".to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("SSO login failed: {}", stderr))
    }
}

/// Validate AWS credentials by calling STS get-caller-identity
#[tauri::command]
pub async fn validate_credentials(profile_name: String) -> Result<String, String> {
    let output = std::process::Command::new("aws")
        .args([
            "sts",
            "get-caller-identity",
            "--profile",
            &profile_name,
            "--output",
            "json",
        ])
        .output()
        .map_err(|e| format!("Failed to validate credentials: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Invalid credentials: {}", stderr))
    }
}
