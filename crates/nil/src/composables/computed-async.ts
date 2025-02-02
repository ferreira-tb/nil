import type { Ref } from 'vue';
import { handleError } from '@/lib/error';
import {
  type AsyncComputedOnCancel,
  type AsyncComputedOptions,
  computedAsync as original,
} from '@vueuse/core';

export function computedAsync<T>(
  initial: T,
  callback: (onCancel: AsyncComputedOnCancel) => Promise<T> | T,
  options?: AsyncComputedOptions
) {
  return original(callback, initial, { onError: handleError, ...options }) as Readonly<Ref<T>>;
}
