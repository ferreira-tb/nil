mod academy;
mod farm;
mod iron_mine;
mod prefecture;
pub mod prelude;
mod quarry;
mod sawmill;
mod silo;
mod stable;
mod wall;
mod warehouse;

use derive_more::Deref;
use serde::{Deserialize, Serialize};

pub trait Building {
  fn level(&self) -> BuildingLevel;
  fn max_level(&self) -> BuildingLevel;
}

#[derive(Clone, Copy, Debug, Default, Deref, Deserialize, Serialize)]
pub struct BuildingLevel(u8);

impl BuildingLevel {
  pub const fn new(level: u8) -> Self {
    Self(level)
  }
}
