// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(try_blocks, try_trait_v2)]

mod error;
mod middleware;
mod response;
mod router;
mod server;
mod state;
mod websocket;

pub use error::Error;
pub use server::{LocalServer, load_local, start_local};
