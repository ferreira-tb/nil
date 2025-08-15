// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::model::*;
use serde::de::DeserializeOwned;
use tauri::Wry;
use tauri::plugin::PluginApi;

pub struct Mobile;

impl Mobile {
  pub(crate) fn new<C>(_: PluginApi<Wry, C>) -> Result<Self>
  where
    C: DeserializeOwned,
  {
    Ok(Self)
  }

  pub fn get_android_version(&self) -> Result<Option<AndroidVersion>> {
    Ok(None)
  }
}
