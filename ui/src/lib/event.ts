// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { handleError } from '@/lib/error';
import type { Fn, MaybePromise, Option } from '@tb-dev/utils';
import { getCurrentWebviewWindow, type WebviewWindow } from '@tauri-apps/api/webviewWindow';

export type ListenerFn<T> = (payload: T) => MaybePromise<unknown>;

class Listener<T extends EventPayload> {
  public readonly on: (fn: ListenerFn<T>) => Promise<Fn>;

  private constructor(id: T['kind']) {
    this.on = (fn: ListenerFn<T>) => {
      Listener.webview ??= getCurrentWebviewWindow();
      return Listener.webview.listen<T>(`nil://${id}`, ({ payload }) => {
        (async () => {
          try {
            await fn(payload);
          } catch (err) {
            handleError(err);
          }
        })();
      });
    };
  }

  private static webview: Option<WebviewWindow>;
  public static readonly listeners = {
    onChatMessage: new this<ChatMessagePayload>('chat-message'),
    onGuestLeft: new this<GuestLeftPayload>('guest-left'),
    onLobbyUpdated: new this<LobbyUpdatedPayload>('lobby-updated'),
    onPlayerSpawned: new this<PlayerSpawnedPayload>('player-spawned'),
    onPlayerStatusUpdated: new this<PlayerStatusUpdatedPayload>('player-status-updated'),
    onRoundUpdated: new this<RoundUpdatedPayload>('round-updated'),
    onVillageSpawned: new this<VillageSpawnedPayload>('village-spawned'),
  } as const;
}

type EventObject = typeof Listener.listeners;

export type EventProxy = {
  [K in keyof EventObject]: EventObject[K]['on'];
};

export const events = new Proxy(Listener.listeners as unknown as EventProxy, {
  get: (target: EventProxy, key: keyof EventProxy) => {
    return Reflect.get(Reflect.get(target, key), 'on');
  },

  defineProperty: () => false,
  deleteProperty: () => false,
  set: () => false,
});
