// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type EventPayload =
  | ChatUpdatedPayload
  | PlayerUpdatedPayload
  | PublicVillageUpdatedPayload
  | RoundUpdatedPayload
  | VillageUpdatedPayload;

type EventPayloadKind = EventPayload['kind'];

interface ChatUpdatedPayload {
  readonly kind: 'chat-updated';
  readonly message: ChatMessage;
}

interface PlayerUpdatedPayload {
  readonly kind: 'player-updated';
  readonly player: PlayerId;
}

interface PublicVillageUpdatedPayload {
  readonly kind: 'public-village-updated';
  readonly coord: Coord;
}

interface RoundUpdatedPayload {
  readonly kind: 'round-updated';
  readonly round: Round;
}

interface VillageUpdatedPayload {
  readonly kind: 'village-updated';
  readonly coord: Coord;
}
