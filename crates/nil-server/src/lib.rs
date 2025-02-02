mod error;
mod response;
mod router;
mod state;
mod websocket;

pub use error::{Error, Result};
use nil_core::World;
use state::ServerState;
use std::net::SocketAddr;
use tauri::async_runtime::spawn;
use tokio::net::TcpListener;
use tokio::task::AbortHandle;

pub struct Server {
  abort_handle: AbortHandle,
}

impl Server {
  pub fn serve(world: World) -> Self {
    let task = spawn(async move {
      let router = router::create()
        .with_state(ServerState::new(world))
        .into_make_service_with_connect_info::<SocketAddr>();

      let listener = TcpListener::bind("0.0.0.0:8050")
        .await
        .unwrap();

      axum::serve(listener, router).await.unwrap();
    });

    Self {
      abort_handle: task.inner().abort_handle(),
    }
  }

  pub fn close(self) {
    self.abort_handle.abort();
  }
}

impl Drop for Server {
  fn drop(&mut self) {
    self.abort_handle.abort();
  }
}
