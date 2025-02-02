import { invoke } from '@tauri-apps/api/core';

export * from './player';
export * from './client';
export * from './server';

export function isDev() {
  return invoke<boolean>('is_dev');
}

export function showWindow() {
  return invoke<null>('show_window');
}
