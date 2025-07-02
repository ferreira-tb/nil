// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::round::Round;

impl Client {
  /// GET `/round`
  pub async fn get_round(&self) -> Result<Round> {
    self.http.get_json("round").await
  }

  /// POST `/round/end-turn`
  pub async fn end_turn(&self) -> Result<()> {
    self
      .http
      .post("round/end-turn", &self.player)
      .await
  }

  /// GET `/round/start`
  pub async fn start_round(&self) -> Result<()> {
    self.http.get("round/start").await
  }
}
