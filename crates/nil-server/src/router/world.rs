use crate::res;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::FutureExt;
use nil_core::world::World;

pub async fn get(State(app): State<App>) -> Response {
  app
    .world(World::state)
    .map(|world| res!(OK, Json(world)))
    .await
}
