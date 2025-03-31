use serde::Serialize;
use serde::ser::Serializer;

pub use std::result::Result as StdResult;

pub type Result<T, E = Error> = StdResult<T, E>;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Json(#[from] serde_json::Error),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}
