// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_core::village::Coord;

pub async fn get(State(app): State<App>, Json(coord): Json<Coord>) -> Response {
  app
    .village(coord, Clone::clone)
    .map_ok(|village| res!(OK, Json(village)))
    .unwrap_or_else(from_core_err)
    .await
}
