// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type EventPayload =
  | ChatMessagePayload
  | FailedToSaveWorldPayload
  | GuestLeftPayload
  | LobbyUpdatedPayload
  | PlayerSpawnedPayload
  | PlayerStatusUpdatedPayload
  | RoundUpdatedPayload
  | VillageSpawnedPayload;

type ChatMessagePayload = {
  readonly kind: 'chat-message';
  readonly message: ChatMessage;
};

type FailedToSaveWorldPayload = {
  readonly kind: 'failed-to-save-world';
  readonly error: string;
};

type GuestLeftPayload = {
  readonly kind: 'guest-left';
  readonly guest: Player;
};

type LobbyUpdatedPayload = {
  readonly kind: 'lobby-updated';
  readonly lobby: Lobby;
};

type PlayerSpawnedPayload = {
  readonly kind: 'player-spawned';
  readonly player: Player;
};

type PlayerStatusUpdatedPayload = {
  readonly kind: 'player-status-updated';
  readonly player: PlayerId;
  readonly status: PlayerStatus;
};

type RoundUpdatedPayload = {
  readonly kind: 'round-updated';
  readonly round: Round;
};

type VillageSpawnedPayload = {
  readonly kind: 'village-spawned';
  readonly village: VillagePublicState;
};
