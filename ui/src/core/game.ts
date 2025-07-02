// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import * as commands from '@/commands';
import * as dialog from '@/lib/dialog';
import { Entity } from '@/core/entity/abstract';
import { exit } from '@tauri-apps/plugin-process';
import type { SocketAddrV4 } from '@/lib/net/addr-v4';

export async function joinGame(options: { player: PlayerOptions; serverAddr: SocketAddrV4 }) {
  await commands.startClient(options.player.id, options.serverAddr);

  // O jogador já existirá no mundo se isso for um jogo salvo.
  if (!(await commands.playerExists(options.player.id))) {
    await commands.spawnPlayer(options.player);
  }

  NIL.player.setId(options.player.id);
  await NIL.update();

  if (await commands.isRoundIdle()) {
    await go('lobby');
  } else {
    const player = NIL.player.refs().player;
    if (player.value?.isGuest()) {
      await commands.spawnPlayerVillage(player.value.id);
    } else if (player.value?.isInactive()) {
      await commands.setPlayerStatus(player.value.id, 'active');
    }

    await NIL.village.go({ timeout: 500, keepTrying: true });
  }
}

export async function hostGame(options: { player: PlayerOptions; world: WorldOptions }) {
  const addr = await commands.startServerWithOptions(options.world);
  await joinGame({ player: options.player, serverAddr: addr.asLocal() });
}

export async function hostSavedGame(options: { path: string; player: PlayerOptions }) {
  const addr = await commands.startServerWithSavedata(options.path);
  await joinGame({ player: options.player, serverAddr: addr.asLocal() });
}

export async function leaveGame() {
  await commands.stopClient();
  await commands.stopServer();

  Entity.dispose();
  await go('home');
}

export async function exitGame() {
  await leaveGame();
  await exit(0);
}

export async function saveGame() {
  const path = await dialog.save({
    filters: [{ name: 'Nil', extensions: ['nil'] }],
  });

  if (path) {
    await commands.saveWorld(path);
  }
}
