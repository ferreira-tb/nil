use super::BuildingLevel;
use nil_macros::Building;

#[derive(Building, Debug)]
pub struct Farm {
  level: BuildingLevel,
}

impl Farm {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);
}

impl Default for Farm {
  fn default() -> Self {
    Self { level: BuildingLevel::new(1) }
  }
}
