// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::infrastructure::building::{BuildingId, BuildingLevel};
use nil_core::resource::{Food, Iron, Resources, Stone, Wood};
use nil_core::village::{Coord, Stability};

impl Client {
  /// POST `/cheat/infrastructure`
  pub async fn cheat_set_max_infrastructure(&self, coord: Coord) -> Result<()> {
    self
      .http
      .post("cheat/infrastructure", coord)
      .await
  }

  /// POST `/cheat/infrastructure/building`
  pub async fn cheat_set_building_level(
    &self,
    coord: Coord,
    id: BuildingId,
    level: BuildingLevel,
  ) -> Result<()> {
    self
      .http
      .post("cheat/infrastructure/building", (coord, id, level))
      .await
  }

  /// GET `/cheat/resources`
  pub async fn cheat_set_max_resources(&self) -> Result<()> {
    self.http.get("cheat/resources").await
  }

  /// POST `/cheat/resources`
  pub async fn cheat_set_resources(&self, resources: Resources) -> Result<()> {
    self
      .http
      .post("cheat/resources", resources)
      .await
  }

  /// POST `/cheat/resources/food`
  pub async fn cheat_set_food(&self, food: Food) -> Result<()> {
    self
      .http
      .post("cheat/resources/food", food)
      .await
  }

  /// POST `/cheat/resources/iron`
  pub async fn cheat_set_iron(&self, iron: Iron) -> Result<()> {
    self
      .http
      .post("cheat/resources/iron", iron)
      .await
  }

  /// GET `/cheat/resources/silo`
  pub async fn cheat_set_max_silo_resources(&self) -> Result<()> {
    self.http.get("cheat/resources/silo").await
  }

  /// POST `/cheat/resources/stone`
  pub async fn cheat_set_stone(&self, stone: Stone) -> Result<()> {
    self
      .http
      .post("cheat/resources/stone", stone)
      .await
  }

  /// GET `/cheat/resources/warehouse`
  pub async fn cheat_set_max_warehouse_resources(&self) -> Result<()> {
    self
      .http
      .get("cheat/resources/warehouse")
      .await
  }

  /// POST `/cheat/resources/wood`
  pub async fn cheat_set_wood(&self, wood: Wood) -> Result<()> {
    self
      .http
      .post("cheat/resources/wood", wood)
      .await
  }

  /// POST `/cheat/village/stability`
  pub async fn cheat_set_stability(&self, coord: Coord, stability: Stability) -> Result<()> {
    self
      .http
      .post("cheat/village/stability", (coord, stability))
      .await
  }
}
