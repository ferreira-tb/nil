use super::BuildingLevel;
use nil_macros::Building;

#[derive(Building, Debug)]
pub struct Quarry {
  level: BuildingLevel,
}

impl Quarry {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);
}

impl Default for Quarry {
  fn default() -> Self {
    Self { level: BuildingLevel::new(1) }
  }
}
