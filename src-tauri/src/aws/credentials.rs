use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
#[allow(dead_code)]
pub enum CredentialsError {
    #[error("AWS credentials file not found at {0}")]
    FileNotFound(String),

    #[error("Failed to read credentials file: {0}")]
    ReadError(String),

    #[error("Failed to parse credentials file: {0}")]
    ParseError(String),

    #[error("Home directory not found")]
    HomeDirNotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsProfile {
    pub name: String,
    pub region: Option<String>,
    pub source: ProfileSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_start_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProfileSource {
    Credentials,
    Config,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsRegion {
    pub code: String,
    pub name: String,
}

/// Get the path to the AWS credentials file
fn get_credentials_path() -> Result<PathBuf, CredentialsError> {
    // Check for environment variable override
    if let Ok(path) = std::env::var("AWS_SHARED_CREDENTIALS_FILE") {
        return Ok(PathBuf::from(path));
    }

    let home = dirs::home_dir().ok_or(CredentialsError::HomeDirNotFound)?;
    Ok(home.join(".aws").join("credentials"))
}

/// Get the path to the AWS config file
fn get_config_path() -> Result<PathBuf, CredentialsError> {
    // Check for environment variable override
    if let Ok(path) = std::env::var("AWS_CONFIG_FILE") {
        return Ok(PathBuf::from(path));
    }

    let home = dirs::home_dir().ok_or(CredentialsError::HomeDirNotFound)?;
    Ok(home.join(".aws").join("config"))
}

/// Parse an INI-style file into sections
fn parse_ini_file(content: &str) -> HashMap<String, HashMap<String, String>> {
    let mut sections: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        // Check for section header
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len() - 1].to_string();
            // Handle "profile xxx" format in config file
            if current_section.starts_with("profile ") {
                current_section = current_section["profile ".len()..].to_string();
            }
            sections.entry(current_section.clone()).or_default();
            continue;
        }

        // Parse key=value pairs
        if let Some(pos) = line.find('=') {
            let key = line[..pos].trim().to_string();
            let value = line[pos + 1..].trim().to_string();

            if !current_section.is_empty() {
                sections
                    .entry(current_section.clone())
                    .or_default()
                    .insert(key, value);
            }
        }
    }

    sections
}

/// List all AWS profiles from credentials and config files
pub fn list_profiles() -> Result<Vec<AwsProfile>, CredentialsError> {
    let mut profiles: HashMap<String, AwsProfile> = HashMap::new();

    // Parse credentials file
    let credentials_path = get_credentials_path()?;
    if credentials_path.exists() {
        let content = fs::read_to_string(&credentials_path)
            .map_err(|e| CredentialsError::ReadError(e.to_string()))?;

        let sections = parse_ini_file(&content);

        for (name, _props) in sections {
            profiles.insert(
                name.clone(),
                AwsProfile {
                    name: name.clone(),
                    region: None,
                    source: ProfileSource::Credentials,
                    sso_start_url: None,
                    sso_region: None,
                    role_arn: None,
                },
            );
        }
    }

    // Parse config file
    let config_path = get_config_path()?;
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .map_err(|e| CredentialsError::ReadError(e.to_string()))?;

        let sections = parse_ini_file(&content);

        for (name, props) in sections {
            let region = props.get("region").cloned();
            let sso_start_url = props.get("sso_start_url").cloned();
            let sso_region = props.get("sso_region").cloned();
            let role_arn = props.get("role_arn").cloned();

            if let Some(existing) = profiles.get_mut(&name) {
                // Profile exists in credentials, update with config data
                existing.region = region.or(existing.region.clone());
                existing.sso_start_url = sso_start_url;
                existing.sso_region = sso_region;
                existing.role_arn = role_arn;
                existing.source = ProfileSource::Both;
            } else {
                // Profile only in config
                profiles.insert(
                    name.clone(),
                    AwsProfile {
                        name: name.clone(),
                        region,
                        source: ProfileSource::Config,
                        sso_start_url,
                        sso_region,
                        role_arn,
                    },
                );
            }
        }
    }

    // Convert to vector and sort by name
    let mut result: Vec<AwsProfile> = profiles.into_values().collect();
    result.sort_by(|a, b| {
        // Put "default" first, then sort alphabetically
        if a.name == "default" {
            std::cmp::Ordering::Less
        } else if b.name == "default" {
            std::cmp::Ordering::Greater
        } else {
            a.name.cmp(&b.name)
        }
    });

    Ok(result)
}

