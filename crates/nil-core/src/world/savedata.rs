// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::chat::Chat;
use crate::continent::Continent;
use crate::error::{Error, Result};
use crate::player::{PlayerManager, PlayerStatus};
use crate::round::{Phase, Round};
use crate::script::Scripting;
use crate::world::{World, WorldConfig, WorldStats};
use jiff::Zoned;
use nil_util::serde::{read_file, write_file};
use serde::{Deserialize, Serialize};
use std::path::Path;

impl World {
  pub(super) fn consume_pending_save(&mut self) -> Result<()> {
    if let Some(mut path) = self.pending_save.take() {
      path.set_extension("nil");
      save(self, &path)?;
    }

    Ok(())
  }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Savedata {
  pub continent: Continent,
  pub player_manager: PlayerManager,
  pub round: Round,
  pub config: WorldConfig,
  pub stats: WorldStats,
  pub chat: Chat,
  pub scripting: Scripting,
  pub time: Zoned,
}

impl Savedata {
  pub(super) fn load(path: &Path) -> Result<Self> {
    read_file(path).map_err(|_| Error::FailedToLoadWorld)
  }
}

fn save(world: &World, path: &Path) -> Result<()> {
  let mut savedata = Savedata {
    continent: world.continent.clone(),
    player_manager: world.player_manager.clone(),
    round: world.round.clone(),
    config: world.config.clone(),
    stats: world.stats.clone(),
    chat: world.chat.clone(),
    scripting: world.scripting.clone(),
    time: Zoned::now(),
  };

  savedata.player_manager.remove_guests();
  for player in savedata.player_manager.players_mut() {
    *player.status_mut() = PlayerStatus::Inactive;
  }

  *savedata.round.phase_mut() = Phase::Idle;

  write_file(path, &savedata).map_err(|_| Error::FailedToSaveWorld)
}
