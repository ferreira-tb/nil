// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod manager;

use crate::resource::Resources;
use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

pub use manager::PlayerManager;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
  id: PlayerId,
  status: PlayerStatus,
  resources: Resources,
}

impl Player {
  pub fn new(options: PlayerOptions) -> Self {
    Self {
      id: options.id,
      status: PlayerStatus::Guest,
      resources: Resources::PLAYER.clone(),
    }
  }

  pub fn id(&self) -> PlayerId {
    self.id.clone()
  }

  pub fn status(&self) -> PlayerStatus {
    self.status
  }

  pub(crate) fn status_mut(&mut self) -> &mut PlayerStatus {
    &mut self.status
  }

  pub fn resources(&self) -> &Resources {
    &self.resources
  }

  pub(crate) fn resources_mut(&mut self) -> &mut Resources {
    &mut self.resources
  }

  pub fn is_guest(&self) -> bool {
    matches!(self.status, PlayerStatus::Guest)
  }

  pub fn is_active(&self) -> bool {
    matches!(self.status, PlayerStatus::Active)
  }

  pub fn is_inactive(&self) -> bool {
    matches!(self.status, PlayerStatus::Inactive)
  }
}

#[derive(Debug, Display, From, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[from(String, &str, Arc<str>, Box<str>)]
pub struct PlayerId(Arc<str>);

impl Clone for PlayerId {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}

impl AsRef<str> for PlayerId {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl Deref for PlayerId {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlayerStatus {
  #[default]
  Guest,
  Active,
  Inactive,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerOptions {
  pub id: PlayerId,
}
