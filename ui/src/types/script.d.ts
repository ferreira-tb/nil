// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Script = {
  readonly id: ScriptId;
  readonly owner: PlayerId;
  name: string;
  code: string;
};

type ScriptId = number;
