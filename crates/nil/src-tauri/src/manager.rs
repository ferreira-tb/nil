use tauri::{AppHandle, Manager, WebviewWindow, Window, Wry};

pub trait ManagerExt: Manager<Wry> {
  fn main_window(&self) -> WebviewWindow<Wry> {
    self.get_webview_window("main").unwrap()
  }
}

impl ManagerExt for AppHandle<Wry> {}
impl ManagerExt for WebviewWindow<Wry> {}
impl ManagerExt for Window<Wry> {}