/// Get common AWS regions
pub fn get_regions() -> Vec<AwsRegion> {
    vec![
        AwsRegion { code: "us-east-1".to_string(), name: "US East (N. Virginia)".to_string() },
        AwsRegion { code: "us-east-2".to_string(), name: "US East (Ohio)".to_string() },
        AwsRegion { code: "us-west-1".to_string(), name: "US West (N. California)".to_string() },
        AwsRegion { code: "us-west-2".to_string(), name: "US West (Oregon)".to_string() },
        AwsRegion { code: "af-south-1".to_string(), name: "Africa (Cape Town)".to_string() },
        AwsRegion { code: "ap-east-1".to_string(), name: "Asia Pacific (Hong Kong)".to_string() },
        AwsRegion { code: "ap-south-1".to_string(), name: "Asia Pacific (Mumbai)".to_string() },
        AwsRegion { code: "ap-south-2".to_string(), name: "Asia Pacific (Hyderabad)".to_string() },
        AwsRegion { code: "ap-southeast-1".to_string(), name: "Asia Pacific (Singapore)".to_string() },
        AwsRegion { code: "ap-southeast-2".to_string(), name: "Asia Pacific (Sydney)".to_string() },
        AwsRegion { code: "ap-southeast-3".to_string(), name: "Asia Pacific (Jakarta)".to_string() },
        AwsRegion { code: "ap-northeast-1".to_string(), name: "Asia Pacific (Tokyo)".to_string() },
        AwsRegion { code: "ap-northeast-2".to_string(), name: "Asia Pacific (Seoul)".to_string() },
        AwsRegion { code: "ap-northeast-3".to_string(), name: "Asia Pacific (Osaka)".to_string() },
        AwsRegion { code: "ca-central-1".to_string(), name: "Canada (Central)".to_string() },
        AwsRegion { code: "eu-central-1".to_string(), name: "Europe (Frankfurt)".to_string() },
        AwsRegion { code: "eu-central-2".to_string(), name: "Europe (Zurich)".to_string() },
        AwsRegion { code: "eu-west-1".to_string(), name: "Europe (Ireland)".to_string() },
        AwsRegion { code: "eu-west-2".to_string(), name: "Europe (London)".to_string() },
        AwsRegion { code: "eu-west-3".to_string(), name: "Europe (Paris)".to_string() },
        AwsRegion { code: "eu-south-1".to_string(), name: "Europe (Milan)".to_string() },
        AwsRegion { code: "eu-south-2".to_string(), name: "Europe (Spain)".to_string() },
        AwsRegion { code: "eu-north-1".to_string(), name: "Europe (Stockholm)".to_string() },
        AwsRegion { code: "me-south-1".to_string(), name: "Middle East (Bahrain)".to_string() },
        AwsRegion { code: "me-central-1".to_string(), name: "Middle East (UAE)".to_string() },
        AwsRegion { code: "sa-east-1".to_string(), name: "South America (São Paulo)".to_string() },
    ]
}

/// Input for adding a new profile with access keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddProfileInput {
    pub name: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub region: String,
    pub session_token: Option<String>,
}

/// Input for adding an SSO profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSsoProfileInput {
    pub name: String,
    pub sso_start_url: String,
    pub sso_region: String,
    pub sso_account_id: String,
    pub sso_role_name: String,
    pub region: String,
}

/// Ensure ~/.aws directory exists
fn ensure_aws_dir() -> Result<PathBuf, CredentialsError> {
    let home = dirs::home_dir().ok_or(CredentialsError::HomeDirNotFound)?;
    let aws_dir = home.join(".aws");

    if !aws_dir.exists() {
        fs::create_dir_all(&aws_dir)
            .map_err(|e| CredentialsError::ReadError(format!("Failed to create ~/.aws: {}", e)))?;
    }

    Ok(aws_dir)
}

/// Check if a profile already exists
pub fn profile_exists(name: &str) -> Result<bool, CredentialsError> {
    let profiles = list_profiles()?;
    Ok(profiles.iter().any(|p| p.name == name))
}

/// Add a new profile with access key credentials
/// This ONLY adds new profiles, never modifies existing ones
pub fn add_profile(input: AddProfileInput) -> Result<(), CredentialsError> {
    // Check if profile already exists
    if profile_exists(&input.name)? {
        return Err(CredentialsError::ParseError(format!(
            "Profile '{}' already exists. Choose a different name.",
            input.name
        )));
    }

    ensure_aws_dir()?;

    // Add to credentials file
    let credentials_path = get_credentials_path()?;
    let mut credentials_content = if credentials_path.exists() {
        fs::read_to_string(&credentials_path)
            .map_err(|e| CredentialsError::ReadError(e.to_string()))?
    } else {
        String::new()
    };

    // Ensure file ends with newline
    if !credentials_content.is_empty() && !credentials_content.ends_with('\n') {
        credentials_content.push('\n');
    }

    // Add newline before new profile if file has content
    if !credentials_content.is_empty() {
        credentials_content.push('\n');
    }

    // Add credentials
    credentials_content.push_str(&format!("[{}]\n", input.name));
    credentials_content.push_str(&format!("aws_access_key_id = {}\n", input.access_key_id));
    credentials_content.push_str(&format!("aws_secret_access_key = {}\n", input.secret_access_key));

    if let Some(token) = &input.session_token {
        credentials_content.push_str(&format!("aws_session_token = {}\n", token));
    }

    fs::write(&credentials_path, credentials_content)
        .map_err(|e| CredentialsError::ReadError(format!("Failed to write credentials: {}", e)))?;

    // Add to config file
    let config_path = get_config_path()?;
    let mut config_content = if config_path.exists() {
        fs::read_to_string(&config_path)
            .map_err(|e| CredentialsError::ReadError(e.to_string()))?
    } else {
        String::new()
    };

    // Ensure file ends with newline
    if !config_content.is_empty() && !config_content.ends_with('\n') {
        config_content.push('\n');
    }

    if !config_content.is_empty() {
        config_content.push('\n');
    }

    // Add config (use "profile name" format for non-default profiles)
    if input.name == "default" {
        config_content.push_str("[default]\n");
    } else {
        config_content.push_str(&format!("[profile {}]\n", input.name));
    }
    config_content.push_str(&format!("region = {}\n", input.region));
    config_content.push_str("output = json\n");

    fs::write(&config_path, config_content)
        .map_err(|e| CredentialsError::ReadError(format!("Failed to write config: {}", e)))?;

    Ok(())
}

