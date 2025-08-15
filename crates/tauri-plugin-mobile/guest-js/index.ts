// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { ShareTextRequest } from './types';

export type * from './types';

export function shareText(request: ShareTextRequest) {
  return invoke<null>('plugin:mobile|share_text', { request });
}
