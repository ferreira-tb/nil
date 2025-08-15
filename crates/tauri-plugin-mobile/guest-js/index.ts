// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from '@tb-dev/utils';
import { invoke } from '@tauri-apps/api/core';
import type { AndroidVersion } from './types';

export type * from './types';

export function getAndroidVersion() {
  return invoke<Option<AndroidVersion>>('plugin:mobile|get_android_version');
}
