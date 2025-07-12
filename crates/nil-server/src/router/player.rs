// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use futures::{FutureExt, TryFutureExt};
use itertools::Itertools;
use nil_core::player::{Player, PlayerId, PlayerOptions, PlayerStatus};

pub async fn exists(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  app
    .world(|world| world.has_player(&id))
    .map(|yes| res!(OK, Json(yes)))
    .await
}

pub async fn get(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  app
    .player_manager(|pm| pm.player(&id).cloned())
    .map_ok(|player| res!(OK, Json(player)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_all(State(app): State<App>) -> Response {
  app
    .player_manager(|pm| pm.players().cloned().collect_vec())
    .map(|players| res!(OK, Json(players)))
    .await
}

pub async fn get_coords(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  app
    .continent(|k| {
      k.player_coords_by(|player| player == &id)
        .collect_vec()
    })
    .map(|coords| res!(OK, Json(coords)))
    .await
}

pub async fn get_maintenance(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
) -> Response {
  let id = &current_player.0;
  app
    .world(|world| world.get_player_maintenance(id))
    .map_ok(|maintenance| res!(OK, Json(maintenance)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_status(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  app
    .player_manager(|pm| pm.player(&id).map(Player::status))
    .map_ok(|status| res!(OK, Json(status)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_storage_capacity(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
) -> Response {
  let id = &current_player.0;
  app
    .world(|world| world.get_player_storage_capacity(id))
    .map_ok(|capacity| res!(OK, Json(capacity)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_status(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
  Json(status): Json<PlayerStatus>,
) -> Response {
  let id = &current_player.0;
  app
    .world_mut(|world| world.set_player_status(id, status))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn spawn(State(app): State<App>, Json(options): Json<PlayerOptions>) -> Response {
  app
    .world_mut(|world| world.spawn_player(Player::new(options)))
    .map_ok(|()| res!(CREATED))
    .unwrap_or_else(from_core_err)
    .await
}
