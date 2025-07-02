// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod academy;
pub mod farm;
pub mod iron_mine;
pub mod prefecture;
pub mod prelude;
pub mod quarry;
pub mod sawmill;
pub mod silo;
pub mod stable;
pub mod wall;
pub mod warehouse;

use crate::error::Result;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{cmp, fmt};
use strum::{Display, EnumIter};

use crate::resource::{
  BaseCost,
  BaseCostGrowth,
  Food,
  Iron,
  Maintenance,
  MaintenanceRatio,
  ResourceRatio,
  Resources,
  Stone,
  Wood,
  Workforce,
  WorkforceGrowth,
};

pub trait Building {
  fn id(&self) -> BuildingId;
  fn level(&self) -> BuildingLevel;
  fn max_level(&self) -> BuildingLevel;
  fn is_enabled(&self) -> bool;

  fn base_cost(&self) -> BaseCost;
  fn base_cost_growth(&self) -> BaseCostGrowth;

  /// Taxa de manutenção do edifício em seu nível atual.
  fn maintenance(&self, stats: &BuildingStatsTable) -> Result<Maintenance>;
  fn maintenance_ratio(&self) -> MaintenanceRatio;

  fn wood_ratio(&self) -> ResourceRatio;
  fn stone_ratio(&self) -> ResourceRatio;
  fn iron_ratio(&self) -> ResourceRatio;

  fn workforce(&self) -> Workforce;
  fn workforce_growth(&self) -> WorkforceGrowth;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Display, EnumIter)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[remain::sorted]
pub enum BuildingId {
  Academy,
  Farm,
  IronMine,
  Prefecture,
  Quarry,
  Sawmill,
  Silo,
  Stable,
  Wall,
  Warehouse,
}

/// Informações sobre o edifício num determinado nível.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildingStats {
  pub level: BuildingLevel,
  pub base_cost: BaseCost,
  pub resources: Resources,
  pub maintenance: Maintenance,
  pub workforce: Workforce,
}

#[derive(Clone, Debug, Deref, Deserialize, Serialize)]
pub struct BuildingStatsTable(HashMap<BuildingLevel, BuildingStats>);

impl BuildingStatsTable {
  pub(crate) fn new(building: &dyn Building) -> Self {
    let max_level = building.max_level();
    let mut table = HashMap::with_capacity((max_level.0 + 1).into());

    let wood_ratio = *building.wood_ratio();
    let stone_ratio = *building.stone_ratio();
    let iron_ratio = *building.iron_ratio();
    let maintenance_ratio = *building.maintenance_ratio();

    let mut base_cost = f64::from(building.base_cost());
    let mut maintenance = base_cost * maintenance_ratio;
    let mut workforce = f64::from(building.workforce());

    let base_cost_growth = *building.base_cost_growth();
    let workforce_growth = *building.workforce_growth();

    for level in (0..=max_level.0).rev() {
      let level = BuildingLevel::new(level);
      let resources = Resources {
        food: Food::zero(),
        iron: Iron::from((base_cost * wood_ratio).ceil()),
        stone: Stone::from((base_cost * stone_ratio).ceil()),
        wood: Wood::from((base_cost * iron_ratio).ceil()),
      };

      table.insert(
        level,
        BuildingStats {
          level,
          base_cost: BaseCost::from(base_cost.ceil()),
          resources,
          maintenance: Maintenance::from(maintenance.ceil()),
          workforce: Workforce::from(workforce.ceil()),
        },
      );

      debug_assert!(base_cost.is_finite());
      debug_assert!(base_cost >= 0.0);

      debug_assert!(maintenance.is_finite());
      debug_assert!(maintenance >= 0.0);

      debug_assert!(workforce.is_finite());
      debug_assert!(workforce >= 0.0);

      base_cost -= base_cost * base_cost_growth;
      maintenance = base_cost * maintenance_ratio;
      workforce -= workforce * workforce_growth;
    }

    table.shrink_to_fit();

    Self(table)
  }
}

#[derive(Clone, Copy, Debug, Default, Deref, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct BuildingLevel(u8);

impl BuildingLevel {
  #[inline]
  pub const fn new(level: u8) -> Self {
    Self(level)
  }
}

impl PartialEq<u8> for BuildingLevel {
  fn eq(&self, other: &u8) -> bool {
    self.0.eq(other)
  }
}

impl PartialOrd<u8> for BuildingLevel {
  fn partial_cmp(&self, other: &u8) -> Option<cmp::Ordering> {
    self.0.partial_cmp(other)
  }
}

impl fmt::Display for BuildingLevel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}
