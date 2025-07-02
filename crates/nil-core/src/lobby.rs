// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::event::{Emitter, Event};
use crate::player::PlayerId;
use crate::world::World;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[cfg(debug_assertions)]
use tracing::info;

#[derive(Debug)]
pub struct Lobby {
  emitter: Emitter,
  ready: HashSet<PlayerId>,
}

impl Lobby {
  fn new(emitter: Emitter) -> Self {
    Self { emitter, ready: HashSet::default() }
  }

  pub fn state(&self) -> LobbyState {
    LobbyState::from(self)
  }

  pub fn set_ready(&mut self, id: PlayerId, ready: bool) {
    if ready {
      self.ready.insert(id);
    } else {
      self.ready.remove(&id);
    }

    self.emit_update();
  }

  fn broadcast(&self, event: Event) {
    #[cfg(debug_assertions)]
    info!(EVENT = ?event);

    self.emitter.broadcast(event);
  }

  fn emit_update(&self) {
    self.broadcast(Event::LobbyUpdated { lobby: self.state() });
  }
}

impl From<&World> for Lobby {
  fn from(world: &World) -> Self {
    Self::new(world.emitter())
  }
}

impl From<&Lobby> for LobbyState {
  fn from(lobby: &Lobby) -> Self {
    Self { ready: lobby.ready.clone() }
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LobbyState {
  ready: HashSet<PlayerId>,
}
