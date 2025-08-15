// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareTextRequest {
  pub title: String,
  pub text: String,
  pub mime_type: ShareTextMimeType,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub enum ShareTextMimeType {
  #[default]
  #[serde(rename = "text/plain")]
  Plain,
  #[serde(rename = "text/html")]
  Html,
  #[serde(rename = "text/json")]
  Json,
}
