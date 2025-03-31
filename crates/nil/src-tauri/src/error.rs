use nil_client::Error as NilClientError;
use nil_core::error::Error as NilCoreError;
use nil_server::Error as NilServerError;
use serde::Serialize;
use serde::ser::Serializer;
use std::error::Error as StdError;
use tauri::Error as TauriError;

pub use std::result::Result as StdResult;

pub type Result<T, E = Error> = StdResult<T, E>;
pub type BoxResult<T> = StdResult<T, Box<dyn StdError>>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Client not connected")]
  ClientDisconnected,
  #[error("Feature not supported on mobile")]
  MobileNotSupported,

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
