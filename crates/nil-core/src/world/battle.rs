// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::battle::{Battle, BattleResult};
use crate::error::Result;
use crate::infrastructure::prelude::BuildingLevel;
use crate::military::squad::Squad;
use crate::world::World;

impl World {
  pub fn simulate_battle(
    &self,
    attacker: &[Squad],
    defender: &[Squad],
    wall: BuildingLevel,
  ) -> Result<BattleResult> {
    let wall_stats = if wall > BuildingLevel::ZERO {
      self
        .stats
        .infrastructure
        .wall()
        .get(wall)
        .map(Some)?
    } else {
      None
    };

    let result = Battle::builder()
      .attacker(attacker)
      .defender(defender)
      .maybe_wall(wall_stats)
      .build()
      .battle_result();

    Ok(result)
  }
}
