use aws_config::BehaviorVersion;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::presigning::PresigningConfig;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Bucket {
    pub name: String,
    pub creation_date: Option<i64>,
    pub region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Object {
    pub key: String,
    pub size: i64,
    pub last_modified: Option<i64>,
    pub storage_class: Option<String>,
    pub is_folder: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3ListResult {
    pub objects: Vec<S3Object>,
    pub common_prefixes: Vec<String>,
    pub is_truncated: bool,
    pub next_continuation_token: Option<String>,
}

/// Create an S3 client with the specified profile and region
async fn create_s3_client(profile: &str, region: &str) -> Result<S3Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .profile_name(profile)
        .region(aws_config::Region::new(region.to_string()))
        .load()
        .await;

    Ok(S3Client::new(&config))
}

/// List all S3 buckets
pub async fn list_buckets(profile: &str, region: &str) -> Result<Vec<S3Bucket>, String> {
    let client = create_s3_client(profile, region).await?;

    let result = client
        .list_buckets()
        .send()
        .await
        .map_err(|e| format!("Failed to list S3 buckets: {}", e))?;

    let buckets = result
        .buckets()
        .iter()
        .map(|b| S3Bucket {
            name: b.name().unwrap_or_default().to_string(),
            creation_date: b.creation_date().map(|d| d.as_secs_f64() as i64 * 1000),
            region: None, // Would need separate call to get bucket location
        })
        .collect();

    Ok(buckets)
}

/// List objects in an S3 bucket with optional prefix
pub async fn list_objects(
    profile: &str,
    region: &str,
    bucket: &str,
    prefix: Option<&str>,
    continuation_token: Option<&str>,
    max_keys: Option<i32>,
) -> Result<S3ListResult, String> {
    let client = create_s3_client(profile, region).await?;

    let mut request = client
        .list_objects_v2()
        .bucket(bucket)
        .delimiter("/"); // Use delimiter to get "folder" structure

    if let Some(p) = prefix {
        request = request.prefix(p);
    }

    if let Some(token) = continuation_token {
        request = request.continuation_token(token);
    }

    if let Some(max) = max_keys {
        request = request.max_keys(max);
    }

    let result = request
        .send()
        .await
        .map_err(|e| format!("Failed to list S3 objects: {}", e))?;

    let objects: Vec<S3Object> = result
        .contents()
        .iter()
        .map(|obj| S3Object {
            key: obj.key().unwrap_or_default().to_string(),
            size: obj.size().unwrap_or(0),
            last_modified: obj.last_modified().map(|d| (d.as_secs_f64() * 1000.0) as i64),
            storage_class: obj.storage_class().map(|s| s.as_str().to_string()),
            is_folder: false,
        })
        .collect();

    let common_prefixes: Vec<String> = result
        .common_prefixes()
        .iter()
        .filter_map(|cp| cp.prefix().map(|s| s.to_string()))
        .collect();

    Ok(S3ListResult {
        objects,
        common_prefixes,
        is_truncated: result.is_truncated().unwrap_or(false),
        next_continuation_token: result.next_continuation_token().map(|s| s.to_string()),
    })
}

/// Download an S3 object to a local file
pub async fn download_object(
    profile: &str,
    region: &str,
    bucket: &str,
    key: &str,
    local_path: &str,
) -> Result<(), String> {
    let client = create_s3_client(profile, region).await?;

    let result = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| format!("Failed to download S3 object: {}", e))?;

    let body = result
        .body
        .collect()
        .await
        .map_err(|e| format!("Failed to read S3 object body: {}", e))?;

    let bytes = body.into_bytes();

    // Ensure parent directory exists
    if let Some(parent) = Path::new(local_path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    std::fs::write(local_path, bytes)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

/// Upload a local file to S3
pub async fn upload_object(
    profile: &str,
    region: &str,
    bucket: &str,
    key: &str,
    local_path: &str,
) -> Result<(), String> {
    let client = create_s3_client(profile, region).await?;

    let body = std::fs::read(local_path)
        .map_err(|e| format!("Failed to read local file: {}", e))?;

    let body = aws_sdk_s3::primitives::ByteStream::from(body);

    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await
        .map_err(|e| format!("Failed to upload S3 object: {}", e))?;

    Ok(())
}

/// Delete an S3 object
pub async fn delete_object(
    profile: &str,
    region: &str,
    bucket: &str,
    key: &str,
) -> Result<(), String> {
    let client = create_s3_client(profile, region).await?;

    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| format!("Failed to delete S3 object: {}", e))?;

    Ok(())
}

/// Generate a presigned URL for an S3 object
pub async fn get_presigned_url(
    profile: &str,
    region: &str,
    bucket: &str,
    key: &str,
    expires_in_secs: u64,
) -> Result<String, String> {
    let client = create_s3_client(profile, region).await?;

    let presigning_config = PresigningConfig::expires_in(Duration::from_secs(expires_in_secs))
        .map_err(|e| format!("Failed to create presigning config: {}", e))?;

    let presigned = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .presigned(presigning_config)
        .await
        .map_err(|e| format!("Failed to generate presigned URL: {}", e))?;

    Ok(presigned.uri().to_string())
}

/// Get object metadata without downloading the content
pub async fn head_object(
    profile: &str,
    region: &str,
    bucket: &str,
    key: &str,
) -> Result<S3Object, String> {
    let client = create_s3_client(profile, region).await?;

    let result = client
        .head_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| format!("Failed to get S3 object metadata: {}", e))?;

    Ok(S3Object {
        key: key.to_string(),
        size: result.content_length().unwrap_or(0),
        last_modified: result.last_modified().map(|d| (d.as_secs_f64() * 1000.0) as i64),
        storage_class: result.storage_class().map(|s| s.as_str().to_string()),
        is_folder: false,
    })
}

/// Get the content of a small text file (for preview)
pub async fn get_object_content(
    profile: &str,
    region: &str,
    bucket: &str,
    key: &str,
    max_bytes: Option<i64>,
) -> Result<String, String> {
    let client = create_s3_client(profile, region).await?;

    let mut request = client.get_object().bucket(bucket).key(key);

    // Limit the range if max_bytes is specified
    if let Some(max) = max_bytes {
        request = request.range(format!("bytes=0-{}", max - 1));
    }

    let result = request
        .send()
        .await
        .map_err(|e| format!("Failed to get S3 object: {}", e))?;

    let body = result
        .body
        .collect()
        .await
        .map_err(|e| format!("Failed to read S3 object body: {}", e))?;

    let bytes = body.into_bytes();

    String::from_utf8(bytes.to_vec())
        .map_err(|_| "Object content is not valid UTF-8 text".to_string())
}
