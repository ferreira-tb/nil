// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_payload::battle::SimulateBattleRequest;

pub async fn simulate(State(app): State<App>, Json(req): Json<SimulateBattleRequest>) -> Response {
  app
    .world(|world| world.simulate_battle(&req.attacker, &req.defender, req.luck, req.wall))
    .map_ok(|result| res!(OK, Json(result)))
    .unwrap_or_else(from_core_err)
    .await
}
