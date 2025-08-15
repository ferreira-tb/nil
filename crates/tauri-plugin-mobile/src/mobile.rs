// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::model::*;
use serde::de::DeserializeOwned;
use tauri::Wry;
use tauri::plugin::{PluginApi, PluginHandle};

pub struct Mobile(PluginHandle<Wry>);

impl Mobile {
  pub(crate) fn new<C>(api: PluginApi<Wry, C>) -> Result<Self>
  where
    C: DeserializeOwned,
  {
    api
      .register_android_plugin("com.plugin.mobile", "MobilePlugin")
      .map(Self)
      .map_err(Into::into)
  }

  pub fn share_text(&self, request: ShareTextRequest) -> Result<()> {
    self
      .0
      .run_mobile_plugin("shareText", request)
      .map_err(Into::into)
  }
}
