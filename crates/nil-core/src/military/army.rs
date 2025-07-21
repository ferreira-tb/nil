// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::military::squad::Squad;
use crate::military::unit::UnitId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::EnumIs;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Army {
  squads: HashMap<UnitId, Squad>,
  state: ArmyState,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, EnumIs)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ArmyState {
  #[default]
  Idle,
}
