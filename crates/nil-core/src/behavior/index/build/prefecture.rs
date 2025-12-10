// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::behavior::{Behavior, BehaviorScore};
use crate::continent::Coord;
use crate::error::Result;
use crate::infrastructure::building::{Building, BuildingId};
use crate::world::World;
use bon::Builder;
use std::marker::PhantomData;

#[derive(Builder, Debug)]
pub struct BuildPrefectureBehavior<T>
where
  T: Building,
{
  coord: Coord,
  building: BuildingId,
  marker: PhantomData<T>,
}

impl<T> Behavior for BuildPrefectureBehavior<T>
where
  T: Building + 'static,
{
  fn score(&self, world: &World) -> Result<BehaviorScore> {
    let infrastructure = world.infrastructure(self.coord)?;
    let building = infrastructure.building(self.building);

    if building.is_max_level()
      || !building
        .infrastructure_requirements()
        .has_required_levels(infrastructure)
    {
      return Ok(BehaviorScore::new(0.0));
    }

    let owner = world.continent().owner_of(self.coord)?;
    let stats = world.stats().infrastructure();
    let required_resources = &stats
      .building(self.building)?
      .get(building.level() + 1u8)?
      .resources;

    if !world
      .ruler(owner.clone())?
      .has_resources(required_resources)
    {
      return Ok(BehaviorScore::new(0.0));
    }

    let score = base_score(world, self.building)?;

    todo!()
  }

  fn behave(&self, _: &mut World) -> Result<()> {
    Ok(())
  }
}

fn base_score(world: &World, building: BuildingId) -> Result<BehaviorScore> {
  let round = world.round().id();

  todo!()
}
