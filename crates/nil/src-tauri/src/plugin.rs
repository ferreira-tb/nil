use crate::manager::ManagerExt;
use anyhow::Result;
use tauri::Wry;
use tauri::plugin::TauriPlugin;

pub fn prevent_default() -> TauriPlugin<Wry> {
  use tauri_plugin_prevent_default::Flags;

  tauri_plugin_prevent_default::Builder::new()
    .with_flags(Flags::all().difference(Flags::DEV_TOOLS))
    .general_autofill(false)
    .password_autosave(false)
    .build()
}

pub fn single_instance() -> TauriPlugin<Wry> {
  tauri_plugin_single_instance::init(|app, _, _| {
    let window = app.main_window();
    let _: Result<()> = try {
      window.show()?;
      window.set_focus()?;
    };
  })
}

pub fn window_state() -> TauriPlugin<Wry> {
  use tauri_plugin_window_state::StateFlags as Flags;

  tauri_plugin_window_state::Builder::new()
    .with_state_flags(Flags::POSITION)
    .build()
}
