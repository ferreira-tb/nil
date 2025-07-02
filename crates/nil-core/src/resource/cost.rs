// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::Deref;
use serde::{Deserialize, Serialize};

/// Custo base de uma entidade, como edifícios ou unidades.
#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct BaseCost(u32);

impl BaseCost {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl From<BaseCost> for f64 {
  fn from(value: BaseCost) -> Self {
    f64::from(value.0)
  }
}

impl From<f64> for BaseCost {
  fn from(value: f64) -> Self {
    Self::new(value as u32)
  }
}

#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct BaseCostGrowth(f64);

impl BaseCostGrowth {
  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.clamp(0.0, 1.0))
  }
}

/// Proporção entre o custo total e um dado recurso.
#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct ResourceRatio(f64);

impl ResourceRatio {
  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.clamp(0.0, 1.0))
  }

  #[inline]
  pub const fn as_f64(self) -> f64 {
    self.0
  }
}

/// Verifica, durante a compilação, se a soma das proporções dos recursos é igual a 1.
#[macro_export]
macro_rules! check_total_resource_ratio {
  ($first:expr, $($other:expr),+ $(,)?) => {
    const _: () = {
      let mut first = $first.as_f64();
      $(first += $other.as_f64();)+
      assert!((first - 1.0).abs() < 0.001);
    };
  };
}
