// src/lib/stores/notifications.ts
import { writable } from "svelte/store";

export type NotificationType = "info" | "success" | "error" | "warning";

export interface Notification {
  id: number;
  message: string;
  type: NotificationType;
  timeout?: number;
}

const notifications = writable<Notification[]>([]);

let id = 0;

function send(
  message: string,
  type: NotificationType = "info",
  timeout = 3000,
) {
  const newNotification: Notification = {
    id: id++,
    message,
    type,
    timeout,
  };

  notifications.update((n) => [...n, newNotification]);

  // Auto-remove after timeout
  if (timeout) {
    setTimeout(() => {
      notifications.update((n) =>
        n.filter((notif) => notif.id !== newNotification.id),
      );
    }, timeout);
  }
}

export { notifications, send };
