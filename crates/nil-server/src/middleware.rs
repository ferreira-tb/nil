// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::AnyResult;
use crate::res;
use axum::extract::Request;
use axum::http::HeaderValue;
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use derive_more::Deref;
use nil_core::player::PlayerId;

#[derive(Clone, Debug, Deref)]
pub struct CurrentPlayer(pub(crate) PlayerId);

impl TryFrom<&HeaderValue> for CurrentPlayer {
  type Error = anyhow::Error;

  fn try_from(value: &HeaderValue) -> AnyResult<Self> {
    let bytes = value.as_bytes().to_vec();
    let id = String::from_utf8(bytes)?;
    Ok(Self(PlayerId::from(id)))
  }
}

pub async fn authorization(mut request: Request, next: Next) -> Response {
  if let Some(header) = request.headers().get(AUTHORIZATION) {
    let Ok(player) = CurrentPlayer::try_from(header) else {
      return res!(BAD_REQUEST, "Player name contains invalid characters");
    };

    request.extensions_mut().insert(player);
    next.run(request).await
  } else {
    res!(UNAUTHORIZED, "Missing player name header")
  }
}
