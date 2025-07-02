// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::infrastructure::building::{BuildingId, BuildingLevel};
use crate::resource::Workforce;
use derive_more::Deref;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::VecDeque;
use strum::EnumIs;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildQueue {
  orders: VecDeque<BuildOrder>,
  current_id: BuildOrderId,
}

impl BuildQueue {
  /// Consome força de trabalho até que ela se esgote ou toda a fila de construção seja concluída.
  pub(super) fn process(&mut self, mut workforce: Workforce) {
    loop {
      if workforce == 0
        || self
          .orders
          .pop_front_if(|order| order.update(&mut workforce))
          .is_none()
      {
        break;
      }
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildOrder {
  id: BuildOrderId,
  building: BuildingId,
  level: BuildingLevel,
  workforce: Workforce,
  status: BuildOrderStatus,
}

impl BuildOrder {
  #[inline]
  pub fn id(&self) -> BuildOrderId {
    self.id
  }

  /// Consome força de trabalho para avançar o progresso da ordem de construção.
  ///
  /// Retorna um booleano indicando se a construção foi concluída.
  fn update(&mut self, workforce: &mut Workforce) -> bool {
    let BuildOrderStatus::Pending(current) = &mut self.status else {
      return true;
    };

    if *current == 0 {
      self.status = BuildOrderStatus::Done;
      return true;
    }

    let previous = *current;
    *current -= *workforce;

    // Reduz a força de trabalho disponível com base na
    // quantidade usada para essa ordem de construção.
    *workforce -= previous - *current;

    if *current == 0 {
      self.status = BuildOrderStatus::Done;
    }

    self.status.is_done()
  }
}

#[derive(Clone, Copy, Debug, Default, Deref)]
pub struct BuildOrderId(u64);

impl BuildOrderId {
  #[inline]
  #[must_use]
  pub const fn next(self) -> Self {
    Self(self.0.wrapping_add(1))
  }
}

impl Serialize for BuildOrderId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.0.to_string())
  }
}

impl<'de> Deserialize<'de> for BuildOrderId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    String::deserialize(deserializer)?
      .parse::<u64>()
      .map(Self)
      .map_err(D::Error::custom)
  }
}

#[derive(Clone, Debug, EnumIs, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum BuildOrderStatus {
  Done,
  Pending(Workforce),
}

impl BuildOrderStatus {
  fn new(workforce: Workforce) -> Self {
    Self::Pending(workforce)
  }
}
