import { invoke } from '@tauri-apps/api/core';
import type { S3Bucket, S3Object, S3ListResult } from '$lib/types/s3';

/**
 * List all S3 buckets
 */
export async function listBuckets(profile: string, region: string): Promise<S3Bucket[]> {
  return invoke<S3Bucket[]>('list_s3_buckets', { profile, region });
}

/**
 * List objects in an S3 bucket
 */
export async function listObjects(
  profile: string,
  region: string,
  bucket: string,
  options?: {
    prefix?: string;
    continuationToken?: string;
    maxKeys?: number;
  }
): Promise<S3ListResult> {
  return invoke<S3ListResult>('list_s3_objects', {
    profile,
    region,
    bucket,
    prefix: options?.prefix ?? null,
    continuationToken: options?.continuationToken ?? null,
    maxKeys: options?.maxKeys ?? null,
  });
}

/**
 * Download an S3 object to a local file
 */
export async function downloadObject(
  profile: string,
  region: string,
  bucket: string,
  key: string,
  localPath: string
): Promise<void> {
  return invoke<void>('download_s3_object', {
    profile,
    region,
    bucket,
    key,
    localPath,
  });
}

/**
 * Upload a local file to S3
 */
export async function uploadObject(
  profile: string,
  region: string,
  bucket: string,
  key: string,
  localPath: string
): Promise<void> {
  return invoke<void>('upload_s3_object', {
    profile,
    region,
    bucket,
    key,
    localPath,
  });
}

/**
 * Delete an S3 object
 */
export async function deleteObject(
  profile: string,
  region: string,
  bucket: string,
  key: string
): Promise<void> {
  return invoke<void>('delete_s3_object', {
    profile,
    region,
    bucket,
    key,
  });
}

/**
 * Get a presigned URL for an S3 object
 */
export async function getPresignedUrl(
  profile: string,
  region: string,
  bucket: string,
  key: string,
  expiresInSecs: number = 3600
): Promise<string> {
  return invoke<string>('get_s3_presigned_url', {
    profile,
    region,
    bucket,
    key,
    expiresInSecs,
  });
}

/**
 * Get object metadata
 */
export async function headObject(
  profile: string,
  region: string,
  bucket: string,
  key: string
): Promise<S3Object> {
  return invoke<S3Object>('head_s3_object', {
    profile,
    region,
    bucket,
    key,
  });
}

/**
 * Get text content of an S3 object (for preview)
 */
export async function getObjectContent(
  profile: string,
  region: string,
  bucket: string,
  key: string,
  maxBytes?: number
): Promise<string> {
  return invoke<string>('get_s3_object_content', {
    profile,
    region,
    bucket,
    key,
    maxBytes: maxBytes ?? null,
  });
}

/**
 * Helper to format file size
 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
}

/**
 * Helper to get file extension from key
 */
export function getFileExtension(key: string): string {
  const parts = key.split('.');
  return parts.length > 1 ? parts[parts.length - 1].toLowerCase() : '';
}

/**
 * Helper to check if file is previewable
 */
export function isPreviewable(key: string): boolean {
  const ext = getFileExtension(key);
  const previewableExtensions = [
    'txt', 'md', 'json', 'xml', 'html', 'css', 'js', 'ts', 'py', 'rb',
    'java', 'go', 'rs', 'sh', 'yaml', 'yml', 'toml', 'ini', 'conf', 'log',
    'csv', 'sql', 'env', 'gitignore', 'dockerfile'
  ];
  return previewableExtensions.includes(ext);
}

/**
 * Helper to get file icon based on extension
 */
export function getFileIcon(key: string, isFolder: boolean): string {
  if (isFolder) return 'folder';

  const ext = getFileExtension(key);
  const iconMap: Record<string, string> = {
    // Images
    'jpg': 'image', 'jpeg': 'image', 'png': 'image', 'gif': 'image',
    'svg': 'image', 'webp': 'image', 'ico': 'image',
    // Documents
    'pdf': 'pdf', 'doc': 'document', 'docx': 'document',
    // Archives
    'zip': 'archive', 'tar': 'archive', 'gz': 'archive', 'rar': 'archive',
    // Code
    'js': 'code', 'ts': 'code', 'py': 'code', 'rb': 'code', 'java': 'code',
    'go': 'code', 'rs': 'code', 'cpp': 'code', 'c': 'code', 'h': 'code',
    // Data
    'json': 'data', 'xml': 'data', 'csv': 'data', 'yaml': 'data', 'yml': 'data',
    // Config
    'env': 'config', 'ini': 'config', 'conf': 'config', 'toml': 'config',
  };

  return iconMap[ext] || 'file';
}
