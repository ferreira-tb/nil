use serde::Serialize;
use serde::ser::Serializer;
use std::error::Error as StdError;

pub use std::result::Result as StdResult;

pub type BoxResult<T> = StdResult<T, Box<dyn StdError>>;
pub type CResult<T> = StdResult<T, String>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("server: {0}")]
  Server(#[from] nil_server::Error),

  #[error("{0}")]
  Other(#[from] anyhow::Error),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}

impl From<Error> for String {
  fn from(value: Error) -> Self {
    value.to_string()
  }
}

impl From<tauri::Error> for Error {
  fn from(value: tauri::Error) -> Self {
    Self::Other(value.into())
  }
}
