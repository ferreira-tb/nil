mod player;
mod round;

use crate::continent::Continent;
use crate::event::{Emitter, Event, Listener};
use crate::player::PlayerManager;
use crate::round::Round;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;
use std::sync::Arc;

#[derive(Debug)]
pub struct World {
  name: Arc<str>,
  continent: Continent,
  player_manager: PlayerManager,
  round: Round,
  emitter: Emitter,
}

impl World {
  pub fn new(config: WorldOptions) -> Self {
    Self {
      name: config.name.into(),
      continent: Continent::new(config.size.get()),
      player_manager: PlayerManager::default(),
      round: Round::default(),
      emitter: Emitter::default(),
    }
  }

  pub fn state(&self) -> WorldState {
    WorldState::from(self)
  }

  pub fn continent(&self) -> &Continent {
    &self.continent
  }

  pub fn continent_mut(&mut self) -> &mut Continent {
    &mut self.continent
  }

  pub fn player_manager(&self) -> &PlayerManager {
    &self.player_manager
  }

  pub fn player_manager_mut(&mut self) -> &mut PlayerManager {
    &mut self.player_manager
  }

  pub fn round(&self) -> &Round {
    &self.round
  }

  fn emit(&self, event: Event) {
    self.emitter.emit(event);
  }

  pub fn subscribe(&self) -> Listener {
    self.emitter.subscribe()
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WorldState {
  name: Arc<str>,
}

impl From<&World> for WorldState {
  fn from(world: &World) -> Self {
    Self { name: Arc::clone(&world.name) }
  }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldOptions {
  pub name: String,
  pub size: NonZeroU8,
}

impl WorldOptions {
  pub fn into_world(self) -> World {
    World::new(self)
  }
}
