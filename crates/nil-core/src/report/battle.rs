// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::battle::BattleResult;
use crate::military::unit::stats::haul::Haul;
use crate::ruler::Ruler;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleReport {
  attacker: Ruler,
  defenders: Vec<Ruler>,
  haul: Haul,

  #[serde(flatten)]
  result: BattleResult,
}
