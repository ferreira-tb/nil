use super::BuildingLevel;
use nil_macros::Building;

#[derive(Building, Debug)]
pub struct Academy {
  level: BuildingLevel,
}

impl Academy {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(25);
}

impl Default for Academy {
  fn default() -> Self {
    Self { level: BuildingLevel::new(0) }
  }
}
