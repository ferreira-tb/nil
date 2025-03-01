import type { ShallowRef } from 'vue';
import { handleError } from '@/lib/error';
import { computedAsync } from '@vueuse/core';

export function asyncComputed<T>(initial: T, callback: () => Promise<T> | T) {
  const state = computedAsync(callback, initial, {
    onError: handleError,
    shallow: true,
  });

  return state as Readonly<ShallowRef<T>>;
}
