import type { Player } from '@/types/player';

export class PlayerImpl implements Player {
  public readonly id: string;
  public readonly name: string;

  constructor(player: Player) {
    this.id = player.id;
    this.name = player.name;
  }
}
