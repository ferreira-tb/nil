// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use futures::{FutureExt, TryFutureExt};
use nil_core::world::World;

pub async fn end_turn(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
) -> Response {
  app
    .world_mut(|world| world.end_turn(&player))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get(State(app): State<App>) -> Response {
  app
    .round(Clone::clone)
    .map(|round| res!(OK, Json(round)))
    .await
}

pub async fn start(State(app): State<App>) -> Response {
  app
    .world_mut(World::start_round)
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}
