use super::prelude::*;

#[derive(Unit)]
pub struct Swordsman {
  amount: u32,
}

impl Swordsman {
  pub const ID: UnitId = UnitId::new(1);
  pub const KIND: UnitKind = UnitKind::Infantry;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(0),
    general_defense: Power::new(0),
    cavalry_defense: Power::new(0),
    speed: Speed::new(0.0),
    haul: Haul::new(0),
  };
}
