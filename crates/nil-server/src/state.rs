use nil_core::World;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) struct ServerState {
  world: Arc<RwLock<World>>,
}

impl ServerState {
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
}

impl Clone for ServerState {
  fn clone(&self) -> Self {
    Self { world: Arc::clone(&self.world) }
  }
}
