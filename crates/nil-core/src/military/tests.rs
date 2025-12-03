// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::{ContinentSize, Coord};
use crate::military::Military;
use crate::military::army::{ArmyId, ArmyPersonnel};
use crate::military::maneuver::{Maneuver, ManeuverKind, ManeuverRequest};
use crate::npc::bot::{Bot, BotId};
use crate::ruler::Ruler;

#[test]
fn spawn() {
  let size = ContinentSize::new(50);
  let mut military = Military::new(size);
  assert_eq!(military.count_armies(), 0);

  let coord = Coord::splat(0);
  let ruler = make_ruler("Bot 1");
  let personnel = ArmyPersonnel::splat(10);
  military.spawn(coord, ruler, personnel);

  assert_eq!(military.count_armies(), 1);
}

#[test]
fn collapse_armies() {
  let size = ContinentSize::new(50);
  let mut military = Military::new(size);

  let coord = Coord::splat(0);
  let ruler = make_ruler("Bot 1");

  let mut spawn = |ruler: Ruler| {
    let personnel = ArmyPersonnel::splat(10);
    military.spawn(coord, ruler, personnel);
  };

  for _ in 0..10 {
    spawn(ruler.clone());
  }

  spawn(make_ruler("Bot 2"));

  assert_eq!(military.count_armies_at(coord), 11);
  military.collapse_armies();
  assert_eq!(military.count_armies_at(coord), 2);
}

#[test]
fn intersection() {
  let size = ContinentSize::new(50);
  let mut military = Military::new(size);

  let ruler = make_ruler("Bot 1");
  let mut coords = Vec::with_capacity(10);

  let mut spawn = |coord: Coord| {
    let personnel = ArmyPersonnel::splat(10);
    military.spawn(coord, ruler.clone(), personnel);
  };

  for i in 0..10 {
    let coord = Coord::splat(i);
    spawn(coord);
    spawn(coord);

    coords.push(coord);
  }

  assert_eq!(military.count_armies(), coords.len() * 2);

  let coords = coords.iter().take(3).copied();
  let military = military.intersection(coords).unwrap();
  assert_eq!(military.count_armies(), 3 * 2);
}

#[test]
#[should_panic]
fn origin_is_destination() {
  let coord = Coord::splat(0);
  let request = ManeuverRequest {
    army: ArmyId::new(),
    kind: ManeuverKind::Attack,
    origin: coord,
    destination: coord,
  };

  let _ = Maneuver::new(&request).unwrap();
}

fn make_ruler(id: &str) -> Ruler {
  Ruler::from(&Bot::new(BotId::from(id)))
}
