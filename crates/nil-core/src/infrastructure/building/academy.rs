// Copyright (C) Tsukilabs contributors
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
pub struct Academy {
  level: BuildingLevel,
  enabled: bool,
}

impl Academy {
  pub const ID: BuildingId = BuildingId::Academy;
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(25);

  pub const BASE_COST: BaseCost = BaseCost::new(50_000);
  pub const BASE_COST_GROWTH: BaseCostGrowth = BaseCostGrowth::new(0.2);
  pub const MAINTENANCE_RATIO: MaintenanceRatio = MaintenanceRatio::new(0.005);

  pub const WOOD_RATIO: ResourceRatio = ResourceRatio::new(0.3);
  pub const STONE_RATIO: ResourceRatio = ResourceRatio::new(0.4);
  pub const IRON_RATIO: ResourceRatio = ResourceRatio::new(0.3);

  pub const WORKFORCE: Workforce = Workforce::new(100);
  pub const WORKFORCE_GROWTH: WorkforceGrowth = WorkforceGrowth::new(0.2);
}

impl Default for Academy {
  fn default() -> Self {
    Self {
      level: BuildingLevel::new(0),
      enabled: true,
    }
  }
}

check_total_resource_ratio!(
  Academy::WOOD_RATIO,
  Academy::STONE_RATIO,
  Academy::IRON_RATIO,
);
