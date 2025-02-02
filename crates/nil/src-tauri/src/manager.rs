use crate::error::Result;
use crate::state::Nil;
use nil_client::Client;
use tauri::{AppHandle, Manager, State, WebviewWindow, Window, Wry};

pub trait ManagerExt: Manager<Wry> {
  fn nil(&self) -> State<Nil> {
    self.state::<Nil>()
  }

  async fn with_client<F, T>(&self, f: F) -> Result<T>
  where
    F: AsyncFnOnce(&Client) -> T,
  {
    self.nil().with_client(f).await
  }
}

impl ManagerExt for AppHandle<Wry> {}
impl ManagerExt for WebviewWindow<Wry> {}
impl ManagerExt for Window<Wry> {}
