// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

const sortCollator = new Intl.Collator(undefined, {
  numeric: true,
  sensitivity: 'variant',
  usage: 'sort',
});

export function compare(a: string, b: string): number {
  return sortCollator.compare(a, b);
}
