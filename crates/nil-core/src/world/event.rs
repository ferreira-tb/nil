// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::chat::ChatMessage;
use crate::event::{Event, Listener};
use crate::player::{Player, PlayerId, PlayerStatus};
use crate::village::{Coord, PublicVillage};

impl World {
  #[inline]
  pub fn subscribe(&self) -> Listener {
    self.emitter.subscribe()
  }

  /// Emite o evento para todos os jogadores.
  fn broadcast(&self, event: Event) {
    self.emitter.broadcast(event);
  }

  /// Emite o evento para um jogador em específico.
  fn emit_to(&self, target: PlayerId, event: Event) {
    self.emitter.emit_to(target, event);
  }

  /// Emite o evento para o dono da aldeia na coordenada especificada, se houver.
  fn emit_to_owner(&self, coord: Coord, event: Event) {
    if let Ok(village) = self.village(coord)
      && let Some(player) = village.player()
    {
      self.emitter.emit_to(player, event);
    }
  }

  /// Emite [`Event::ChatMessage`].
  pub(super) fn emit_chat_message(&self, message: ChatMessage) {
    self.broadcast(Event::ChatMessage { message });
  }

  /// Emite [`Event::GuestLeft`].
  pub(super) fn emit_guest_left(&self, guest: Player) {
    self.broadcast(Event::GuestLeft { guest });
  }

  /// Emite [`Event::PlayerSpawned`].
  pub(super) fn emit_player_spawned(&self, player: Player) {
    self.broadcast(Event::PlayerSpawned { player });
  }

  /// Emite [`Event::PlayerStatusUpdated`].
  pub(super) fn emit_player_status_updated(&self, id: PlayerId, status: PlayerStatus) {
    self.broadcast(Event::PlayerStatusUpdated { player: id, status });
  }

  /// Emite [`Event::PlayerUpdated`].
  pub(super) fn emit_player_updated(&self, player: PlayerId) {
    self.emit_to(player.clone(), Event::PlayerUpdated { player });
  }

  /// Emite [`Event::PublicVillageUpdated`].
  pub(super) fn emit_public_village_updated(&self, coord: Coord) {
    self.broadcast(Event::PublicVillageUpdated { coord });
  }

  /// Emite [`Event::RoundUpdated`].
  pub(super) fn emit_round_updated(&self) {
    let round = self.round.clone();
    self.broadcast(Event::RoundUpdated { round });
  }

  /// Emite [`Event::VillageSpawned`].
  pub(super) fn emit_village_spawned(&self, village: PublicVillage) {
    self.broadcast(Event::VillageSpawned { village });
  }

  /// Emite [`Event::VillageUpdated`].
  pub(super) fn emit_village_updated(&self, coord: Coord) {
    self.emit_to_owner(coord, Event::VillageUpdated { coord });
  }
}
