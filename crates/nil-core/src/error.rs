use crate::player::PlayerId;
use crate::unit::UnitId;
use crate::village::Coord;
use serde::Serialize;
use serde::ser::Serializer;
use strum::EnumIs;

pub use std::result::Result as StdResult;

pub type Result<T, E = Error> = StdResult<T, E>;

#[non_exhaustive]
#[derive(Debug, EnumIs, thiserror::Error)]
#[remain::sorted]
pub enum Error {
  #[error("Coord out of bounds: {0:?}")]
  CoordOutOfBounds(Coord),
  #[error("Index out of bounds: {0}")]
  IndexOutOfBounds(usize),
  #[error("No player found")]
  NoPlayer,
  #[error("Player not found: {0}")]
  PlayerNotFound(PlayerId),
  #[error("Round already started")]
  RoundAlreadyStarted,
  #[error("Unit not found: {0}")]
  UnitNotFound(UnitId),
  #[error("Village not found: {0}")]
  VillageNotFound(Coord),
  #[error("World is full")]
  WorldIsFull,
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}
