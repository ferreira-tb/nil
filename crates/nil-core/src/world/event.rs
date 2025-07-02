// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::chat::ChatMessage;
use crate::event::{Event, Listener};
use crate::player::{Player, PlayerId, PlayerStatus};
use crate::village::VillagePublicState;

#[cfg(debug_assertions)]
use tracing::info;

impl World {
  #[inline]
  pub fn subscribe(&self) -> Listener {
    self.emitter.subscribe()
  }

  #[inline]
  pub(super) fn broadcast(&self, event: Event) {
    #[cfg(debug_assertions)]
    info!(EVENT = ?event);

    self.emitter.broadcast(event);
  }

  #[inline]
  pub(super) fn emit_chat_message(&self, message: ChatMessage) {
    self.broadcast(Event::ChatMessage { message });
  }

  #[inline]
  pub(super) fn emit_guest_left(&self, guest: Player) {
    self.broadcast(Event::GuestLeft { guest });
  }

  #[inline]
  pub(super) fn emit_player_spawned(&self, player: Player) {
    self.broadcast(Event::PlayerSpawned { player });
  }

  #[inline]
  pub(super) fn emit_player_status_updated(&self, id: PlayerId, status: PlayerStatus) {
    self.broadcast(Event::PlayerStatusUpdated { player: id, status });
  }

  #[inline]
  pub(super) fn emit_round_update(&self) {
    let round = self.round.clone();
    self.broadcast(Event::RoundUpdated { round });
  }

  #[inline]
  pub(super) fn emit_village_spawned(&self, village: VillagePublicState) {
    self.broadcast(Event::VillageSpawned { village });
  }
}
