// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::error::Result;
use crate::village::Coord;

impl World {
  pub fn rename_village(&mut self, coord: Coord, name: &str) -> Result<()> {
    let village = self.continent.village_mut(coord)?;
    *village.name_mut() = name.to_owned();
    self.emit_public_village_updated(coord);
    self.emit_village_updated(coord);
    Ok(())
  }
}
