// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod army;
pub mod squad;
pub mod unit;

use crate::continent::ContinentIndex;
use crate::military::army::Army;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Military(HashMap<ContinentIndex, Vec<Army>>);
