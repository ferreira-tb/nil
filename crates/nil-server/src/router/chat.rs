// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::FutureExt;
use itertools::Itertools;
use nil_core::chat::ChatMessagePlayer;

pub async fn get_all(State(app): State<App>) -> Response {
  app
    .chat(|chat| chat.iter().cloned().collect_vec())
    .map(|messages| res!(OK, Json(messages)))
    .await
}

pub async fn push(State(app): State<App>, Json(message): Json<ChatMessagePlayer>) -> Response {
  app
    .world_mut(|world| world.push_chat_message(message))
    .map(|id| res!(OK, Json(id)))
    .await
}
