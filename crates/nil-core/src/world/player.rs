// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::error::{Error, Result};
use crate::player::{Player, PlayerId, PlayerStatus};
use crate::village::Village;

impl World {
  #[inline]
  pub fn has_player(&self, id: &PlayerId) -> bool {
    self.player_manager.has(id)
  }

  pub fn set_player_status(&mut self, id: &PlayerId, status: PlayerStatus) -> Result<()> {
    *self
      .player_manager
      .player_mut(id)?
      .status_mut() = status;

    self.emit_player_updated(id.clone());

    Ok(())
  }

  pub fn spawn_player(&mut self, mut player: Player) -> Result<()> {
    let id = player.id();
    if self.has_player(&id) {
      Err(Error::PlayerAlreadySpawned(id))
    } else {
      let (coord, field) = self.continent.find_spawn_point()?;
      *field = Village::builder(coord)
        .name(&*id)
        .owner(&id)
        .build()
        .into();

      *player.status_mut() = PlayerStatus::Active;
      self.player_manager.insert(player);

      self.emit_public_village_updated(coord);

      Ok(())
    }
  }
}
