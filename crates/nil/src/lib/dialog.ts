import { message } from '@tauri-apps/plugin-dialog';

export function error(text: string) {
  message(text, { title: 'Error', kind: 'error' }).catch(console.error);
}

export function info(text: string) {
  message(text, { title: 'Info', kind: 'info' }).catch(console.error);
}

export function warn(text: string) {
  message(text, { title: 'Warning', kind: 'warning' }).catch(console.error);
}
