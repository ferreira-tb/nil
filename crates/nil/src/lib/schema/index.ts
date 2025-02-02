import * as base from './base';
import * as config from './config';

export function isIp(value: unknown) {
  return base.ip.safeParse(value).success;
}

export function isPlayerConfig(value: unknown) {
  return config.playerConfig.safeParse(value).success;
}

export function isWorldConfig(value: unknown) {
  return config.worldConfig.safeParse(value).success;
}
