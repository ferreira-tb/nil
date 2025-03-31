use crate::error::{Error, Result};
use crate::player::PlayerId;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::num::NonZeroU32;
use strum::EnumIs;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Round {
  id: RoundId,
  phase: Phase,
}

impl Round {
  pub(crate) fn start<I>(&mut self, players: I) -> Result<()>
  where
    I: IntoIterator<Item = PlayerId>,
  {
    if !self.is_idle() {
      return Err(Error::RoundAlreadyStarted);
    }

    self.id.next();
    self.phase = Phase::Player {
      pending: players.into_iter().collect(),
    };

    Ok(())
  }

  pub const fn is_idle(&self) -> bool {
    self.phase.is_idle()
  }
}

#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct RoundId(NonZeroU32);

impl RoundId {
  const fn next(&mut self) {
    self.0 = self.0.saturating_add(1);
  }
}

impl Default for RoundId {
  fn default() -> Self {
    Self(unsafe { NonZeroU32::new_unchecked(1) })
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, EnumIs)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Phase {
  #[default]
  Idle,
  Player {
    pending: HashSet<PlayerId>,
  },
}
