// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::military::squad::SquadSize;
use crate::resources::prelude::*;
use derive_more::{Deref, From, Into};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Clone, Copy, Debug, Deref, Default, From, Into, Deserialize, Serialize)]
#[from(u32, Food, Iron, Stone, Wood)]
#[into(u32, f64, Food, Iron, Stone, Wood)]
pub struct Haul(u32);

impl Haul {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl From<Resources> for Haul {
  fn from(resources: Resources) -> Self {
    Haul::new(resources.sum())
  }
}

impl Add for Haul {
  type Output = Haul;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl AddAssign for Haul {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl Mul<SquadSize> for Haul {
  type Output = Haul;

  fn mul(self, rhs: SquadSize) -> Self::Output {
    Self(self.0.saturating_mul(*rhs))
  }
}

impl MulAssign<SquadSize> for Haul {
  fn mul_assign(&mut self, rhs: SquadSize) {
    *self = *self * rhs;
  }
}
