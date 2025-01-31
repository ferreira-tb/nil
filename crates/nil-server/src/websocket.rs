#![allow(unused_imports, unused_mut, unused_variables)]

use axum::body::Bytes;
use axum::extract::ws::{Message, WebSocket};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use std::net::SocketAddr;
use std::ops::ControlFlow;

pub(crate) async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
  todo!()
}
