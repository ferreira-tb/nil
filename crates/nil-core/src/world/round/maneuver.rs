// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::battle::{Battle, BattleResult};
use crate::error::Result;
use crate::infrastructure::building::{Building, BuildingLevel};
use crate::military::army::{Army, ArmyState};
use crate::military::maneuver::{Maneuver, ManeuverDirection, ManeuverKind};
use crate::report::battle::BattleReport;
use crate::resources::Resources;
use crate::ruler::Ruler;
use crate::world::World;
use itertools::Itertools;
use nil_util::result::WrapOk;

impl World {
  pub(super) fn process_maneuvers(&mut self) -> Result<()> {
    for maneuver in self.military.advance_maneuvers()? {
      debug_assert!(maneuver.is_done());
      match maneuver.direction() {
        ManeuverDirection::Going => self.process_going_maneuver(maneuver)?,
        ManeuverDirection::Returning => self.process_returning_maneuver(&maneuver)?,
      }
    }

    Ok(())
  }

  fn process_going_maneuver(&mut self, mut maneuver: Maneuver) -> Result<()> {
    let army_id = maneuver.army();
    let destination = maneuver.destination();
    let rulers = ManeuverRulers::resolve(self, &maneuver)?;

    match maneuver.kind() {
      ManeuverKind::Attack => {
        // TODO: Emit battle report and apply defender losses.
        let battle_result = perform_battle(self, &maneuver)?;
        let attacker_surviving_personnel = battle_result.attacker_surviving_personnel();

        if attacker_surviving_personnel.is_empty() {
          self.military.remove_army(army_id)?;
        } else {
          let army = self.military.army_mut(army_id)?;
          *army.personnel_mut() = attacker_surviving_personnel.clone();

          maneuver.reverse()?;
          self.military.insert_maneuver(maneuver);
        }
      }
      ManeuverKind::Support => {
        self
          .military
          .relocate_army(army_id, destination)?;
      }
    }

    Ok(())
  }

  fn process_returning_maneuver(&mut self, maneuver: &Maneuver) -> Result<()> {
    let army = self.military.army_mut(maneuver.army())?;
    *army.state_mut() = ArmyState::Idle;
    Ok(())
  }
}

struct ManeuverRulers {
  sender: Ruler,
  targets: Vec<Ruler>,
}

impl ManeuverRulers {
  fn resolve(world: &World, maneuver: &Maneuver) -> Result<Self> {
    let sender = world
      .military
      .army(maneuver.army())?
      .owner()
      .clone();

    let mut targets = match maneuver.kind() {
      ManeuverKind::Attack => {
        world
          .military
          .idle_armies_at(maneuver.destination())
          .map(Army::owner)
          .unique()
          .cloned()
          .collect_vec()
      }
      ManeuverKind::Support => {
        let owner = world.city(maneuver.destination())?.owner();
        owner
          .is_player()
          .then(|| vec![owner.clone()])
          .unwrap_or_default()
      }
    };

    targets.retain(|target| target != &sender);

    Ok(Self { sender, targets })
  }

  fn emit_battle_report(self, world: &World, report: &BattleReport) {
    if let Some(player) = self.sender.player().cloned() {
      world.emit_battle_report(player, report.clone());
    }

    for target in self.targets {
      debug_assert_ne!(self.sender, target);
      if let Some(player) = target.player().cloned() {
        world.emit_battle_report(player, report.clone());
      }
    }
  }
}

fn perform_battle(world: &World, maneuver: &Maneuver) -> Result<BattleResult> {
  let attacker = world.military.squads(maneuver.army())?;
  let defender = world
    .military
    .idle_squads_at(maneuver.destination());

  let wall = world
    .infrastructure(maneuver.destination())?
    .wall()
    .level();

  let wall_stats = (wall > BuildingLevel::ZERO)
    .then(|| world.stats.infrastructure.wall().get(wall))
    .transpose()?;

  Battle::builder()
    .attacker(&attacker)
    .defender(&defender)
    .maybe_wall(wall_stats)
    .build()
    .result()
    .wrap_ok()
}

/// The amount of resources hauled should be the smallest of the following:
/// - hauling capacity of the attacking army;
/// - target ruler's current resources;
/// - storage capacity of the target city;
/// - proportion of the target city's storage capacity to the attacked ruler's total storage capacity.
fn calculate_hauled_resources(
  world: &World,
  maneuver: &Maneuver,
  attacker: &Army,
) -> Result<Resources> {
  let mut haul = attacker.haul();

  let target = world.city(maneuver.destination())?.owner();
  // let target_resources = world.g

  Ok(Resources::new())
}
