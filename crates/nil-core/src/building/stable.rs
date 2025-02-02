use super::BuildingLevel;
use nil_macros::Building;

#[derive(Building, Debug)]
pub struct Stable {
  level: BuildingLevel,
}

impl Stable {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(20);
}

impl Default for Stable {
  fn default() -> Self {
    Self { level: BuildingLevel::new(0) }
  }
}
