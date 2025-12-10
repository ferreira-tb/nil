// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod prefecture;

use crate::behavior::index::IdleBehavior;
use crate::behavior::{Behavior, BehaviorScore, process};
use crate::continent::Coord;
use crate::error::Result;
use crate::world::World;
use bon::Builder;

#[derive(Builder, Debug)]
pub struct BuildBehavior {
  coord: Coord,
}

impl Behavior for BuildBehavior {
  fn score(&self, world: &World) -> Result<BehaviorScore> {
    let infrastructure = world.infrastructure(self.coord)?;
    let in_queue = infrastructure.prefecture().turns_in_queue();
    let score = BehaviorScore::new(1.0 - (in_queue / 3.0));

    Ok(score)
  }

  fn behave(&self, world: &mut World) -> Result<()> {
    let behaviors = vec![IdleBehavior.boxed()];
    process(world, &behaviors)
  }
}

#[cfg(test)]
mod tests {}
