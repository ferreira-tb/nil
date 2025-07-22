// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod army;
pub mod squad;
pub mod unit;

use crate::continent::{ContinentIndex, ContinentSize, IntoContinentIndex};
use crate::military::army::Army;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Military {
  grid: HashMap<ContinentIndex, Vec<Army>>,
  size: ContinentSize,
}

// TODO: All armies should be reconciled when the round ends to avoid having
// multiple copies of idle armies owned by the same entity in the same location.
impl Military {
  pub(crate) fn new(size: ContinentSize) -> Self {
    Self { grid: HashMap::new(), size }
  }

  pub(crate) fn insert<I>(&mut self, index: I, army: Army)
  where
    I: IntoContinentIndex,
  {
    let index = index.into_index(self.size);
    self
      .grid
      .entry(index)
      .or_default()
      .push(army);
  }
}
