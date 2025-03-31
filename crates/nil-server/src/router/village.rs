use crate::state::App;
use crate::{res, response};
use axum::extract::{Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_core::village::Coord;

pub async fn get(State(app): State<App>, Json(coord): Json<Coord>) -> Response {
  app
    .continent(|k| k.village(coord).cloned())
    .map_ok(|village| res!(OK, Json(village)))
    .unwrap_or_else(response::from_err)
    .await
}
