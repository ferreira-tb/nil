use super::Client;
use crate::error::Result;
use nil_core::{Player, PlayerConfig, PlayerId};
use serde_json::json;

impl Client {
  pub async fn get_player(&self, id: PlayerId) -> Result<Player> {
    self
      .post_json("player", json!({ "id": id }))
      .await
  }

  pub async fn spawn_player(&self, config: PlayerConfig) -> Result<PlayerId> {
    self.put_json("player/spawn", config).await
  }
}
