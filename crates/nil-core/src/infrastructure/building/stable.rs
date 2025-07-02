// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{BuildingId, BuildingLevel};
use crate::check_total_resource_ratio;
use nil_core_macros::Building;
use serde::{Deserialize, Serialize};

use crate::resource::{
  BaseCost,
  BaseCostGrowth,
  MaintenanceRatio,
  ResourceRatio,
  Workforce,
  WorkforceGrowth,
};

#[derive(Building, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stable {
  level: BuildingLevel,
  enabled: bool,
}

impl Stable {
  pub const ID: BuildingId = BuildingId::Stable;
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(20);

  pub const BASE_COST: BaseCost = BaseCost::new(50_000);
  pub const BASE_COST_GROWTH: BaseCostGrowth = BaseCostGrowth::new(0.2);
  pub const MAINTENANCE_RATIO: MaintenanceRatio = MaintenanceRatio::new(0.005);

  pub const WOOD_RATIO: ResourceRatio = ResourceRatio::new(0.4);
  pub const STONE_RATIO: ResourceRatio = ResourceRatio::new(0.3);
  pub const IRON_RATIO: ResourceRatio = ResourceRatio::new(0.3);

  pub const WORKFORCE: Workforce = Workforce::new(150);
  pub const WORKFORCE_GROWTH: WorkforceGrowth = WorkforceGrowth::new(0.2);
}

impl Default for Stable {
  fn default() -> Self {
    Self {
      level: BuildingLevel::new(0),
      enabled: true,
    }
  }
}

check_total_resource_ratio!(Stable::WOOD_RATIO, Stable::STONE_RATIO, Stable::IRON_RATIO);
