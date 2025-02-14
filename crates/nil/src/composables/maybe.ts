import type { Option } from '@tb-dev/utils';
import { type MaybeRefOrGetter, toValue } from 'vue';

export function maybe<T, U>(value: MaybeRefOrGetter<Option<T>>, fn: (value: T) => U): null | U {
  const _value = toValue(value);
  return _value ? fn(_value) : null;
}
