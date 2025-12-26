import { writable, derived, get } from 'svelte/store';

// Settings interface
export interface AppSettings {
  // Appearance
  sidebarWidth: number;
  terminalFontSize: number;

  // Defaults
  defaultRegion: string;
  defaultShell: string;

  // Behavior
  confirmBeforeClose: boolean;
  autoRefreshInterval: number; // 0 = disabled, otherwise seconds
}

// Default settings
const defaultSettings: AppSettings = {
  sidebarWidth: 280,
  terminalFontSize: 14,
  defaultRegion: 'us-east-1',
  defaultShell: '/bin/sh',
  confirmBeforeClose: true,
  autoRefreshInterval: 0,
};

// Storage key
const STORAGE_KEY = 'aws-connector-settings';

// Load settings from localStorage
function loadSettings(): AppSettings {
  if (typeof window === 'undefined') return defaultSettings;

  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      const parsed = JSON.parse(stored);
      // Merge with defaults to handle new settings
      return { ...defaultSettings, ...parsed };
    }
  } catch (e) {
    console.error('Failed to load settings:', e);
  }

  return defaultSettings;
}

// Save settings to localStorage
function saveSettings(settings: AppSettings): void {
  if (typeof window === 'undefined') return;

  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  } catch (e) {
    console.error('Failed to save settings:', e);
  }
}

// Create the settings store
function createSettingsStore() {
  const { subscribe, set, update } = writable<AppSettings>(loadSettings());

  return {
    subscribe,

    // Update a single setting
    setSetting<K extends keyof AppSettings>(key: K, value: AppSettings[K]) {
      update(settings => {
        const newSettings = { ...settings, [key]: value };
        saveSettings(newSettings);
        return newSettings;
      });
    },

    // Update multiple settings
    setSettings(partial: Partial<AppSettings>) {
      update(settings => {
        const newSettings = { ...settings, ...partial };
        saveSettings(newSettings);
        return newSettings;
      });
    },

    // Reset to defaults
    reset() {
      set(defaultSettings);
      saveSettings(defaultSettings);
    },

    // Get current value synchronously
    get() {
      return get({ subscribe });
    },
  };
}

export const settings = createSettingsStore();

// Derived stores for convenience
export const sidebarWidth = derived(settings, $s => $s.sidebarWidth);
export const terminalFontSize = derived(settings, $s => $s.terminalFontSize);
export const defaultRegion = derived(settings, $s => $s.defaultRegion);
export const defaultShell = derived(settings, $s => $s.defaultShell);
