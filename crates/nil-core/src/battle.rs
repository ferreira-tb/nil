use crate::unit::{UnitBox, UnitKind};
use bon::Builder;

#[derive(Builder)]
pub struct Battle {
  #[builder(default, into)]
  attacker: Vec<UnitBox>,
  #[builder(default, into)]
  defender: Vec<UnitBox>,
}

impl Battle {
  pub fn offensive_power(&self) -> OffensivePower {
    OffensivePower::new(&self.attacker)
  }

  pub fn defensive_power(&self) -> DefensivePower {
    DefensivePower::new(&self.defender, self.offensive_power())
  }
}

#[derive(Copy, Clone, Debug)]
pub struct OffensivePower {
  pub total: f64,
  pub general_ratio: f64,
  pub cavalry_ratio: f64,
  pub ranged_ratio: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct DefensivePower {
  pub total: f64,
  pub general_ratio: f64,
  pub cavalry_ratio: f64,
  pub ranged_ratio: f64,
}

impl OffensivePower {
  pub fn new(units: &[UnitBox]) -> Self {
    let mut general = 0.0;
    let mut cavalry = 0.0;
    let mut ranged = 0.0;

    for unit in units {
      match unit.kind() {
        UnitKind::Infantry => {
          general += *unit.squad_attack();
        }
        UnitKind::Cavalry => {
          cavalry += *unit.squad_attack();
        }
        UnitKind::Ranged => {
          ranged += *unit.squad_attack();
        }
      }
    }

    let total = general + cavalry + ranged;

    OffensivePower {
      total,
      general_ratio: general / total,
      cavalry_ratio: cavalry / total,
      ranged_ratio: ranged / total,
    }
  }
}

impl DefensivePower {
  pub fn new(units: &[UnitBox], offensive_power: OffensivePower) -> Self {
    let mut general = 0.0;
    let mut cavalry = 0.0;
    let mut ranged = 0.0;

    for unit in units {
      general += unit.squad_defense().general;
      cavalry += unit.squad_defense().cavalry;
      ranged += unit.squad_defense().ranged;
    }

    general *= offensive_power.general_ratio;
    cavalry *= offensive_power.cavalry_ratio;
    ranged *= offensive_power.ranged_ratio;

    let total = general + cavalry + ranged;

    DefensivePower {
      total,
      general_ratio: general / total,
      cavalry_ratio: cavalry / total,
      ranged_ratio: ranged / total,
    }
  }
}
