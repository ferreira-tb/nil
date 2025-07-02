// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::FutureExt;
use nil_core::lobby::Lobby;
use nil_core::player::PlayerId;

pub async fn get(State(app): State<App>) -> Response {
  app
    .lobby(Lobby::state)
    .map(|lobby| res!(OK, Json(lobby)))
    .await
}

pub async fn ready(State(app): State<App>, Json((id, ready)): Json<(PlayerId, bool)>) -> Response {
  app
    .lobby_mut(|l| l.set_ready(id, ready))
    .map(|()| res!(OK))
    .await
}
