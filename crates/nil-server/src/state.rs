use nil_core::continent::Continent;
use nil_core::player::PlayerManager;
use nil_core::round::Round;
use nil_core::world::World;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) struct App {
  world: Arc<RwLock<World>>,
}

impl App {
  pub fn new(world: World) -> Self {
    Self { world: Arc::new(RwLock::new(world)) }
  }

  pub async fn world<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&World) -> T,
  {
    f(&*self.world.read().await)
  }

  pub async fn world_mut<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&mut World) -> T,
  {
    f(&mut *self.world.write().await)
  }

  pub async fn continent<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&Continent) -> T,
  {
    self
      .world(|world| f(world.continent()))
      .await
  }

  pub async fn player_manager<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&PlayerManager) -> T,
  {
    self
      .world(|world| f(world.player_manager()))
      .await
  }

  pub async fn round<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&Round) -> T,
  {
    self.world(|world| f(world.round())).await
  }
}

impl Clone for App {
  fn clone(&self) -> Self {
    Self { world: Arc::clone(&self.world) }
  }
}
