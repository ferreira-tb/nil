export type Route = 'home' | 'host-game' | 'join-game' | 'lobby' | 'settings' | GameRoute;

export type GameRoute =
  | 'academy'
  | 'farm'
  | 'iron-mine'
  | 'prefecture'
  | 'quarry'
  | 'sawmill'
  | 'silo'
  | 'stable'
  | 'village'
  | 'wall'
  | 'warehouse';
