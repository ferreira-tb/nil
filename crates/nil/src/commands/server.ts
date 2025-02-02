import { invoke } from '@tauri-apps/api/core';
import type { WorldConfig } from '@/types/world';

export function getServerVersion() {
  return invoke<string>('get_server_version');
}

export function isServerOk() {
  return invoke<boolean>('is_server_ok');
}

export function startServer(worldConfig: WorldConfig) {
  return invoke<null>('start_server', { worldConfig });
}

export function stopServer() {
  return invoke<null>('stop_server');
}
