// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::Coord;
use crate::error::{Error, Result};
use crate::military::unit::{UnitBox, UnitId};
use crate::resources::{Resources, Workforce};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::num::NonZeroU32;
use strum::EnumIs;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademyRecruitQueue {
  orders: VecDeque<AcademyRecruitOrder>,
}

impl AcademyRecruitQueue {
  pub(crate) fn recruit(
    &mut self,
    request: &AcademyRecruitOrderRequest,
    current_resources: Option<&Resources>,
  ) -> Result<&AcademyRecruitOrder> {
    let unit = UnitBox::from(request.unit);
    let chunk = unit.as_dyn().chunk();
    let resources = &chunk.resources() * request.chunks;
    let workforce = chunk.workforce() * request.chunks;

    if let Some(current_resources) = current_resources
      && current_resources
        .checked_sub(&resources)
        .is_none()
    {
      return Err(Error::InsufficientResources);
    }

    self.orders.push_back(AcademyRecruitOrder {
      id: AcademyRecruitOrderId::new(),
      unit: unit.id(),
      resources,
      workforce,
      state: AcademyRecruitOrderState::new(workforce),
    });

    let len = self.orders.len();
    Ok(unsafe {
      self
        .orders
        .get(len.unchecked_sub(1))
        .unwrap_unchecked()
    })
  }

  pub fn iter(&self) -> impl Iterator<Item = &AcademyRecruitOrder> {
    self.orders.iter()
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.orders.len()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.orders.is_empty()
  }
}

#[must_use]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademyRecruitOrder {
  id: AcademyRecruitOrderId,
  unit: UnitId,
  resources: Resources,
  workforce: Workforce,
  state: AcademyRecruitOrderState,
}

impl AcademyRecruitOrder {
  #[inline]
  pub fn id(&self) -> AcademyRecruitOrderId {
    self.id
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AcademyRecruitOrderId(Uuid);

impl AcademyRecruitOrderId {
  #[must_use]
  pub fn new() -> Self {
    Self(Uuid::new_v4())
  }
}

impl Default for AcademyRecruitOrderId {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Clone, Debug, EnumIs, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum AcademyRecruitOrderState {
  Pending { workforce: Workforce },
  Done,
}

impl AcademyRecruitOrderState {
  fn workforce_mut(&mut self) -> Option<&mut Workforce> {
    if let Self::Pending { workforce } = self { Some(workforce) } else { None }
  }
}

impl AcademyRecruitOrderState {
  fn new(workforce: Workforce) -> Self {
    Self::Pending { workforce }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademyRecruitOrderRequest {
  pub coord: Coord,
  pub unit: UnitId,
  pub chunks: NonZeroU32,
}
