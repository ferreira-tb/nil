// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::response::{MaybeResponse, from_err};
use dashmap::DashMap;
use either::Either;
use nil_core::chat::Chat;
use nil_core::continent::Continent;
use nil_core::npc::bot::BotManager;
use nil_core::npc::precursor::PrecursorManager;
use nil_core::player::PlayerManager;
use nil_core::ranking::Ranking;
use nil_core::report::ReportManager;
use nil_core::round::Round;
use nil_core::world::{World, WorldId};
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) struct App {
  worlds: Arc<DashMap<WorldId, Arc<RwLock<World>>>>,
}

impl App {
  pub fn new() -> Self {
    Self { worlds: Arc::new(DashMap::new()) }
  }

  pub fn with_world(world: World) -> Self {
    let app = Self::new();
    let id = world.config().id();
    app
      .worlds
      .insert(id, Arc::new(RwLock::new(world)));

    app
  }

  pub(crate) fn get(&self, id: WorldId) -> Result<Arc<RwLock<World>>> {
    self
      .worlds
      .get(&id)
      .map(|world| Arc::clone(&world))
      .ok_or_else(|| Error::InvalidWorld(id))
  }

  pub async fn world<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&World) -> T,
  {
    match self.get(id) {
      Ok(world) => Either::Left(f(&*world.read().await)),
      Err(err) => Either::Right(from_err(err)),
    }
  }

  pub async fn world_mut<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&mut World) -> T,
  {
    match self.get(id) {
      Ok(world) => Either::Left(f(&mut *world.write().await)),
      Err(err) => Either::Right(from_err(err)),
    }
  }

  pub async fn bot_manager<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&BotManager) -> T,
  {
    self
      .world(id, |world| f(world.bot_manager()))
      .await
  }

  pub async fn chat<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&Chat) -> T,
  {
    self.world(id, |world| f(world.chat())).await
  }

  pub async fn continent<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&Continent) -> T,
  {
    self
      .world(id, |world| f(world.continent()))
      .await
  }

  pub async fn player_manager<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&PlayerManager) -> T,
  {
    self
      .world(id, |world| f(world.player_manager()))
      .await
  }

  pub async fn precursor_manager<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&PrecursorManager) -> T,
  {
    self
      .world(id, |world| f(world.precursor_manager()))
      .await
  }

  pub async fn ranking<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&Ranking) -> T,
  {
    self
      .world(id, |world| f(world.ranking()))
      .await
  }

  pub async fn report_manager<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&ReportManager) -> T,
  {
    self
      .world(id, |world| f(world.report()))
      .await
  }

  pub async fn round<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&Round) -> T,
  {
    self
      .world(id, |world| f(world.round()))
      .await
  }
}

impl Clone for App {
  fn clone(&self) -> Self {
    Self { worlds: Arc::clone(&self.worlds) }
  }
}

impl Default for App {
  fn default() -> Self {
    Self::new()
  }
}
