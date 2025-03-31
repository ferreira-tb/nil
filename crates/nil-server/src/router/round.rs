use crate::state::App;
use crate::{res, response};
use axum::extract::{Json, State};
use axum::response::Response;
use futures::{FutureExt, TryFutureExt};
use nil_core::world::World;

pub async fn get(State(app): State<App>) -> Response {
  app
    .round(Clone::clone)
    .map(|round| res!(OK, Json(round)))
    .await
}

pub async fn start(State(app): State<App>) -> Response {
  app
    .world_mut(World::start_round)
    .map_ok(|()| res!(NO_CONTENT))
    .unwrap_or_else(response::from_err)
    .await
}
