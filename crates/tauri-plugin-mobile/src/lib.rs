// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod command;
pub mod error;
pub mod model;

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Wry};

#[cfg(desktop)]
use desktop::Mobile;
#[cfg(mobile)]
use mobile::Mobile;

pub trait MobileExt {
  fn mobile(&self) -> &Mobile;
}

impl<T: Manager<Wry>> MobileExt for T {
  fn mobile(&self) -> &Mobile {
    self.state::<Mobile>().inner()
  }
}

pub fn init() -> TauriPlugin<Wry> {
  Builder::new("mobile")
    .setup(|app, api| {
      app.manage(Mobile::new(api)?);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![command::get_android_version])
    .build()
}
