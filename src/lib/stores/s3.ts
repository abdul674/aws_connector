import { writable, derived, get } from 'svelte/store';
import type { S3Bucket, S3Object, S3BrowserState } from '$lib/types/s3';
import {
  listBuckets as apiListBuckets,
  listObjects as apiListObjects,
  downloadObject as apiDownloadObject,
  deleteObject as apiDeleteObject,
  getObjectContent as apiGetObjectContent,
  getPresignedUrl as apiGetPresignedUrl,
} from '$lib/api/s3';
import { selectedProfile, selectedRegion } from './profiles';

// Store for S3 buckets
export const s3Buckets = writable<S3Bucket[]>([]);
export const s3BucketsLoading = writable<boolean>(false);
export const s3BucketsError = writable<string | null>(null);

// Store for the currently selected bucket
export const selectedBucket = writable<string | null>(null);

// Store for browser state (current folder, objects, etc.)
export const s3BrowserState = writable<S3BrowserState>({
  bucket: '',
  prefix: '',
  objects: [],
  folders: [],
  loading: false,
  error: null,
});

// Store for active S3 browser session
export const s3BrowserActive = writable<boolean>(false);

// Navigation history for back/forward
export const s3NavigationHistory = writable<string[]>([]);
export const s3NavigationIndex = writable<number>(-1);

// Derived stores
export const hasS3Buckets = derived(s3Buckets, ($buckets) => $buckets.length > 0);

export const currentS3Path = derived(s3BrowserState, ($state) => {
  if (!$state.bucket) return '';
  return $state.prefix ? `s3://${$state.bucket}/${$state.prefix}` : `s3://${$state.bucket}/`;
});

export const s3Breadcrumbs = derived(s3BrowserState, ($state) => {
  const parts: { name: string; prefix: string }[] = [];

  if (!$state.bucket) return parts;

  // Add bucket as root
  parts.push({ name: $state.bucket, prefix: '' });

  // Add each folder in the path
  if ($state.prefix) {
    const pathParts = $state.prefix.split('/').filter(Boolean);
    let currentPrefix = '';
    for (const part of pathParts) {
      currentPrefix += part + '/';
      parts.push({ name: part, prefix: currentPrefix });
    }
  }

  return parts;
});

/**
 * Load S3 buckets
 */
export async function loadS3Buckets(): Promise<void> {
  const profile = get(selectedProfile);
  const region = get(selectedRegion);

  if (!profile || !region) {
    return;
  }

  s3BucketsLoading.set(true);
  s3BucketsError.set(null);

  try {
    const buckets = await apiListBuckets(profile, region);
    s3Buckets.set(buckets);
  } catch (error) {
    s3BucketsError.set(String(error));
    console.error('Failed to load S3 buckets:', error);
  } finally {
    s3BucketsLoading.set(false);
  }
}

/**
 * Open an S3 bucket in the browser
 */
export async function openS3Bucket(bucket: string): Promise<void> {
  selectedBucket.set(bucket);
  s3BrowserActive.set(true);

  // Reset navigation
  s3NavigationHistory.set([]);
  s3NavigationIndex.set(-1);

  await navigateToPrefix(bucket, '');
}

/**
 * Navigate to a prefix (folder) in the current bucket
 */
export async function navigateToPrefix(bucket: string, prefix: string): Promise<void> {
  const profile = get(selectedProfile);
  const region = get(selectedRegion);

  if (!profile || !region) {
    return;
  }

  s3BrowserState.update((state) => ({
    ...state,
    bucket,
    prefix,
    loading: true,
    error: null,
  }));

  try {
    const result = await apiListObjects(profile, region, bucket, { prefix });

    // Extract folder names from common prefixes
    const folders = result.common_prefixes.map((p) => {
      // Remove the current prefix and trailing slash to get folder name
      const folderPath = p.replace(prefix, '');
      return folderPath.replace(/\/$/, '');
    });

    // Filter out objects that are just the folder marker
    const objects = result.objects.filter((obj) => {
      // Skip if it's the current prefix itself
      if (obj.key === prefix) return false;
      // Skip folder markers (keys ending with /)
      if (obj.key.endsWith('/')) return false;
      return true;
    });

    s3BrowserState.update((state) => ({
      ...state,
      objects,
      folders,
      loading: false,
    }));

    // Add to navigation history
    const path = `${bucket}:${prefix}`;
    s3NavigationHistory.update((history) => {
      const currentIndex = get(s3NavigationIndex);
      // Remove forward history if navigating from middle
      const newHistory = history.slice(0, currentIndex + 1);
      newHistory.push(path);
      return newHistory;
    });
    s3NavigationIndex.update((n) => n + 1);

  } catch (error) {
    s3BrowserState.update((state) => ({
      ...state,
      loading: false,
      error: String(error),
    }));
    console.error('Failed to list S3 objects:', error);
  }
}

/**
 * Navigate into a folder
 */
export async function enterFolder(folderName: string): Promise<void> {
  const state = get(s3BrowserState);
  const newPrefix = state.prefix + folderName + '/';
  await navigateToPrefix(state.bucket, newPrefix);
}

/**
 * Navigate up one level
 */
export async function navigateUp(): Promise<void> {
  const state = get(s3BrowserState);
  if (!state.prefix) return;

  // Remove last folder from prefix
  const parts = state.prefix.split('/').filter(Boolean);
  parts.pop();
  const newPrefix = parts.length > 0 ? parts.join('/') + '/' : '';

  await navigateToPrefix(state.bucket, newPrefix);
}

