import * as options from './options';

export function isPlayerOptions(value: unknown) {
  return options.player.safeParse(value).success;
}

export function isWorldOptions(value: unknown) {
  return options.world.safeParse(value).success;
}
