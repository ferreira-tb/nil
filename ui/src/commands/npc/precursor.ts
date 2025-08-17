// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function getPublicPrecursor(id: PrecursorId) {
  return invoke<PublicPrecursor>('get_public_precursor', { id });
}
