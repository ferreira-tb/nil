export interface Player {
  readonly id: PlayerId;
  readonly name: string;
}

export type PlayerId = string;

export interface PlayerConfig {
  readonly name: string;
}
