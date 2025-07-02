// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{Player, PlayerId};
use crate::error::{Error, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlayerManager(IndexMap<PlayerId, Player>);

impl PlayerManager {
  pub fn player(&self, id: &PlayerId) -> Result<&Player> {
    self
      .0
      .get(id)
      .ok_or_else(|| Error::PlayerNotFound(id.clone()))
  }

  pub fn player_mut(&mut self, id: &PlayerId) -> Result<&mut Player> {
    self
      .0
      .get_mut(id)
      .ok_or_else(|| Error::PlayerNotFound(id.clone()))
  }

  pub fn players(&self) -> impl Iterator<Item = &Player> {
    self.0.values()
  }

  pub fn players_mut(&mut self) -> impl Iterator<Item = &mut Player> {
    self.0.values_mut()
  }

  #[inline]
  pub fn has(&self, id: &PlayerId) -> bool {
    self.0.contains_key(id)
  }

  pub(crate) fn insert(&mut self, player: Player) {
    self.0.insert(player.id(), player);
  }

  pub(crate) fn remove_guest(&mut self, id: &PlayerId) -> Option<Player> {
    let index = self
      .0
      .iter()
      .position(|(_, player)| player.is_guest() && player.id == *id)?;

    self
      .0
      .shift_remove_index(index)
      .map(|(_, player)| player)
  }

  pub(crate) fn remove_guests(&mut self) {
    self.0.retain(|_, player| !player.is_guest());
  }
}
