// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Bot {
  readonly id: BotId;
  readonly name: string;
  readonly ethics: Ethics;
  readonly resources: Resources;
}

type BotId = number;
