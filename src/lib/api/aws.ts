import { invoke } from '@tauri-apps/api/core';
import type { AwsProfile, AwsRegion, PrerequisiteStatus, AddProfileInput, AddSsoProfileInput } from '$lib/types/aws';

/**
 * List all AWS profiles from credentials and config files
 */
export async function listAwsProfiles(): Promise<AwsProfile[]> {
  return invoke<AwsProfile[]>('list_aws_profiles');
}

/**
 * Get list of AWS regions
 */
export async function listAwsRegions(): Promise<AwsRegion[]> {
  return invoke<AwsRegion[]>('list_aws_regions');
}

/**
 * Check if AWS CLI is installed
 */
export async function checkAwsCli(): Promise<string> {
  return invoke<string>('check_aws_cli');
}

/**
 * Check if session-manager-plugin is installed
 */
export async function checkSsmPlugin(): Promise<string> {
  return invoke<string>('check_ssm_plugin');
}

/**
 * Get the default region for a profile
 */
export async function getProfileRegion(profileName: string): Promise<string | null> {
  return invoke<string | null>('get_profile_region', { profileName });
}

/**
 * Check all prerequisites (AWS CLI and SSM plugin)
 */
export async function checkPrerequisites(): Promise<PrerequisiteStatus> {
  const status: PrerequisiteStatus = {
    awsCli: { installed: false },
    ssmPlugin: { installed: false },
  };

  try {
    const version = await checkAwsCli();
    status.awsCli = { installed: true, version };
  } catch (error) {
    status.awsCli = { installed: false, error: String(error) };
  }

  try {
    const version = await checkSsmPlugin();
    status.ssmPlugin = { installed: true, version };
  } catch (error) {
    status.ssmPlugin = { installed: false, error: String(error) };
  }

  return status;
}

/**
 * Check if a profile name already exists
 */
export async function checkProfileExists(name: string): Promise<boolean> {
  return invoke<boolean>('check_profile_exists', { name });
}

/**
 * Add a new AWS profile with access key credentials
 */
export async function addAwsProfile(input: AddProfileInput): Promise<void> {
  return invoke('add_aws_profile', {
    name: input.name,
    accessKeyId: input.accessKeyId,
    secretAccessKey: input.secretAccessKey,
    region: input.region,
    sessionToken: input.sessionToken,
  });
}

/**
 * Add a new AWS SSO profile
 */
export async function addAwsSsoProfile(input: AddSsoProfileInput): Promise<void> {
  return invoke('add_aws_sso_profile', {
    name: input.name,
    ssoStartUrl: input.ssoStartUrl,
    ssoRegion: input.ssoRegion,
    ssoAccountId: input.ssoAccountId,
    ssoRoleName: input.ssoRoleName,
    region: input.region,
  });
}

/**
 * Delete an AWS profile
 */
export async function deleteAwsProfile(name: string): Promise<void> {
  return invoke('delete_aws_profile', { name });
}

/**
 * Run SSO login for a profile
 */
export async function ssoLogin(profileName: string): Promise<string> {
  return invoke<string>('sso_login', { profileName });
}

/**
 * Validate credentials by calling STS get-caller-identity
 */
export async function validateCredentials(profileName: string): Promise<string> {
  return invoke<string>('validate_credentials', { profileName });
}
