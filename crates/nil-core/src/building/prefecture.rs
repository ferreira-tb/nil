use super::BuildingLevel;
use nil_macros::Building;

#[derive(Building, Debug)]
pub struct Prefecture {
  level: BuildingLevel,
}

impl Prefecture {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);
}

impl Default for Prefecture {
  fn default() -> Self {
    Self { level: BuildingLevel::new(1) }
  }
}