/**
 * Navigate back in history
 */
export async function navigateBack(): Promise<void> {
  const history = get(s3NavigationHistory);
  const currentIndex = get(s3NavigationIndex);

  if (currentIndex <= 0) return;

  const newIndex = currentIndex - 1;
  const path = history[newIndex];
  const [bucket, prefix] = path.split(':');

  s3NavigationIndex.set(newIndex);

  // Navigate without adding to history
  const profile = get(selectedProfile);
  const region = get(selectedRegion);

  if (!profile || !region) return;

  s3BrowserState.update((state) => ({
    ...state,
    bucket,
    prefix: prefix || '',
    loading: true,
    error: null,
  }));

  try {
    const result = await apiListObjects(profile, region, bucket, { prefix: prefix || undefined });

    const folders = result.common_prefixes.map((p) => {
      const folderPath = p.replace(prefix || '', '');
      return folderPath.replace(/\/$/, '');
    });

    const objects = result.objects.filter((obj) => {
      if (obj.key === prefix) return false;
      if (obj.key.endsWith('/')) return false;
      return true;
    });

    s3BrowserState.update((state) => ({
      ...state,
      objects,
      folders,
      loading: false,
    }));
  } catch (error) {
    s3BrowserState.update((state) => ({
      ...state,
      loading: false,
      error: String(error),
    }));
  }
}

/**
 * Navigate forward in history
 */
export async function navigateForward(): Promise<void> {
  const history = get(s3NavigationHistory);
  const currentIndex = get(s3NavigationIndex);

  if (currentIndex >= history.length - 1) return;

  const newIndex = currentIndex + 1;
  const path = history[newIndex];
  const [bucket, prefix] = path.split(':');

  s3NavigationIndex.set(newIndex);

  // Navigate without adding to history
  const profile = get(selectedProfile);
  const region = get(selectedRegion);

  if (!profile || !region) return;

  s3BrowserState.update((state) => ({
    ...state,
    bucket,
    prefix: prefix || '',
    loading: true,
    error: null,
  }));

  try {
    const result = await apiListObjects(profile, region, bucket, { prefix: prefix || undefined });

    const folders = result.common_prefixes.map((p) => {
      const folderPath = p.replace(prefix || '', '');
      return folderPath.replace(/\/$/, '');
    });

    const objects = result.objects.filter((obj) => {
      if (obj.key === prefix) return false;
      if (obj.key.endsWith('/')) return false;
      return true;
    });

    s3BrowserState.update((state) => ({
      ...state,
      objects,
      folders,
      loading: false,
    }));
  } catch (error) {
    s3BrowserState.update((state) => ({
      ...state,
      loading: false,
      error: String(error),
    }));
  }
}

/**
 * Refresh current folder
 */
export async function refreshS3Browser(): Promise<void> {
  const state = get(s3BrowserState);
  if (!state.bucket) return;

  await navigateToPrefix(state.bucket, state.prefix);
}

/**
 * Download an S3 object
 */
export async function downloadS3Object(key: string, localPath: string): Promise<void> {
  const profile = get(selectedProfile);
  const region = get(selectedRegion);
  const bucket = get(selectedBucket);

  if (!profile || !region || !bucket) {
    throw new Error('Profile, region, and bucket are required');
  }

  await apiDownloadObject(profile, region, bucket, key, localPath);
}

/**
 * Delete an S3 object
 */
export async function deleteS3Object(key: string): Promise<void> {
  const profile = get(selectedProfile);
  const region = get(selectedRegion);
  const bucket = get(selectedBucket);

  if (!profile || !region || !bucket) {
    throw new Error('Profile, region, and bucket are required');
  }

  await apiDeleteObject(profile, region, bucket, key);

  // Refresh the current view
  await refreshS3Browser();
}

/**
 * Get presigned URL for an object
 */
export async function getS3PresignedUrl(key: string, expiresInSecs: number = 3600): Promise<string> {
  const profile = get(selectedProfile);
  const region = get(selectedRegion);
  const bucket = get(selectedBucket);

  if (!profile || !region || !bucket) {
    throw new Error('Profile, region, and bucket are required');
  }

  return apiGetPresignedUrl(profile, region, bucket, key, expiresInSecs);
}

/**
 * Get object content for preview
 */
export async function getS3ObjectContent(key: string, maxBytes?: number): Promise<string> {
  const profile = get(selectedProfile);
  const region = get(selectedRegion);
  const bucket = get(selectedBucket);

  if (!profile || !region || !bucket) {
    throw new Error('Profile, region, and bucket are required');
  }

  return apiGetObjectContent(profile, region, bucket, key, maxBytes);
}

/**
 * Close the S3 browser
 */
export function closeS3Browser(): void {
  s3BrowserActive.set(false);
  selectedBucket.set(null);
  s3BrowserState.set({
    bucket: '',
    prefix: '',
    objects: [],
    folders: [],
    loading: false,
    error: null,
  });
  s3NavigationHistory.set([]);
  s3NavigationIndex.set(-1);
}

/**
 * Clear S3 buckets (e.g., when changing profile/region)
 */
export function clearS3Buckets(): void {
  s3Buckets.set([]);
  s3BucketsError.set(null);
  closeS3Browser();
}
