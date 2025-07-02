// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::lobby::LobbyState;
use nil_core::player::PlayerId;

impl Client {
  /// GET `/lobby`
  pub async fn get_lobby(&self) -> Result<LobbyState> {
    self.http.get_json("lobby").await
  }

  /// POST `/lobby/ready`
  pub async fn set_lobby_ready(&self, id: PlayerId, ready: bool) -> Result<()> {
    self
      .http
      .post("lobby/ready", (id, ready))
      .await
  }
}
