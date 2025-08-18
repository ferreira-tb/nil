// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::continent::Coord;
use nil_core::village::{PublicVillage, Village};

impl Client {
  /// POST `/village`
  pub async fn get_village(&self, coord: Coord) -> Result<Village> {
    self.http.post_json("village", coord).await
  }

  /// GET `/village/public`
  pub async fn get_public_villages(&self) -> Result<Vec<PublicVillage>> {
    self.http.get_json("village/public").await
  }

  /// POST `/village/public`
  pub async fn get_public_village(&self, coord: Coord) -> Result<PublicVillage> {
    self
      .http
      .post_json("village/public", coord)
      .await
  }

  /// POST `/village/public-by`
  pub async fn get_public_villages_by(&self, coords: Vec<Coord>) -> Result<Vec<PublicVillage>> {
    self
      .http
      .post_json("village/public-by", coords)
      .await
  }

  /// POST `/village/rename`
  pub async fn rename_village(&self, coord: Coord, name: &str) -> Result<()> {
    self
      .http
      .post("village/rename", (coord, name))
      .await
  }
}
