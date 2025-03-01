use crate::res;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_core::village::Coord;

pub async fn get(State(app): State<App>, Json(coord): Json<Coord>) -> Response {
  app
    .continent(|k| k.village(coord).cloned())
    .map_ok(|village| res!(OK, Json(village)))
    .unwrap_or_else(|err| res!(NOT_FOUND, err.to_string()))
    .await
}
