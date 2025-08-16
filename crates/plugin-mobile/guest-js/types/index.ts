// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface ShareTextRequest {
  readonly title: string;
  readonly text: string;
  readonly mimeType: ShareTextMimeType;
}

export type ShareTextMimeType = 'text/plain' | 'text/html' | 'text/json';
