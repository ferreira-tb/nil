type Player = {
  readonly id: PlayerId;
  readonly status: PlayerStatus;
};

type PlayerId = string;

type PlayerStatus = 'active' | 'guest' | 'inactive';

type PlayerOptions = {
  readonly id: PlayerId;
};
