use super::BuildingLevel;
use nil_macros::Building;

#[derive(Building, Debug)]
pub struct Warehouse {
  level: BuildingLevel,
}

impl Warehouse {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);
}

impl Default for Warehouse {
  fn default() -> Self {
    Self { level: BuildingLevel::new(1) }
  }
}
