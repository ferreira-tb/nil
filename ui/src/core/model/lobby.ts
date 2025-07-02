// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { getLobby } from '@/commands/lobby';

export class LobbyImpl implements Lobby {
  public readonly ready: readonly PlayerId[];

  private constructor(lobby: Lobby) {
    this.ready = lobby.ready;
  }

  public isReady(id: PlayerId) {
    return this.ready.includes(id);
  }

  public static create(lobby: Lobby) {
    return new LobbyImpl(lobby);
  }

  public static async load() {
    const lobby = await getLobby();
    return this.create(lobby);
  }
}
