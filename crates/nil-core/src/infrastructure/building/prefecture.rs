// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod queue;

use super::{BuildingId, BuildingLevel};
use crate::check_total_resource_ratio;
use crate::resource::{
  BaseCost,
  BaseCostGrowth,
  MaintenanceRatio,
  ResourceRatio,
  Workforce,
  WorkforceGrowth,
};
use nil_core_macros::Building;
use serde::{Deserialize, Serialize};

pub use queue::{BuildOrder, BuildOrderId, BuildOrderStatus, BuildQueue};

/// Centro logístico da aldeia, responsável pela construção de edifícios.
#[derive(Building, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Prefecture {
  level: BuildingLevel,
  enabled: bool,
  queue: BuildQueue,
}

impl Prefecture {
  pub const ID: BuildingId = BuildingId::Prefecture;
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);

  pub const BASE_COST: BaseCost = BaseCost::new(150_000);
  pub const BASE_COST_GROWTH: BaseCostGrowth = BaseCostGrowth::new(0.2);
  pub const MAINTENANCE_RATIO: MaintenanceRatio = MaintenanceRatio::new(0.005);

  pub const WOOD_RATIO: ResourceRatio = ResourceRatio::new(0.3);
  pub const STONE_RATIO: ResourceRatio = ResourceRatio::new(0.5);
  pub const IRON_RATIO: ResourceRatio = ResourceRatio::new(0.2);

  pub const WORKFORCE: Workforce = Workforce::new(50);
  pub const WORKFORCE_GROWTH: WorkforceGrowth = WorkforceGrowth::new(0.2);

  pub(crate) fn process_queue(&mut self) {
    self.queue.process(self.level.into());
  }
}

impl Default for Prefecture {
  fn default() -> Self {
    Self {
      level: BuildingLevel::new(1),
      enabled: true,
      queue: BuildQueue::default(),
    }
  }
}

check_total_resource_ratio!(
  Prefecture::WOOD_RATIO,
  Prefecture::STONE_RATIO,
  Prefecture::IRON_RATIO,
);
