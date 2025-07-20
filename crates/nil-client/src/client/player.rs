// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::continent::Coord;
use nil_core::infrastructure::storage::OverallStorageCapacity;
use nil_core::player::{Player, PlayerId, PlayerOptions, PlayerStatus};
use nil_core::resources::Maintenance;

impl Client {
  /// GET `/player`
  pub async fn get_players(&self) -> Result<Vec<Player>> {
    self.http.get_json("player").await
  }

  /// POST `/player`
  pub async fn get_player(&self, id: PlayerId) -> Result<Player> {
    self.http.post_json("player", id).await
  }

  /// GET `/player/capacity`
  pub async fn get_player_storage_capacity(&self) -> Result<OverallStorageCapacity> {
    self.http.get_json("player/capacity").await
  }

  /// GET `/player/maintenance`
  pub async fn get_player_maintenance(&self) -> Result<Maintenance> {
    self
      .http
      .get_json("player/maintenance")
      .await
  }

  /// POST `/player/spawn`
  pub async fn spawn_player(&self, options: PlayerOptions) -> Result<()> {
    self.http.post("player/spawn", options).await
  }

  /// POST `/player/status`
  pub async fn set_player_status(&self, status: PlayerStatus) -> Result<()> {
    self.http.post("player/status", status).await
  }

  /// GET `/player/{id}/coord`
  pub async fn get_player_coords(&self, id: PlayerId) -> Result<Vec<Coord>> {
    self
      .http
      .get_json(&format!("player/{id}/coord"))
      .await
  }

  /// GET `/player/{id}/exists`
  pub async fn player_exists(&self, id: PlayerId) -> Result<bool> {
    self
      .http
      .get_json(&format!("player/{id}/exists"))
      .await
  }

  /// GET `/player/{id}/status`
  pub async fn get_player_status(&self, id: PlayerId) -> Result<PlayerStatus> {
    self
      .http
      .get_json(&format!("player/{id}/status"))
      .await
  }
}
