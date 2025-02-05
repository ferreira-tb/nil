use super::{Cell, World};
use crate::error::{Error, Result};
use crate::village::{Coord, Village};

impl World {
  pub fn village(&self, coord: impl Into<Coord>) -> Result<&Village> {
    let coord = coord.into();
    match self.cell(coord)? {
      Cell::Village(village) => Ok(village),
      _ => Err(Error::NotAVillage(coord)),
    }
  }

  pub fn village_mut(&mut self, coord: impl Into<Coord>) -> Result<&mut Village> {
    let coord = coord.into();
    match self.cell_mut(coord)? {
      Cell::Village(village) => Ok(village),
      _ => Err(Error::NotAVillage(coord)),
    }
  }
}
