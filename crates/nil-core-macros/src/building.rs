// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod mine;
mod storage;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub use mine::impl_mine;
pub use storage::impl_storage;

pub fn impl_building(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    mod __impl_building {
      use super::#name;
      use crate::error::Result;
      use crate::infrastructure::requirements::InfrastructureRequirements;
      use crate::infrastructure::building::{
        Building,
        BuildingId,
        BuildingLevel,
        BuildingStatsTable,
      };
      use crate::resource::{
        BaseCost,
        BaseCostGrowth,
        Maintenance,
        MaintenanceRatio,
        ResourceRatio,
        Workforce,
        WorkforceGrowth,
      };

      impl Building for #name {
        fn id(&self) -> BuildingId {
          Self::ID
        }

        fn is_enabled(&self) -> bool {
          self.enabled
        }

        fn toggle(&mut self, enabled: bool) {
          self.enabled = enabled;
        }

        fn level(&self) -> BuildingLevel {
          self.level
        }

        fn min_level(&self) -> BuildingLevel {
          Self::MIN_LEVEL
        }

        fn max_level(&self) -> BuildingLevel {
          Self::MAX_LEVEL
        }

        fn set_level(&mut self, level: BuildingLevel) {
          self.level = level.clamp(Self::MIN_LEVEL, Self::MAX_LEVEL);
        }

        fn increase_level(&mut self) {
          self.increase_level_by(1u8)
        }

        fn increase_level_by(&mut self, amount: u8) {
          self.set_level(self.level + amount)
        }

        fn decrease_level(&mut self) {
          self.decrease_level_by(1u8)
        }

        fn decrease_level_by(&mut self, amount: u8) {
          self.set_level(self.level - amount)
        }

        fn base_cost(&self) -> BaseCost {
          Self::BASE_COST
        }

        fn base_cost_growth(&self) -> BaseCostGrowth {
          Self::BASE_COST_GROWTH
        }

        fn maintenance(&self, stats: &BuildingStatsTable) -> Result<Maintenance> {
          Ok(stats.get(self.level)?.maintenance)
        }

        fn maintenance_ratio(&self) -> MaintenanceRatio {
          Self::MAINTENANCE_RATIO
        }

        fn wood_ratio(&self) -> ResourceRatio {
          Self::WOOD_RATIO
        }

        fn stone_ratio(&self) -> ResourceRatio {
          Self::STONE_RATIO
        }

        fn iron_ratio(&self) -> ResourceRatio {
          Self::IRON_RATIO
        }

        fn workforce(&self) -> Workforce {
          Self::WORKFORCE
        }

        fn workforce_growth(&self) -> WorkforceGrowth {
          Self::WORKFORCE_GROWTH
        }

        fn infrastructure_requirements(&self) -> &InfrastructureRequirements {
          &Self::INFRASTRUCTURE_REQUIREMENTS
        }
      }
    }
  };

  stream.into()
}
