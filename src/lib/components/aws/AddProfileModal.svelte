<script lang="ts">
  import { listAwsRegions } from '$lib/api/aws';
  import type { AwsRegion } from '$lib/types/aws';
  import {
    checkProfileExists,
    addAwsProfile,
    addAwsSsoProfile,
    validateCredentials,
  } from '$lib/api/aws';
  import { loadProfiles } from '$lib/stores/profiles';

  interface Props {
    open: boolean;
    onClose: () => void;
  }

  let { open, onClose }: Props = $props();

  type ProfileType = 'accessKey' | 'sso';

  let profileType = $state<ProfileType>('accessKey');
  let isSubmitting = $state(false);
  let error = $state<string | null>(null);
  let validationResult = $state<string | null>(null);
  let regions = $state<AwsRegion[]>([]);

  // Access Key form fields
  let name = $state('');
  let accessKeyId = $state('');
  let secretAccessKey = $state('');
  let region = $state('us-east-1');
  let sessionToken = $state('');

  // SSO form fields
  let ssoStartUrl = $state('');
  let ssoRegion = $state('us-east-1');
  let ssoAccountId = $state('');
  let ssoRoleName = $state('');

  $effect(() => {
    if (open) {
      loadRegions();
      resetForm();
    }
  });

  async function loadRegions() {
    try {
      regions = await listAwsRegions();
    } catch (e) {
      console.error('Failed to load regions:', e);
    }
  }

  function resetForm() {
    profileType = 'accessKey';
    name = '';
    accessKeyId = '';
    secretAccessKey = '';
    region = 'us-east-1';
    sessionToken = '';
    ssoStartUrl = '';
    ssoRegion = 'us-east-1';
    ssoAccountId = '';
    ssoRoleName = '';
    error = null;
    validationResult = null;
  }

  async function handleSubmit() {
    error = null;
    validationResult = null;
    isSubmitting = true;

    try {
      // Check if profile already exists
      const exists = await checkProfileExists(name);
      if (exists) {
        error = `Profile "${name}" already exists. Please choose a different name.`;
        return;
      }

      if (profileType === 'accessKey') {
        await addAwsProfile({
          name,
          accessKeyId,
          secretAccessKey,
          region,
          sessionToken: sessionToken || undefined,
        });
      } else {
        await addAwsSsoProfile({
          name,
          ssoStartUrl,
          ssoRegion,
          ssoAccountId,
          ssoRoleName,
          region,
        });
      }

      // Validate credentials
      try {
        const result = await validateCredentials(name);
        validationResult = 'Credentials validated successfully!';
      } catch (e) {
        // For SSO profiles, validation might fail until login is done
        if (profileType === 'sso') {
          validationResult = 'Profile added. Run SSO login to authenticate.';
        } else {
          error = `Profile added but validation failed: ${e}`;
          return;
        }
      }

      // Reload profiles
      await loadProfiles();

      // Close modal after short delay to show success
      setTimeout(() => {
        onClose();
      }, 1500);
    } catch (e) {
      error = `Failed to add profile: ${e}`;
    } finally {
      isSubmitting = false;
    }
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      onClose();
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      onClose();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if open}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="modal-backdrop"
    onclick={handleBackdropClick}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div class="modal" role="document">
      <div class="modal-header">
        <h2>Add AWS Profile</h2>
        <button class="close-btn" onclick={onClose} aria-label="Close">
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <div class="profile-type-tabs">
          <button
            class="tab"
            class:active={profileType === 'accessKey'}
            onclick={() => profileType = 'accessKey'}
          >
            Access Keys
          </button>
          <button
            class="tab"
            class:active={profileType === 'sso'}
            onclick={() => profileType = 'sso'}
          >
            SSO / IAM Identity Center
          </button>
        </div>

        <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
          <div class="form-group">
            <label for="profile-name">Profile Name</label>
            <input
              type="text"
              id="profile-name"
              bind:value={name}
              placeholder="my-aws-profile"
              required
            />
          </div>

          {#if profileType === 'accessKey'}
            <div class="form-group">
              <label for="access-key-id">Access Key ID</label>
              <input
                type="text"
                id="access-key-id"
                bind:value={accessKeyId}
                placeholder="AKIA..."
                required
              />
            </div>

            <div class="form-group">
              <label for="secret-access-key">Secret Access Key</label>
              <input
                type="password"
                id="secret-access-key"
                bind:value={secretAccessKey}
                placeholder="Your secret access key"
                required
              />
            </div>

            <div class="form-group">
              <label for="session-token">Session Token (optional)</label>
              <input
                type="password"
                id="session-token"
                bind:value={sessionToken}
                placeholder="For temporary credentials"
              />
            </div>

            <div class="form-group">
              <label for="region">Default Region</label>
              <select id="region" bind:value={region}>
                {#each regions as r (r.code)}
                  <option value={r.code}>{r.code} - {r.name}</option>
                {/each}
              </select>
            </div>
          {:else}
            <div class="form-group">
              <label for="sso-start-url">SSO Start URL</label>
              <input
                type="url"
                id="sso-start-url"
                bind:value={ssoStartUrl}
                placeholder="https://my-sso-portal.awsapps.com/start"
                required
              />
            </div>

            <div class="form-group">
              <label for="sso-region">SSO Region</label>
              <select id="sso-region" bind:value={ssoRegion}>
                {#each regions as r (r.code)}
                  <option value={r.code}>{r.code}</option>
                {/each}
              </select>
            </div>

            <div class="form-group">
              <label for="sso-account-id">AWS Account ID</label>
              <input
                type="text"
                id="sso-account-id"
                bind:value={ssoAccountId}
                placeholder="123456789012"
                required
              />
            </div>

            <div class="form-group">
              <label for="sso-role-name">SSO Role Name</label>
              <input
                type="text"
                id="sso-role-name"
                bind:value={ssoRoleName}
                placeholder="AdministratorAccess"
                required
              />
            </div>

            <div class="form-group">
              <label for="default-region">Default Region</label>
              <select id="default-region" bind:value={region}>
                {#each regions as r (r.code)}
                  <option value={r.code}>{r.code} - {r.name}</option>
                {/each}
              </select>
            </div>
          {/if}

          {#if error}
            <div class="message error">{error}</div>
          {/if}

          {#if validationResult}
            <div class="message success">{validationResult}</div>
          {/if}

          <div class="form-actions">
            <button type="button" class="btn-secondary" onclick={onClose}>
              Cancel
            </button>
            <button type="submit" class="btn-primary" disabled={isSubmitting}>
              {#if isSubmitting}
                Adding...
              {:else}
                Add Profile
              {/if}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background-color: var(--color-bg-secondary);
    border-radius: 12px;
    border: 1px solid var(--color-border);
    width: 100%;
    max-width: 480px;
    max-height: 90vh;
    overflow-y: auto;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3);
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--color-border);
  }

  .modal-header h2 {
    font-size: 18px;
    font-weight: 600;
    color: var(--color-text-primary);
    margin: 0;
  }

  .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: 6px;
    color: var(--color-text-muted);
    transition: background-color 150ms ease, color 150ms ease;
  }

  .close-btn:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .modal-body {
    padding: 20px;
  }

  .profile-type-tabs {
    display: flex;
    gap: 8px;
    margin-bottom: 20px;
  }

  .tab {
    flex: 1;
    padding: 10px 16px;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 500;
    color: var(--color-text-secondary);
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    transition: all 150ms ease;
  }

  .tab:hover {
    color: var(--color-text-primary);
    background-color: var(--color-bg-hover);
  }

  .tab.active {
    color: var(--color-accent);
    background-color: var(--color-accent-subtle);
    border-color: var(--color-accent);
  }

  .form-group {
    margin-bottom: 16px;
  }

  .form-group label {
    display: block;
    font-size: 13px;
    font-weight: 500;
    color: var(--color-text-secondary);
    margin-bottom: 6px;
  }

  .form-group input,
  .form-group select {
    width: 100%;
    padding: 10px 12px;
    font-size: 14px;
    color: var(--color-text-primary);
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    transition: border-color 150ms ease, box-shadow 150ms ease;
  }

  .form-group input::placeholder {
    color: var(--color-text-muted);
  }

  .form-group input:focus,
  .form-group select:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: 0 0 0 2px var(--color-accent-subtle);
  }

  .form-group select {
    appearance: none;
    -webkit-appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23a3a3a3' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 12px center;
    padding-right: 36px;
  }

  .message {
    padding: 12px;
    border-radius: 6px;
    font-size: 13px;
    margin-bottom: 16px;
  }

  .message.error {
    background-color: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #f87171;
  }

  .message.success {
    background-color: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.3);
    color: #4ade80;
  }

  .form-actions {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
    margin-top: 24px;
  }

  .btn-secondary,
  .btn-primary {
    padding: 10px 20px;
    font-size: 14px;
    font-weight: 500;
    border-radius: 6px;
    transition: all 150ms ease;
  }

  .btn-secondary {
    color: var(--color-text-secondary);
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
  }

  .btn-secondary:hover {
    color: var(--color-text-primary);
    background-color: var(--color-bg-hover);
  }

  .btn-primary {
    color: white;
    background-color: var(--color-accent);
    border: 1px solid var(--color-accent);
  }

  .btn-primary:hover:not(:disabled) {
    background-color: var(--color-accent-hover);
  }

  .btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
