#![feature(try_blocks)]

mod error;
mod response;
mod router;
mod state;
mod websocket;

pub use error::{AnyResult, Error, Result};
use nil_core::WorldOptions;
use nil_util::spawn;
use state::App;
use std::net::{SocketAddr, SocketAddrV4};
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::task::AbortHandle;

pub struct Server(AbortHandle);

impl Server {
  pub async fn serve(options: WorldOptions) -> Result<(Self, SocketAddrV4)> {
    let world = options.into_world();
    let (tx, rx) = oneshot::channel();

    let task = spawn(async move {
      let router = router::create()
        .with_state(App::new(world))
        .into_make_service_with_connect_info::<SocketAddr>();

      if let Some((listener, addr)) = bind().await {
        let _ = tx.send(Ok(addr));
        axum::serve(listener, router)
          .await
          .expect("failed to start nil server");
      } else {
        let _ = tx.send(Err(Error::FailedToStart));
      }
    });

    let addr = rx.await.unwrap()?;
    let server = Server(task.abort_handle());
    Ok((server, addr))
  }
}

async fn bind() -> Option<(TcpListener, SocketAddrV4)> {
  let result: AnyResult<(TcpListener, SocketAddrV4)> = try {
    let listener = TcpListener::bind("0.0.0.0:0").await?;
    let SocketAddr::V4(addr) = listener.local_addr()? else {
      unreachable!();
    };

    (listener, addr)
  };

  result.ok()
}

impl Drop for Server {
  fn drop(&mut self) {
    self.0.abort();
  }
}
