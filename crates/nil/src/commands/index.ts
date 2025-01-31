import { invoke } from '@tauri-apps/api/core';

export function showWindow() {
  return invoke<null>('show_window');
}
