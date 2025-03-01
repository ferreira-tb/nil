import type { ShallowRef } from 'vue';
import { handleError } from '@/lib/error';
import { useAsyncState } from '@vueuse/core';

export function asyncRef<T>(initial: T, fn: () => Promise<T>) {
  const { execute, state } = useAsyncState<T>(fn, initial, {
    immediate: true,
    onError: handleError,
    resetOnExecute: false,
    shallow: true,
    throwError: false,
  });

  return Object.assign(state as Readonly<ShallowRef<T>>, { execute });
}
