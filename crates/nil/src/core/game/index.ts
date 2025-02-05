import { go } from '@/router';
import { until } from '@vueuse/core';
import * as commands from '@/commands';
import { HandleError } from '@/lib/error';
import { PlayerImpl } from '@/core/player';
import type { Option } from '@tb-dev/utils';
import { VillageImpl } from '@/core/village';
import type { Coord } from '@/types/village';
import { SocketAddrV4 } from '@/lib/net/addr';
import type { WorldConfig } from '@/types/world';
import { exit } from '@tauri-apps/plugin-process';
import type { PlayerConfig } from '@/types/player';
import { computedAsync } from '@/composables/computed-async';
import { fallibleInject, provide, runWithContext } from '@/lib/app';
import { computed, type InjectionKey, shallowRef, watch } from 'vue';

const gameSymbol: InjectionKey<Game> = Symbol('game');

export class Game {
  private readonly player = shallowRef<Option<PlayerImpl>>();
  private readonly playerId = computed(() => this.player.value?.id ?? null);

  private readonly coord = shallowRef<Option<Coord>>();
  private readonly village = computedAsync(null, () => {
    const coord = this.coord.value;
    return coord ? VillageImpl.load(coord) : null;
  });

  private constructor() {
    runWithContext(() => {
      watch(this.player, this.onPlayerUpdate.bind(this));
    });
  }

  @HandleError({ async: true })
  public async join(server: SocketAddrV4, player: PlayerConfig) {
    await commands.startClient(server);
    const playerId = await commands.spawnPlayer(player);
    this.player.value = await PlayerImpl.load(playerId);

    await until(this.coord).toBeTruthy();
    go('village');
  }

  @HandleError({ async: true })
  public async host(world: WorldConfig, player: PlayerConfig) {
    const info = await commands.startServer(world);
    const addr = SocketAddrV4.parse(`127.0.0.1:${info.port}`);
    await this.join(addr, player);
  }

  private async updatePlayer() {
    if (this.playerId.value) {
      this.player.value = await PlayerImpl.load(this.playerId.value);
    }
  }

  private onPlayerUpdate(player: Option<PlayerImpl>) {
    if (player && (!this.coord.value || !player.has(this.coord.value))) {
      this.coord.value = player.villages.at(0);
    } else if (!player) {
      this.coord.value = null;
    }
  }

  @HandleError({ async: true })
  public static async exit() {
    await commands.stopClient();
    await commands.stopServer();
    await exit(0);
  }

  public static use() {
    let game = fallibleInject(gameSymbol);
    if (!game) {
      game = new Game();
      provide(gameSymbol, game);
    }

    return {
      player: game.player as Readonly<typeof game.player>,
      playerId: game.playerId,
      village: game.village,
      join: game.join.bind(game),
      host: game.host.bind(game),
      updatePlayer: game.updatePlayer.bind(game),
    };
  }
}
