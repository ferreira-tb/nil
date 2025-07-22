// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::error::{Error, Result};
use crate::infrastructure::storage::OverallStorageCapacity;
use crate::player::{Player, PlayerId, PlayerStatus};
use crate::resources::Maintenance;
use crate::village::Village;

impl World {
  pub fn get_player_maintenance(&self, player: &PlayerId) -> Result<Maintenance> {
    self
      .continent
      .player_villages_by(|id| id == player)
      .try_fold(Maintenance::default(), |acc, village| {
        Ok(acc + village.maintenance(&self.stats.infrastructure)?)
      })
  }

  pub fn get_player_storage_capacity(&self, player: &PlayerId) -> Result<OverallStorageCapacity> {
    let villages = self
      .continent
      .player_villages_by(|id| id == player);

    self.get_storage_capacity(villages)
  }

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
      let (coord, field) = self.find_spawn_point()?;
      *field = Village::builder(coord)
        .name(&*id)
        .owner(&id)
        .build()
        .into();

      *player.status_mut() = PlayerStatus::Active;
      self.player_manager.spawn(player);

      self.emit_public_village_updated(coord);

      Ok(())
    }
  }
}
