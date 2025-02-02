use crate::building::prelude::*;
use crate::player::PlayerId;
use crate::world::Coord;
use bon::Builder;

#[derive(Builder, Debug)]
pub struct Village {
  #[builder(start_fn, into)]
  pub coord: Coord,
  #[builder(into)]
  pub name: String,
  #[builder(into)]
  pub owner: Option<PlayerId>,
  #[builder(default)]
  pub infrastructure: Infrastructure,
}

#[derive(Debug, Default)]
pub struct Infrastructure {
  pub prefecture: Prefecture,
  pub academy: Academy,
  pub stable: Stable,
  pub sawmill: Sawmill,
  pub quarry: Quarry,
  pub iron_mine: IronMine,
  pub farm: Farm,
  pub warehouse: Warehouse,
  pub wall: Wall,
}
