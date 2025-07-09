// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod prefecture;
pub mod prelude;

use crate::error::CoreResult;
use crate::middleware::CurrentPlayer;
use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::infrastructure::building::BuildingId;
use nil_core::village::Coord;

pub async fn toggle(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
  Json((coord, id, enabled)): Json<(Coord, BuildingId, bool)>,
) -> Response {
  let result: CoreResult<()> = try {
    let mut world = app.world.write().await;
    world
      .round()
      .check_if_player_is_pending(&current_player.0)?;

    if world
      .village(coord)?
      .is_owned_by_player_and(|id| &current_player.0 == id)
    {
      world.toggle_building(coord, id, enabled)?;
    } else {
      return res!(FORBIDDEN);
    }
  };

  result
    .map(|()| res!(OK))
    .unwrap_or_else(from_core_err)
}
