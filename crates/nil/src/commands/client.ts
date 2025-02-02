import { invoke } from '@tauri-apps/api/core';

export function startClient(serverIp: string) {
  return invoke<null>('start_client', { serverIp });
}

export function stopClient() {
  return invoke<null>('stop_client');
}
