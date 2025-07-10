// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::chat::ChatMessage;
use crate::event::{Event, Listener};
use crate::player::PlayerId;
use crate::village::Coord;

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
  pub(super) fn emit_chat_updated(&self, message: ChatMessage) {
    self.broadcast(Event::ChatUpdated { message });
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

  /// Emite [`Event::VillageUpdated`].
  pub(super) fn emit_village_updated(&self, coord: Coord) {
    self.emit_to_owner(coord, Event::VillageUpdated { coord });
  }
}
