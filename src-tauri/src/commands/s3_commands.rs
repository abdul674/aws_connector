use crate::aws::s3::{
    delete_object, download_object, get_object_content, get_presigned_url, head_object,
    list_buckets, list_objects, upload_object, S3Bucket, S3ListResult, S3Object,
};

/// List all S3 buckets
#[tauri::command]
pub async fn list_s3_buckets(profile: String, region: String) -> Result<Vec<S3Bucket>, String> {
    list_buckets(&profile, &region).await
}

/// List objects in an S3 bucket
#[tauri::command]
pub async fn list_s3_objects(
    profile: String,
    region: String,
    bucket: String,
    prefix: Option<String>,
    continuation_token: Option<String>,
    max_keys: Option<i32>,
) -> Result<S3ListResult, String> {
    list_objects(
        &profile,
        &region,
        &bucket,
        prefix.as_deref(),
        continuation_token.as_deref(),
        max_keys,
    )
    .await
}

/// Download an S3 object to a local file
#[tauri::command]
pub async fn download_s3_object(
    profile: String,
    region: String,
    bucket: String,
    key: String,
    local_path: String,
) -> Result<(), String> {
    download_object(&profile, &region, &bucket, &key, &local_path).await
}

/// Upload a local file to S3
#[tauri::command]
pub async fn upload_s3_object(
    profile: String,
    region: String,
    bucket: String,
    key: String,
    local_path: String,
) -> Result<(), String> {
    upload_object(&profile, &region, &bucket, &key, &local_path).await
}

/// Delete an S3 object
#[tauri::command]
pub async fn delete_s3_object(
    profile: String,
    region: String,
    bucket: String,
    key: String,
) -> Result<(), String> {
    delete_object(&profile, &region, &bucket, &key).await
}

/// Generate a presigned URL for an S3 object
#[tauri::command]
pub async fn get_s3_presigned_url(
    profile: String,
    region: String,
    bucket: String,
    key: String,
    expires_in_secs: u64,
) -> Result<String, String> {
    get_presigned_url(&profile, &region, &bucket, &key, expires_in_secs).await
}

/// Get object metadata
#[tauri::command]
pub async fn head_s3_object(
    profile: String,
    region: String,
    bucket: String,
    key: String,
) -> Result<S3Object, String> {
    head_object(&profile, &region, &bucket, &key).await
}

/// Get text content of an S3 object (for preview)
#[tauri::command]
pub async fn get_s3_object_content(
    profile: String,
    region: String,
    bucket: String,
    key: String,
    max_bytes: Option<i64>,
) -> Result<String, String> {
    get_object_content(&profile, &region, &bucket, &key, max_bytes).await
}
