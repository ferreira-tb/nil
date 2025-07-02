// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::to_vec;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub use bytes::Bytes;
pub use serde_json::from_slice;

pub fn to_bytes<T>(value: &T) -> Result<Bytes>
where
  T: ?Sized + Serialize,
{
  Ok(to_vec(value).map(Bytes::from)?)
}

pub fn read_file<T>(path: impl AsRef<Path>) -> Result<T>
where
  T: DeserializeOwned,
{
  Ok(from_slice(&fs::read(path)?)?)
}

pub fn write_file<T>(path: impl AsRef<Path>, value: &T) -> Result<()>
where
  T: ?Sized + Serialize,
{
  let mut file = File::create(path)?;
  file.write_all(&to_vec(value)?)?;
  file.flush()?;
  Ok(())
}
