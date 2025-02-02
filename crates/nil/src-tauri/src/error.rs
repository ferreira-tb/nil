use nil_client::Error as NilClientError;
use nil_core::Error as NilCoreError;
use nil_server::Error as NilServerError;
use serde::Serialize;
use serde::ser::Serializer;
use std::error::Error as StdError;
use tauri::Error as TauriError;

pub use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;
pub type BoxResult<T> = StdResult<T, Box<dyn StdError>>;
pub type CResult<T> = StdResult<T, String>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("client is closed")]
  ClientClosed,
  #[error("server is closed")]
  ServerClosed,

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

macro_rules! impl_from_error {
  ($($err:ident),+) => {
    $(
      impl From<$err> for Error {
        fn from(value: $err) -> Self {
          Self::Other(value.into())
        }
      }
    )+
  };
}

impl_from_error!(NilClientError, NilCoreError, NilServerError, TauriError);
