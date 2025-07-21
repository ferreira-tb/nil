// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::military::unit::{Unit, UnitBox, UnitId, UnitKind};
use derive_more::{Deref, Into};
use serde::{Deserialize, Serialize};

/// A group of units of the same type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Squad {
  unit: UnitBox,
  amount: u32,
}

impl Squad {
  pub fn new(id: UnitId, amount: u32) -> Self {
    Self { unit: UnitBox::from(id), amount }
  }

  #[inline]
  pub fn unit(&self) -> &dyn Unit {
    self.unit.as_dyn()
  }

  #[inline]
  pub fn kind(&self) -> UnitKind {
    self.unit.kind()
  }

  #[inline]
  pub fn amount(&self) -> u32 {
    self.amount
  }

  pub fn attack(&self) -> SquadAttack {
    let attack = self.unit.stats().attack;
    let total = f64::from(attack * self.amount);
    SquadAttack::new(total)
  }

  pub fn defense(&self) -> SquadDefense {
    let infantry = self.unit.stats().infantry_defense;
    let cavalry = self.unit.stats().cavalry_defense;
    let ranged = self.unit.stats().ranged_defense;

    let general_total = f64::from(infantry * self.amount);
    let cavalry_total = f64::from(cavalry * self.amount);
    let ranged_total = f64::from(ranged * self.amount);

    SquadDefense {
      infantry: general_total,
      cavalry: cavalry_total,
      ranged: ranged_total,
    }
  }
}

impl From<UnitId> for Squad {
  fn from(id: UnitId) -> Self {
    Squad::new(id, 0)
  }
}

#[derive(Clone, Copy, Debug, Deref, Into)]
pub struct SquadAttack(f64);

impl SquadAttack {
  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.max(0.0))
  }
}

impl From<f64> for SquadAttack {
  fn from(value: f64) -> Self {
    Self::new(value)
  }
}

pub struct SquadDefense {
  pub(crate) infantry: f64,
  pub(crate) cavalry: f64,
  pub(crate) ranged: f64,
}
