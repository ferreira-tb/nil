// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::village::{Coord, Village};

impl Client {
  /// POST `/village`
  pub async fn get_village(&self, coord: Coord) -> Result<Village> {
    self.http.post_json("village", coord).await
  }
}
