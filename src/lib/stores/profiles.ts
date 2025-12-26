import { writable, derived, get } from 'svelte/store';
import type { AwsProfile, AwsRegion } from '$lib/types/aws';
import { listAwsProfiles, listAwsRegions, getProfileRegion } from '$lib/api/aws';

// Store for AWS profiles
export const profiles = writable<AwsProfile[]>([]);
export const profilesLoading = writable<boolean>(false);
export const profilesError = writable<string | null>(null);

// Store for AWS regions
export const regions = writable<AwsRegion[]>([]);

// Selected profile and region
export const selectedProfile = writable<string>('default');
export const selectedRegion = writable<string>('us-east-1');

// Derived store for the current profile object
export const currentProfile = derived(
  [profiles, selectedProfile],
  ([$profiles, $selectedProfile]) => {
    return $profiles.find((p) => p.name === $selectedProfile) || null;
  }
);

/**
 * Load AWS profiles from the backend
 */
export async function loadProfiles(): Promise<void> {
  profilesLoading.set(true);
  profilesError.set(null);

  try {
    const loadedProfiles = await listAwsProfiles();
    profiles.set(loadedProfiles);

    // If we have profiles but no default is selected, select the first one
    if (loadedProfiles.length > 0) {
      const current = get(selectedProfile);
      const hasSelected = loadedProfiles.some((p) => p.name === current);

      if (!hasSelected) {
        // Select default profile if it exists, otherwise first profile
        const defaultProfile = loadedProfiles.find((p) => p.name === 'default');
        selectedProfile.set(defaultProfile?.name || loadedProfiles[0].name);
      }

      // Set region from profile if available
      const profile = loadedProfiles.find((p) => p.name === get(selectedProfile));
      if (profile?.region) {
        selectedRegion.set(profile.region);
      }
    }
  } catch (error) {
    profilesError.set(String(error));
    console.error('Failed to load AWS profiles:', error);
  } finally {
    profilesLoading.set(false);
  }
}

/**
 * Load AWS regions
 */
export async function loadRegions(): Promise<void> {
  try {
    const loadedRegions = await listAwsRegions();
    regions.set(loadedRegions);
  } catch (error) {
    console.error('Failed to load AWS regions:', error);
  }
}

/**
 * Select a profile and update the region if the profile has a default region
 */
export async function selectProfile(profileName: string): Promise<void> {
  selectedProfile.set(profileName);

  // Try to get the profile's default region
  const profileRegion = await getProfileRegion(profileName);
  if (profileRegion) {
    selectedRegion.set(profileRegion);
  }
}

/**
 * Initialize profiles and regions
 */
export async function initializeAws(): Promise<void> {
  await Promise.all([loadProfiles(), loadRegions()]);
}