/// Add a new SSO profile
/// This ONLY adds new profiles, never modifies existing ones
pub fn add_sso_profile(input: AddSsoProfileInput) -> Result<(), CredentialsError> {
    // Check if profile already exists
    if profile_exists(&input.name)? {
        return Err(CredentialsError::ParseError(format!(
            "Profile '{}' already exists. Choose a different name.",
            input.name
        )));
    }

    ensure_aws_dir()?;

    // SSO profiles only need config file entry
    let config_path = get_config_path()?;
    let mut config_content = if config_path.exists() {
        fs::read_to_string(&config_path)
            .map_err(|e| CredentialsError::ReadError(e.to_string()))?
    } else {
        String::new()
    };

    // Ensure file ends with newline
    if !config_content.is_empty() && !config_content.ends_with('\n') {
        config_content.push('\n');
    }

    if !config_content.is_empty() {
        config_content.push('\n');
    }

    // Add SSO config
    if input.name == "default" {
        config_content.push_str("[default]\n");
    } else {
        config_content.push_str(&format!("[profile {}]\n", input.name));
    }
    config_content.push_str(&format!("sso_start_url = {}\n", input.sso_start_url));
    config_content.push_str(&format!("sso_region = {}\n", input.sso_region));
    config_content.push_str(&format!("sso_account_id = {}\n", input.sso_account_id));
    config_content.push_str(&format!("sso_role_name = {}\n", input.sso_role_name));
    config_content.push_str(&format!("region = {}\n", input.region));
    config_content.push_str("output = json\n");

    fs::write(&config_path, config_content)
        .map_err(|e| CredentialsError::ReadError(format!("Failed to write config: {}", e)))?;

    Ok(())
}

/// Delete a profile (removes from both credentials and config)
pub fn delete_profile(name: &str) -> Result<(), CredentialsError> {
    // Remove from credentials file
    let credentials_path = get_credentials_path()?;
    if credentials_path.exists() {
        let content = fs::read_to_string(&credentials_path)
            .map_err(|e| CredentialsError::ReadError(e.to_string()))?;

        let new_content = remove_section_from_ini(&content, name, false);
        fs::write(&credentials_path, new_content)
            .map_err(|e| CredentialsError::ReadError(format!("Failed to write credentials: {}", e)))?;
    }

    // Remove from config file
    let config_path = get_config_path()?;
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .map_err(|e| CredentialsError::ReadError(e.to_string()))?;

        let new_content = remove_section_from_ini(&content, name, true);
        fs::write(&config_path, new_content)
            .map_err(|e| CredentialsError::ReadError(format!("Failed to write config: {}", e)))?;
    }

    Ok(())
}

/// Remove a section from INI content
fn remove_section_from_ini(content: &str, section_name: &str, is_config: bool) -> String {
    let mut result = String::new();
    let mut skip_section = false;

    let section_header = if is_config && section_name != "default" {
        format!("[profile {}]", section_name)
    } else {
        format!("[{}]", section_name)
    };

    for line in content.lines() {
        let trimmed = line.trim();

        // Check if this is a section header
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            if trimmed == section_header {
                skip_section = true;
                continue;
            } else {
                skip_section = false;
            }
        }

        if !skip_section {
            result.push_str(line);
            result.push('\n');
        }
    }

    // Remove trailing empty lines but keep one newline at end
    result = result.trim_end().to_string();
    if !result.is_empty() {
        result.push('\n');
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ini_file() {
        let content = r#"
[default]
aws_access_key_id = AKIAIOSFODNN7EXAMPLE
aws_secret_access_key = wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY

[profile myprofile]
region = us-west-2
output = json
"#;

        let sections = parse_ini_file(content);

        assert!(sections.contains_key("default"));
        assert!(sections.contains_key("myprofile"));

        let default = sections.get("default").unwrap();
        assert_eq!(default.get("aws_access_key_id").unwrap(), "AKIAIOSFODNN7EXAMPLE");
    }

    #[test]
    fn test_get_regions() {
        let regions = get_regions();
        assert!(!regions.is_empty());
        assert!(regions.iter().any(|r| r.code == "us-east-1"));
    }
}
