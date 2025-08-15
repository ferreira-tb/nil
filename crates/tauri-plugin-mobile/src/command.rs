// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::MobileExt;
use crate::error::Result;
use crate::model::*;
use tauri::{AppHandle, command};

#[command]
pub(crate) async fn get_android_version(app: AppHandle) -> Result<Option<AndroidVersion>> {
  app.mobile().get_android_version()
}
