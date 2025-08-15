// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::MobileExt;
use crate::error::Result;
use crate::model::*;
use tauri::{AppHandle, command};

#[command]
pub(crate) async fn share_text(app: AppHandle, request: ShareTextRequest) -> Result<()> {
  app.mobile().share_text(request)
}
