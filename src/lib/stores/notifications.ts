import { writable, derived } from 'svelte/store';

export type NotificationType = 'success' | 'error' | 'warning' | 'info';

export interface Notification {
  id: string;
  type: NotificationType;
  message: string;
  duration: number; // ms, 0 = persistent
  createdAt: number;
}

const notifications = writable<Notification[]>([]);

let idCounter = 0;

function generateId(): string {
  return `notification-${++idCounter}-${Date.now()}`;
}

export function notify(
  message: string,
  type: NotificationType = 'info',
  duration: number = 5000
): string {
  const id = generateId();
  const notification: Notification = {
    id,
    type,
    message,
    duration,
    createdAt: Date.now(),
  };

  notifications.update((n) => [...n, notification]);

  // Auto-dismiss after duration (if not persistent)
  if (duration > 0) {
    setTimeout(() => {
      dismiss(id);
    }, duration);
  }

  return id;
}

export function dismiss(id: string): void {
  notifications.update((n) => n.filter((notification) => notification.id !== id));
}

export function dismissAll(): void {
  notifications.set([]);
}

// Convenience functions
export function success(message: string, duration = 4000): string {
  return notify(message, 'success', duration);
}

export function error(message: string, duration = 6000): string {
  return notify(message, 'error', duration);
}

export function warning(message: string, duration = 5000): string {
  return notify(message, 'warning', duration);
}

export function info(message: string, duration = 4000): string {
  return notify(message, 'info', duration);
}

// Export the store for the UI
export const notificationList = derived(notifications, ($n) => $n);
