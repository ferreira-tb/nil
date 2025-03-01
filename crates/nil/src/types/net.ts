import type { WorldOptions } from './world';
import type { PlayerOptions } from './player';
import type { SocketAddrV4 } from '@/lib/net/addr-v4';

export type JoinOptions = {
  readonly player: PlayerOptions;
  readonly serverAddr: SocketAddrV4;
};

export type HostOptions = {
  readonly player: PlayerOptions;
  readonly world: WorldOptions;
};
