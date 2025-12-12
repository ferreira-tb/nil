// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_payload::cheat::behavior::CheatGetBuildStepsRequest;

pub async fn get_build_steps(
  State(app): State<App>,
  Json(req): Json<CheatGetBuildStepsRequest>,
) -> Response {
  app
    .world_mut(|world| world.cheat_get_build_steps(req.coord))
    .map_ok(|steps| res!(OK, Json(steps)))
    .unwrap_or_else(from_core_err)
    .await
}
